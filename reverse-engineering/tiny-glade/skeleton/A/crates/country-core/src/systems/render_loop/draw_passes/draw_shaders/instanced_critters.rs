// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/instanced_critters.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: b283adff7fe5b1013da91804de25f0b5afbf57ab7755b53a6881780675ebbf9d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: instanced_critters.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 441
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
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_critters::draw_instanced_butterfly_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":441,"record_offset":20176,"rva":6214688,"va":5374923808}
// 编译实例 RVAs: 0x5ed420
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140540a60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-707205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923808,"raw_hex":"e87b35f5ff","size":5,"soft_fail":false,"target_va":5374216800,"va":5374924000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405410b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-705479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923808,"raw_hex":"e8393cf5ff","size":5,"soft_fail":false,"target_va":5374218416,"va":5374923890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1043457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923808,"raw_hex":"e801ec0f00","size":5,"soft_fail":false,"target_va":5375967552,"va":5374924090,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df9d00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8439892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923808,"raw_hex":"e854c88000","size":5,"soft_fail":false,"target_va":5383363840,"va":5374923943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28669440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923808,"raw_hex":"e80076b501","size":5,"soft_fail":false,"target_va":5403593360,"va":5374923915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_instanced_w_gl_draw_flag
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t782313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923808,"raw_hex":"e8e9ef0b00","size":5,"soft_fail":false,"target_va":5375706352,"va":5374924034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_instanced
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":441,"record_offset":20512,"parent_record_offset":20176,"ancestor_record_offset":20176,"inlinee":21977,"source_line":258,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":441,"record_offset":20464,"parent_record_offset":20176,"ancestor_record_offset":20176,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":441,"record_offset":20384,"parent_record_offset":20176,"ancestor_record_offset":20176,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":441,"record_offset":20412,"parent_record_offset":20384,"ancestor_record_offset":20176,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_critters::draw_instanced_firefly_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":441,"record_offset":20544,"rva":6215040,"va":5374924160}
// 编译实例 RVAs: 0x5ed580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405410b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-706155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374924160,"raw_hex":"e89539f5ff","size":5,"soft_fail":false,"target_va":5374218416,"va":5374924566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1042768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374924160,"raw_hex":"e850e90f00","size":5,"soft_fail":false,"target_va":5375967552,"va":5374924779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df6180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8423977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374924160,"raw_hex":"e8298a8000","size":5,"soft_fail":false,"target_va":5383348608,"va":5374924626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dfd4d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8453464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374924160,"raw_hex":"e858fd8000","size":5,"soft_fail":false,"target_va":5383378128,"va":5374924659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28668865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374924160,"raw_hex":"e8c173b501","size":5,"soft_fail":false,"target_va":5403593360,"va":5374924490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28054153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374924160,"raw_hex":"e88912ac01","size":5,"soft_fail":false,"target_va":5402978880,"va":5374924722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::discard
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t785330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374924160,"raw_hex":"e8b2fb0b00","size":5,"soft_fail":false,"target_va":5375709840,"va":5374924505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_instanced_w_gl_draw_flag
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t781667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374924160,"raw_hex":"e863ed0b00","size":5,"soft_fail":false,"target_va":5375706352,"va":5374924680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_instanced
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":441,"record_offset":22388,"parent_record_offset":20544,"ancestor_record_offset":20544,"inlinee":21977,"source_line":258,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":441,"record_offset":22312,"parent_record_offset":20544,"ancestor_record_offset":20544,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":441,"record_offset":20748,"parent_record_offset":20544,"ancestor_record_offset":20544,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":441,"record_offset":20804,"parent_record_offset":20776,"ancestor_record_offset":20544,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":441,"record_offset":22236,"parent_record_offset":22204,"ancestor_record_offset":20544,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":441,"record_offset":22180,"parent_record_offset":21792,"ancestor_record_offset":20544,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":441,"record_offset":22204,"parent_record_offset":22180,"ancestor_record_offset":20544,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":441,"record_offset":21820,"parent_record_offset":21792,"ancestor_record_offset":20544,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":441,"record_offset":21868,"parent_record_offset":21844,"ancestor_record_offset":20544,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":441,"record_offset":21844,"parent_record_offset":21820,"ancestor_record_offset":20544,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":441,"record_offset":21960,"parent_record_offset":21896,"ancestor_record_offset":20544,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":441,"record_offset":22048,"parent_record_offset":22024,"ancestor_record_offset":20544,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":441,"record_offset":20776,"parent_record_offset":20748,"ancestor_record_offset":20544,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":441,"record_offset":21792,"parent_record_offset":20776,"ancestor_record_offset":20544,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":441,"record_offset":21524,"parent_record_offset":21500,"ancestor_record_offset":20544,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":441,"record_offset":21572,"parent_record_offset":21548,"ancestor_record_offset":20544,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":441,"record_offset":21500,"parent_record_offset":21476,"ancestor_record_offset":20544,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":441,"record_offset":20992,"parent_record_offset":20968,"ancestor_record_offset":20544,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":441,"record_offset":21076,"parent_record_offset":21052,"ancestor_record_offset":20544,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":441,"record_offset":21016,"parent_record_offset":20992,"ancestor_record_offset":20544,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":441,"record_offset":21188,"parent_record_offset":21164,"ancestor_record_offset":20544,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":441,"record_offset":21984,"parent_record_offset":21960,"ancestor_record_offset":20544,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":441,"record_offset":21760,"parent_record_offset":20804,"ancestor_record_offset":20544,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":441,"record_offset":22024,"parent_record_offset":21844,"ancestor_record_offset":20544,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":441,"record_offset":22280,"parent_record_offset":20748,"ancestor_record_offset":20544,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":441,"record_offset":22128,"parent_record_offset":22104,"ancestor_record_offset":20544,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":441,"record_offset":21412,"parent_record_offset":21388,"ancestor_record_offset":20544,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":441,"record_offset":21332,"parent_record_offset":21308,"ancestor_record_offset":20544,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":441,"record_offset":21896,"parent_record_offset":21868,"ancestor_record_offset":20544,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":441,"record_offset":21932,"parent_record_offset":21896,"ancestor_record_offset":20544,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":441,"record_offset":22104,"parent_record_offset":22072,"ancestor_record_offset":20544,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":441,"record_offset":21548,"parent_record_offset":21524,"ancestor_record_offset":20544,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":441,"record_offset":22340,"parent_record_offset":22312,"ancestor_record_offset":20544,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":441,"record_offset":21476,"parent_record_offset":21452,"ancestor_record_offset":20544,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":441,"record_offset":20828,"parent_record_offset":20804,"ancestor_record_offset":20544,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":441,"record_offset":20852,"parent_record_offset":20828,"ancestor_record_offset":20544,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":441,"record_offset":21452,"parent_record_offset":21256,"ancestor_record_offset":20544,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":21364,"parent_record_offset":21280,"ancestor_record_offset":20544,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":21308,"parent_record_offset":21280,"ancestor_record_offset":20544,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":21708,"parent_record_offset":20928,"ancestor_record_offset":20544,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":21280,"parent_record_offset":21256,"ancestor_record_offset":20544,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":20904,"parent_record_offset":20880,"ancestor_record_offset":20544,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":20880,"parent_record_offset":20852,"ancestor_record_offset":20544,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":20928,"parent_record_offset":20904,"ancestor_record_offset":20544,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":441,"record_offset":21136,"parent_record_offset":21108,"ancestor_record_offset":20544,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":441,"record_offset":21164,"parent_record_offset":21136,"ancestor_record_offset":20544,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":441,"record_offset":21224,"parent_record_offset":21108,"ancestor_record_offset":20544,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":441,"record_offset":21108,"parent_record_offset":20928,"ancestor_record_offset":20544,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":21256,"parent_record_offset":20928,"ancestor_record_offset":20544,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":441,"record_offset":20968,"parent_record_offset":20928,"ancestor_record_offset":20544,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":441,"record_offset":21052,"parent_record_offset":20928,"ancestor_record_offset":20544,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":441,"record_offset":21624,"parent_record_offset":20928,"ancestor_record_offset":20544,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":441,"record_offset":22072,"parent_record_offset":22048,"ancestor_record_offset":20544,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_critters::draw_instanced_firefly_glow_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":441,"record_offset":22420,"rva":6215776,"va":5374924896}
// 编译实例 RVAs: 0x5ed860
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405410b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-706918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374924896,"raw_hex":"e89a36f5ff","size":5,"soft_fail":false,"target_va":5374218416,"va":5374925329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1041936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374924896,"raw_hex":"e810e60f00","size":5,"soft_fail":false,"target_va":5375967552,"va":5374925611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df6180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8423214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374924896,"raw_hex":"e82e878000","size":5,"soft_fail":false,"target_va":5383348608,"va":5374925389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df70f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8427067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374924896,"raw_hex":"e83b968000","size":5,"soft_fail":false,"target_va":5383352560,"va":5374925488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dfb190
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8443644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374924896,"raw_hex":"e8fcd68000","size":5,"soft_fail":false,"target_va":5383369104,"va":5374925455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dfd4d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8452701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374924896,"raw_hex":"e85dfa8000","size":5,"soft_fail":false,"target_va":5383378128,"va":5374925422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28668129","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374924896,"raw_hex":"e8e170b501","size":5,"soft_fail":false,"target_va":5403593360,"va":5374925226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28053324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374924896,"raw_hex":"e84c0fac01","size":5,"soft_fail":false,"target_va":5402978880,"va":5374925551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::discard
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t784594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374924896,"raw_hex":"e8d2f80b00","size":5,"soft_fail":false,"target_va":5375709840,"va":5374925241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_instanced_w_gl_draw_flag
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t780838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374924896,"raw_hex":"e826ea0b00","size":5,"soft_fail":false,"target_va":5375706352,"va":5374925509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_instanced
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":441,"record_offset":24360,"parent_record_offset":22420,"ancestor_record_offset":22420,"inlinee":21977,"source_line":258,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":441,"record_offset":24284,"parent_record_offset":22420,"ancestor_record_offset":22420,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":441,"record_offset":22636,"parent_record_offset":22420,"ancestor_record_offset":22420,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":441,"record_offset":24200,"parent_record_offset":22420,"ancestor_record_offset":22420,"inlinee":7336,"source_line":617,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":441,"record_offset":22692,"parent_record_offset":22664,"ancestor_record_offset":22420,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":441,"record_offset":24124,"parent_record_offset":24092,"ancestor_record_offset":22420,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":441,"record_offset":24068,"parent_record_offset":23680,"ancestor_record_offset":22420,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":441,"record_offset":24092,"parent_record_offset":24068,"ancestor_record_offset":22420,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":441,"record_offset":23708,"parent_record_offset":23680,"ancestor_record_offset":22420,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":441,"record_offset":23756,"parent_record_offset":23732,"ancestor_record_offset":22420,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":441,"record_offset":23732,"parent_record_offset":23708,"ancestor_record_offset":22420,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":441,"record_offset":23848,"parent_record_offset":23784,"ancestor_record_offset":22420,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":441,"record_offset":23936,"parent_record_offset":23912,"ancestor_record_offset":22420,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":441,"record_offset":22664,"parent_record_offset":22636,"ancestor_record_offset":22420,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":441,"record_offset":23680,"parent_record_offset":22664,"ancestor_record_offset":22420,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":441,"record_offset":23412,"parent_record_offset":23388,"ancestor_record_offset":22420,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":441,"record_offset":23460,"parent_record_offset":23436,"ancestor_record_offset":22420,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":441,"record_offset":23388,"parent_record_offset":23364,"ancestor_record_offset":22420,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":441,"record_offset":22880,"parent_record_offset":22856,"ancestor_record_offset":22420,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":441,"record_offset":22964,"parent_record_offset":22940,"ancestor_record_offset":22420,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":441,"record_offset":22904,"parent_record_offset":22880,"ancestor_record_offset":22420,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":441,"record_offset":23076,"parent_record_offset":23052,"ancestor_record_offset":22420,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":441,"record_offset":23872,"parent_record_offset":23848,"ancestor_record_offset":22420,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":441,"record_offset":23648,"parent_record_offset":22692,"ancestor_record_offset":22420,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":441,"record_offset":23912,"parent_record_offset":23732,"ancestor_record_offset":22420,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":441,"record_offset":24168,"parent_record_offset":22636,"ancestor_record_offset":22420,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":441,"record_offset":24016,"parent_record_offset":23992,"ancestor_record_offset":22420,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":441,"record_offset":23300,"parent_record_offset":23276,"ancestor_record_offset":22420,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":441,"record_offset":23220,"parent_record_offset":23196,"ancestor_record_offset":22420,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":441,"record_offset":23784,"parent_record_offset":23756,"ancestor_record_offset":22420,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":441,"record_offset":23820,"parent_record_offset":23784,"ancestor_record_offset":22420,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":441,"record_offset":23992,"parent_record_offset":23960,"ancestor_record_offset":22420,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":441,"record_offset":23436,"parent_record_offset":23412,"ancestor_record_offset":22420,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":441,"record_offset":24312,"parent_record_offset":24284,"ancestor_record_offset":22420,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":441,"record_offset":23364,"parent_record_offset":23340,"ancestor_record_offset":22420,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":441,"record_offset":22716,"parent_record_offset":22692,"ancestor_record_offset":22420,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":441,"record_offset":22740,"parent_record_offset":22716,"ancestor_record_offset":22420,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":441,"record_offset":23340,"parent_record_offset":23144,"ancestor_record_offset":22420,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":23252,"parent_record_offset":23168,"ancestor_record_offset":22420,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":23196,"parent_record_offset":23168,"ancestor_record_offset":22420,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":23596,"parent_record_offset":22816,"ancestor_record_offset":22420,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":23168,"parent_record_offset":23144,"ancestor_record_offset":22420,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":22792,"parent_record_offset":22768,"ancestor_record_offset":22420,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":22768,"parent_record_offset":22740,"ancestor_record_offset":22420,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":22816,"parent_record_offset":22792,"ancestor_record_offset":22420,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":441,"record_offset":23024,"parent_record_offset":22996,"ancestor_record_offset":22420,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":441,"record_offset":23052,"parent_record_offset":23024,"ancestor_record_offset":22420,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":441,"record_offset":23112,"parent_record_offset":22996,"ancestor_record_offset":22420,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":441,"record_offset":22996,"parent_record_offset":22816,"ancestor_record_offset":22420,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":23144,"parent_record_offset":22816,"ancestor_record_offset":22420,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":441,"record_offset":22856,"parent_record_offset":22816,"ancestor_record_offset":22420,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":441,"record_offset":22940,"parent_record_offset":22816,"ancestor_record_offset":22420,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":441,"record_offset":23512,"parent_record_offset":22816,"ancestor_record_offset":22420,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":441,"record_offset":24248,"parent_record_offset":24224,"ancestor_record_offset":22420,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":441,"record_offset":23960,"parent_record_offset":23936,"ancestor_record_offset":22420,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":441,"record_offset":24224,"parent_record_offset":24200,"ancestor_record_offset":22420,"inlinee":7337,"source_line":617,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_critters::draw_duck_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":441,"record_offset":24392,"rva":6216608,"va":5374925728}
// 编译实例 RVAs: 0x5edba0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405410b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-708026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374925728,"raw_hex":"e84632f5ff","size":5,"soft_fail":false,"target_va":5374218416,"va":5374926437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1040577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374925728,"raw_hex":"e8c1e00f00","size":5,"soft_fail":false,"target_va":5375967552,"va":5374926970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df6180
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8422089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374925728,"raw_hex":"e8c9828000","size":5,"soft_fail":false,"target_va":5383348608,"va":5374926514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df64b0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8422722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374925728,"raw_hex":"e842858000","size":5,"soft_fail":false,"target_va":5383349424,"va":5374926697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df6640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8423214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374925728,"raw_hex":"e82e878000","size":5,"soft_fail":false,"target_va":5383349824,"va":5374926605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28666975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374925728,"raw_hex":"e85f6cb501","size":5,"soft_fail":false,"target_va":5403593360,"va":5374926380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t28052008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374925728,"raw_hex":"e8280aac01","size":5,"soft_fail":false,"target_va":5402978880,"va":5374926867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28052493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374925728,"raw_hex":"e80d0cac01","size":5,"soft_fail":false,"target_va":5402979344,"va":5374926846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t778709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374925728,"raw_hex":"e8d5e10b00","size":5,"soft_fail":false,"target_va":5375705520,"va":5374926806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeEntityCell::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":441,"record_offset":24796,"parent_record_offset":24392,"ancestor_record_offset":24392,"inlinee":22528,"source_line":691,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":441,"record_offset":24588,"parent_record_offset":24392,"ancestor_record_offset":24392,"inlinee":22526,"source_line":299,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":441,"record_offset":24760,"parent_record_offset":24392,"ancestor_record_offset":24392,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":441,"record_offset":27936,"parent_record_offset":24392,"ancestor_record_offset":24392,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":441,"record_offset":25932,"parent_record_offset":25892,"ancestor_record_offset":24392,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":441,"record_offset":24836,"parent_record_offset":24796,"ancestor_record_offset":24392,"inlinee":22529,"source_line":713,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":441,"record_offset":24612,"parent_record_offset":24588,"ancestor_record_offset":24392,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":441,"record_offset":26396,"parent_record_offset":26372,"ancestor_record_offset":24392,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":441,"record_offset":26056,"parent_record_offset":26028,"ancestor_record_offset":24392,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":441,"record_offset":26028,"parent_record_offset":25892,"ancestor_record_offset":24392,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":441,"record_offset":26148,"parent_record_offset":26084,"ancestor_record_offset":24392,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":441,"record_offset":26236,"parent_record_offset":26212,"ancestor_record_offset":24392,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_unchecked
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":441,"record_offset":26372,"parent_record_offset":25820,"ancestor_record_offset":24392,"inlinee":22535,"source_line":383,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":441,"record_offset":27512,"parent_record_offset":27488,"ancestor_record_offset":24392,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$7::index
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":441,"record_offset":25892,"parent_record_offset":25860,"ancestor_record_offset":24392,"inlinee":22534,"source_line":904,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::fetch_table
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":441,"record_offset":25860,"parent_record_offset":25820,"ancestor_record_offset":24392,"inlinee":22532,"source_line":910,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::get_component
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":441,"record_offset":25820,"parent_record_offset":24796,"ancestor_record_offset":24392,"inlinee":22531,"source_line":942,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":441,"record_offset":25580,"parent_record_offset":25556,"ancestor_record_offset":24392,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":441,"record_offset":24720,"parent_record_offset":24696,"ancestor_record_offset":24392,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":441,"record_offset":25628,"parent_record_offset":25604,"ancestor_record_offset":24392,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":441,"record_offset":25556,"parent_record_offset":25532,"ancestor_record_offset":24392,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":441,"record_offset":25044,"parent_record_offset":25020,"ancestor_record_offset":24392,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":441,"record_offset":25128,"parent_record_offset":25104,"ancestor_record_offset":24392,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":441,"record_offset":25068,"parent_record_offset":25044,"ancestor_record_offset":24392,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":441,"record_offset":25240,"parent_record_offset":25216,"ancestor_record_offset":24392,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":441,"record_offset":24696,"parent_record_offset":24612,"ancestor_record_offset":24392,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":441,"record_offset":26172,"parent_record_offset":26148,"ancestor_record_offset":24392,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":441,"record_offset":27428,"parent_record_offset":26464,"ancestor_record_offset":24392,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":441,"record_offset":26212,"parent_record_offset":26028,"ancestor_record_offset":24392,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":441,"record_offset":26316,"parent_record_offset":26292,"ancestor_record_offset":24392,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":441,"record_offset":25468,"parent_record_offset":25444,"ancestor_record_offset":24392,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":441,"record_offset":25388,"parent_record_offset":25364,"ancestor_record_offset":24392,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":441,"record_offset":24640,"parent_record_offset":24612,"ancestor_record_offset":24392,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":441,"record_offset":25960,"parent_record_offset":25932,"ancestor_record_offset":24392,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":441,"record_offset":24664,"parent_record_offset":24640,"ancestor_record_offset":24392,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":441,"record_offset":26292,"parent_record_offset":26260,"ancestor_record_offset":24392,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":441,"record_offset":25988,"parent_record_offset":25960,"ancestor_record_offset":24392,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":441,"record_offset":25604,"parent_record_offset":25580,"ancestor_record_offset":24392,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":441,"record_offset":27964,"parent_record_offset":27936,"ancestor_record_offset":24392,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":441,"record_offset":25532,"parent_record_offset":25508,"ancestor_record_offset":24392,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":441,"record_offset":24864,"parent_record_offset":24836,"ancestor_record_offset":24392,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":441,"record_offset":24892,"parent_record_offset":24864,"ancestor_record_offset":24392,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":441,"record_offset":25508,"parent_record_offset":25308,"ancestor_record_offset":24392,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":25420,"parent_record_offset":25336,"ancestor_record_offset":24392,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":25364,"parent_record_offset":25336,"ancestor_record_offset":24392,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":25764,"parent_record_offset":24972,"ancestor_record_offset":24392,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":25336,"parent_record_offset":25308,"ancestor_record_offset":24392,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":24948,"parent_record_offset":24924,"ancestor_record_offset":24392,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":24924,"parent_record_offset":24892,"ancestor_record_offset":24392,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":24972,"parent_record_offset":24948,"ancestor_record_offset":24392,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":441,"record_offset":25188,"parent_record_offset":25160,"ancestor_record_offset":24392,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":441,"record_offset":25216,"parent_record_offset":25188,"ancestor_record_offset":24392,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":441,"record_offset":25276,"parent_record_offset":25160,"ancestor_record_offset":24392,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":441,"record_offset":25160,"parent_record_offset":24972,"ancestor_record_offset":24392,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":25308,"parent_record_offset":24972,"ancestor_record_offset":24392,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":441,"record_offset":25020,"parent_record_offset":24972,"ancestor_record_offset":24392,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":441,"record_offset":25104,"parent_record_offset":24972,"ancestor_record_offset":24392,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":441,"record_offset":25680,"parent_record_offset":24972,"ancestor_record_offset":24392,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":441,"record_offset":26260,"parent_record_offset":26236,"ancestor_record_offset":24392,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_critters::draw_wren_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":441,"record_offset":28072,"rva":6217968,"va":5374927088}
// 编译实例 RVAs: 0x5ee0f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140540a60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-710463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927088,"raw_hex":"e8c128f5ff","size":5,"soft_fail":false,"target_va":5374216800,"va":5374927258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405410b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-708762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927088,"raw_hex":"e8662ff5ff","size":5,"soft_fail":false,"target_va":5374218416,"va":5374927173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406aa9b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t772082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927088,"raw_hex":"e8f2c70b00","size":5,"soft_fail":false,"target_va":5375699376,"va":5374927289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1040209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927088,"raw_hex":"e851df0f00","size":5,"soft_fail":false,"target_va":5375967552,"va":5374927338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28666154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927088,"raw_hex":"e82a69b501","size":5,"soft_fail":false,"target_va":5403593360,"va":5374927201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":441,"record_offset":28336,"parent_record_offset":28072,"ancestor_record_offset":28072,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":441,"record_offset":28260,"parent_record_offset":28072,"ancestor_record_offset":28072,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":441,"record_offset":28288,"parent_record_offset":28260,"ancestor_record_offset":28072,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00441.jsonl.gz"}
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
// 完整结构及原始类型记录: instanced_critters.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: instanced_critters.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/instanced_critters.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: instanced_critters.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_critters::draw_instanced_butterfly_shader
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":441,"record_offset":20384,"parent_record_offset":20176,"ancestor_record_offset":20176,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00441.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":441,"record_offset":20412,"parent_record_offset":20384,"ancestor_record_offset":20176,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00441.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":441,"record_offset":20464,"parent_record_offset":20176,"ancestor_record_offset":20176,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00441.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_critters::draw_instanced_firefly_shader
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":441,"record_offset":22312,"parent_record_offset":20544,"ancestor_record_offset":20544,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00441.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":441,"record_offset":22340,"parent_record_offset":22312,"ancestor_record_offset":20544,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00441.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_critters::draw_instanced_firefly_glow_shader
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":441,"record_offset":24284,"parent_record_offset":22420,"ancestor_record_offset":22420,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00441.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":441,"record_offset":24312,"parent_record_offset":24284,"ancestor_record_offset":22420,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00441.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_critters::draw_duck_shader
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":441,"record_offset":27936,"parent_record_offset":24392,"ancestor_record_offset":24392,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00441.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":441,"record_offset":27964,"parent_record_offset":27936,"ancestor_record_offset":24392,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00441.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_critters::draw_wren_shader
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":441,"record_offset":28260,"parent_record_offset":28072,"ancestor_record_offset":28072,"inlinee":21972,"source_line":13,"archive":"_pdb/A/modules/00441.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":441,"record_offset":28288,"parent_record_offset":28260,"ancestor_record_offset":28072,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00441.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":441,"record_offset":28336,"parent_record_offset":28072,"ancestor_record_offset":28072,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00441.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// 父函数直接调用位置 = unknown/当前扫描未找到；PDB 内联及间接调用不能据此排除。
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: instanced_critters.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::event::EventReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::event::EventReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_critters::draw_duck_shader
// 信息 ID: 2ee05f743abdca2422c2badc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_critters::draw_instanced_butterfly_shader
// 信息 ID: a38710fae4c410c923f4fd49
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_critters::draw_instanced_firefly_glow_shader
// 信息 ID: bb0fe2f13428bae5c54a6b6f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_critters::draw_instanced_firefly_shader
// 信息 ID: e8e2a3f73df644b01b525d83
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_critters::draw_wren_shader
// 信息 ID: 0c7655053be7e2d8e5ea3b21
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
