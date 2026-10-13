// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/water_planes_draw.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 22b6a620e8bf67f084be857bd9d41cba6a5311c9dd19a821d1b48222fb4c088c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: water_planes_draw.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 462
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
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::water_planes_draw
// 记录形式: compiled_procedure; instances=1
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":462,"record_offset":32112,"rva":6536544,"va":5375245664}
// 编译实例 RVAs: 0x63bd60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140540a60
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1029558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375245664,"raw_hex":"e84a4af0ff","size":5,"soft_fail":false,"target_va":5374216800,"va":5375246353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405410b0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1027681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375245664,"raw_hex":"e89f51f0ff","size":5,"soft_fail":false,"target_va":5374218416,"va":5375246092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=9
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t720645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375245664,"raw_hex":"e805ff0a00","size":5,"soft_fail":false,"target_va":5375967552,"va":5375246902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df52e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8098206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375245664,"raw_hex":"e89e917b00","size":5,"soft_fail":false,"target_va":5383344864,"va":5375246653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df70f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8106142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375245664,"raw_hex":"e89eb07b00","size":5,"soft_fail":false,"target_va":5383352560,"va":5375246413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df9720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8116104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375245664,"raw_hex":"e888d77b00","size":5,"soft_fail":false,"target_va":5383362336,"va":5375246227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=12
//     target_aliases=1 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28347619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375245664,"raw_hex":"e8e38cb001","size":5,"soft_fail":false,"target_va":5403593360,"va":5375245736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27732027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375245664,"raw_hex":"e83b28a701","size":5,"soft_fail":false,"target_va":5402978880,"va":5375246848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::discard
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t464084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375245664,"raw_hex":"e8d4140700","size":5,"soft_fail":false,"target_va":5375709840,"va":5375245751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t458703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375245664,"raw_hex":"e8cfff0600","size":5,"soft_fail":false,"target_va":5375705520,"va":5375246812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":462,"record_offset":34104,"parent_record_offset":32112,"ancestor_record_offset":32112,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_winter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":462,"record_offset":33880,"parent_record_offset":32112,"ancestor_record_offset":32112,"inlinee":12658,"source_line":205,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":462,"record_offset":34028,"parent_record_offset":32112,"ancestor_record_offset":32112,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$1::bind_terrain_lighting
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":462,"record_offset":34312,"parent_record_offset":32112,"ancestor_record_offset":32112,"inlinee":23612,"source_line":26,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":462,"record_offset":32316,"parent_record_offset":32112,"ancestor_record_offset":32112,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":462,"record_offset":33944,"parent_record_offset":32112,"ancestor_record_offset":32112,"inlinee":7340,"source_line":626,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":462,"record_offset":32372,"parent_record_offset":32344,"ancestor_record_offset":32112,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":462,"record_offset":33804,"parent_record_offset":33772,"ancestor_record_offset":32112,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":462,"record_offset":33748,"parent_record_offset":33360,"ancestor_record_offset":32112,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":462,"record_offset":33772,"parent_record_offset":33748,"ancestor_record_offset":32112,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":462,"record_offset":33388,"parent_record_offset":33360,"ancestor_record_offset":32112,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":462,"record_offset":33436,"parent_record_offset":33412,"ancestor_record_offset":32112,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":462,"record_offset":33412,"parent_record_offset":33388,"ancestor_record_offset":32112,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":462,"record_offset":33528,"parent_record_offset":33464,"ancestor_record_offset":32112,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":462,"record_offset":33616,"parent_record_offset":33592,"ancestor_record_offset":32112,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":462,"record_offset":32344,"parent_record_offset":32316,"ancestor_record_offset":32112,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":462,"record_offset":33360,"parent_record_offset":32344,"ancestor_record_offset":32112,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":462,"record_offset":33092,"parent_record_offset":33068,"ancestor_record_offset":32112,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":462,"record_offset":33140,"parent_record_offset":33116,"ancestor_record_offset":32112,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":462,"record_offset":33068,"parent_record_offset":33044,"ancestor_record_offset":32112,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":462,"record_offset":32560,"parent_record_offset":32536,"ancestor_record_offset":32112,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":462,"record_offset":32644,"parent_record_offset":32620,"ancestor_record_offset":32112,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":462,"record_offset":32584,"parent_record_offset":32560,"ancestor_record_offset":32112,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":462,"record_offset":32756,"parent_record_offset":32732,"ancestor_record_offset":32112,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":462,"record_offset":33552,"parent_record_offset":33528,"ancestor_record_offset":32112,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":462,"record_offset":33328,"parent_record_offset":32372,"ancestor_record_offset":32112,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":462,"record_offset":33592,"parent_record_offset":33412,"ancestor_record_offset":32112,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":462,"record_offset":33848,"parent_record_offset":32316,"ancestor_record_offset":32112,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":462,"record_offset":33696,"parent_record_offset":33672,"ancestor_record_offset":32112,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":462,"record_offset":32980,"parent_record_offset":32956,"ancestor_record_offset":32112,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":462,"record_offset":32900,"parent_record_offset":32876,"ancestor_record_offset":32112,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":462,"record_offset":33464,"parent_record_offset":33436,"ancestor_record_offset":32112,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":462,"record_offset":33500,"parent_record_offset":33464,"ancestor_record_offset":32112,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":462,"record_offset":33672,"parent_record_offset":33640,"ancestor_record_offset":32112,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":462,"record_offset":33116,"parent_record_offset":33092,"ancestor_record_offset":32112,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":462,"record_offset":34056,"parent_record_offset":34028,"ancestor_record_offset":32112,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::is_winter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":462,"record_offset":33908,"parent_record_offset":33880,"ancestor_record_offset":32112,"inlinee":21928,"source_line":109,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":462,"record_offset":33044,"parent_record_offset":33020,"ancestor_record_offset":32112,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":462,"record_offset":32396,"parent_record_offset":32372,"ancestor_record_offset":32112,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":462,"record_offset":32420,"parent_record_offset":32396,"ancestor_record_offset":32112,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":462,"record_offset":33020,"parent_record_offset":32824,"ancestor_record_offset":32112,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":462,"record_offset":32932,"parent_record_offset":32848,"ancestor_record_offset":32112,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":462,"record_offset":32876,"parent_record_offset":32848,"ancestor_record_offset":32112,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":462,"record_offset":33276,"parent_record_offset":32496,"ancestor_record_offset":32112,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":462,"record_offset":32848,"parent_record_offset":32824,"ancestor_record_offset":32112,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":462,"record_offset":32472,"parent_record_offset":32448,"ancestor_record_offset":32112,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":462,"record_offset":32448,"parent_record_offset":32420,"ancestor_record_offset":32112,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":462,"record_offset":32496,"parent_record_offset":32472,"ancestor_record_offset":32112,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":462,"record_offset":32704,"parent_record_offset":32676,"ancestor_record_offset":32112,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":462,"record_offset":32732,"parent_record_offset":32704,"ancestor_record_offset":32112,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":462,"record_offset":32792,"parent_record_offset":32676,"ancestor_record_offset":32112,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":462,"record_offset":32676,"parent_record_offset":32496,"ancestor_record_offset":32112,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":462,"record_offset":32824,"parent_record_offset":32496,"ancestor_record_offset":32112,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":462,"record_offset":32536,"parent_record_offset":32496,"ancestor_record_offset":32112,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":462,"record_offset":32620,"parent_record_offset":32496,"ancestor_record_offset":32112,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":462,"record_offset":33192,"parent_record_offset":32496,"ancestor_record_offset":32112,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":462,"record_offset":33992,"parent_record_offset":33968,"ancestor_record_offset":32112,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":462,"record_offset":33640,"parent_record_offset":33616,"ancestor_record_offset":32112,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":462,"record_offset":33968,"parent_record_offset":33944,"ancestor_record_offset":32112,"inlinee":7341,"source_line":626,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::lilypad_draw
// 记录形式: compiled_procedure; instances=1
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":462,"record_offset":34484,"rva":6538320,"va":5375247440}
// 编译实例 RVAs: 0x63c450
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405410b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1029132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375247440,"raw_hex":"e8f44bf0ff","size":5,"soft_fail":false,"target_va":5374218416,"va":5375247543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t719889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375247440,"raw_hex":"e811fc0a00","size":5,"soft_fail":false,"target_va":5375967552,"va":5375247658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df79c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8107319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375247440,"raw_hex":"e837b57b00","size":5,"soft_fail":false,"target_va":5383354816,"va":5375247492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dfe670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8135049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375247440,"raw_hex":"e889217c00","size":5,"soft_fail":false,"target_va":5383382640,"va":5375247586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28345787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375247440,"raw_hex":"e8bb85b001","size":5,"soft_fail":false,"target_va":5403593360,"va":5375247568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::multi_draw_indexed_indirect
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t459971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375247440,"raw_hex":"e8c3040700","size":5,"soft_fail":false,"target_va":5375707584,"va":5375247608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_indexed_indirect
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":462,"record_offset":34744,"parent_record_offset":34484,"ancestor_record_offset":34484,"inlinee":22561,"source_line":345,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":462,"record_offset":34668,"parent_record_offset":34484,"ancestor_record_offset":34484,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":462,"record_offset":34696,"parent_record_offset":34668,"ancestor_record_offset":34484,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::lilypad_flower_draw
// 记录形式: compiled_procedure; instances=1
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":462,"record_offset":34776,"rva":6538560,"va":5375247680}
// 编译实例 RVAs: 0x63c540
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140540a60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1031088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375247680,"raw_hex":"e85044f0ff","size":5,"soft_fail":false,"target_va":5374216800,"va":5375247883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405410b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1029372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375247680,"raw_hex":"e8044bf0ff","size":5,"soft_fail":false,"target_va":5374218416,"va":5375247783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t719569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375247680,"raw_hex":"e8d1fa0a00","size":5,"soft_fail":false,"target_va":5375967552,"va":5375247978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df76d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8106327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375247680,"raw_hex":"e857b17b00","size":5,"soft_fail":false,"target_va":5383354064,"va":5375247732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dfe670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8134809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375247680,"raw_hex":"e899207c00","size":5,"soft_fail":false,"target_va":5383382640,"va":5375247826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28345547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375247680,"raw_hex":"e8cb84b001","size":5,"soft_fail":false,"target_va":5403593360,"va":5375247808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::multi_draw_indexed_indirect
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t459653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375247680,"raw_hex":"e885030700","size":5,"soft_fail":false,"target_va":5375707584,"va":5375247926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_indexed_indirect
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":462,"record_offset":35100,"parent_record_offset":34776,"ancestor_record_offset":34776,"inlinee":22561,"source_line":345,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":462,"record_offset":35052,"parent_record_offset":34776,"ancestor_record_offset":34776,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":462,"record_offset":34972,"parent_record_offset":34776,"ancestor_record_offset":34776,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":462,"record_offset":35000,"parent_record_offset":34972,"ancestor_record_offset":34776,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::reed_draw
// 记录形式: compiled_procedure; instances=1
// 源码行: 94
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":462,"record_offset":35132,"rva":6538928,"va":5375248048}
// 编译实例 RVAs: 0x63c6b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405410b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1029740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375248048,"raw_hex":"e89449f0ff","size":5,"soft_fail":false,"target_va":5374218416,"va":5375248151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t719281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375248048,"raw_hex":"e8b1f90a00","size":5,"soft_fail":false,"target_va":5375967552,"va":5375248266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df6e00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8103703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375248048,"raw_hex":"e817a77b00","size":5,"soft_fail":false,"target_va":5383351808,"va":5375248100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dfe670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8134441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375248048,"raw_hex":"e8291f7c00","size":5,"soft_fail":false,"target_va":5383382640,"va":5375248194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: water_planes_draw.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28345179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375248048,"raw_hex":"e85b83b001","size":5,"soft_fail":false,"target_va":5403593360,"va":5375248176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::multi_draw_indexed_indirect
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t459363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375248048,"raw_hex":"e863020700","size":5,"soft_fail":false,"target_va":5375707584,"va":5375248216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_indexed_indirect
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":462,"record_offset":35388,"parent_record_offset":35132,"ancestor_record_offset":35132,"inlinee":22561,"source_line":345,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":462,"record_offset":35312,"parent_record_offset":35132,"ancestor_record_offset":35132,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":462,"record_offset":35340,"parent_record_offset":35312,"ancestor_record_offset":35132,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00462.jsonl.gz"}
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
// 完整结构及原始类型记录: water_planes_draw.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: water_planes_draw.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/water_planes_draw.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: water_planes_draw.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::water_planes_draw
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":462,"record_offset":34028,"parent_record_offset":32112,"ancestor_record_offset":32112,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00462.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":462,"record_offset":34056,"parent_record_offset":34028,"ancestor_record_offset":32112,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00462.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":462,"record_offset":34104,"parent_record_offset":32112,"ancestor_record_offset":32112,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00462.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_lighting_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$1::bind_terrain_lighting","relationship":"pdb_inline_direct","evidence":{"module":462,"record_offset":34312,"parent_record_offset":32112,"ancestor_record_offset":32112,"inlinee":23612,"source_line":26,"archive":"_pdb/A/modules/00462.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::lilypad_draw
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":462,"record_offset":34668,"parent_record_offset":34484,"ancestor_record_offset":34484,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00462.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":462,"record_offset":34696,"parent_record_offset":34668,"ancestor_record_offset":34484,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00462.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::lilypad_flower_draw
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":462,"record_offset":34972,"parent_record_offset":34776,"ancestor_record_offset":34776,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00462.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":462,"record_offset":35000,"parent_record_offset":34972,"ancestor_record_offset":34776,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00462.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":462,"record_offset":35052,"parent_record_offset":34776,"ancestor_record_offset":34776,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00462.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::reed_draw
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":462,"record_offset":35312,"parent_record_offset":35132,"ancestor_record_offset":35132,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00462.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":462,"record_offset":35340,"parent_record_offset":35312,"ancestor_record_offset":35132,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00462.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// 父函数直接调用位置 = unknown/当前扫描未找到；PDB 内联及间接调用不能据此排除。
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: water_planes_draw.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafRng>,bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>", "bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafRng>", "bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafRng>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::lilypad_draw
// 信息 ID: be02f3c2f21f7861f97dcaa3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::lilypad_flower_draw
// 信息 ID: b4dfc9bc278d1289f6890a4a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::reed_draw
// 信息 ID: 4ce9c0bdeacc680cb4778e1d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::water_planes_draw::water_planes_draw
// 信息 ID: 926d0607ba179344844a786b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
