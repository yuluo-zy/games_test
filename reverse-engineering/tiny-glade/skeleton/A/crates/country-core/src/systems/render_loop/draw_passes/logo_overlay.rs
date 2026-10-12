// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/draw_passes/logo_overlay.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 6828bd3fef95611c1e7abcf6bf4b6427900796f7344fc2b624a343a724838a09
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: logo_overlay.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 412, 1325, 1358
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_app; // namespace candidate
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
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::logo_overlay::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":412,"record_offset":22192,"rva":5740784,"va":5374449904}
// 编译实例 RVAs: 0x5798f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] bevy_app::sub_app::SubApp::add_systems
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t8599384","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5374449904,"raw_hex":"e958378300","size":5,"soft_fail":false,"target_va":5383049296,"va":5374449907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":412,"record_offset":22352,"parent_record_offset":22192,"ancestor_record_offset":22192,"inlinee":9705,"source_line":284,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::logo_overlay::startup
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5C50
// 首条位置: {"module":1325,"inlinee":23632}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":1325,"record_offset":1504516,"parent_record_offset":1504488,"ancestor_record_offset":1504488,"inlinee":22680,"source_line":43,"archive":"_pdb/A/modules/01325.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1325,"record_offset":1504540,"parent_record_offset":1504516,"ancestor_record_offset":1504488,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/01325.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::logo_overlay::render_logo_overlay
// 记录形式: compiled_procedure; instances=1
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":412,"record_offset":22384,"rva":5740800,"va":5374449920}
// 编译实例 RVAs: 0x579900
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: logo_overlay.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1825392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449920,"raw_hex":"e89025e4ff","size":5,"soft_fail":false,"target_va":5372625024,"va":5374450411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140540a60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: logo_overlay.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-233941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449920,"raw_hex":"e82b6efcff","size":5,"soft_fail":false,"target_va":5374216800,"va":5374450736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eb110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: logo_overlay.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1512245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449920,"raw_hex":"e835131700","size":5,"soft_fail":false,"target_va":5375963408,"va":5374451158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: logo_overlay.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1516497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449920,"raw_hex":"e8d1231700","size":5,"soft_fail":false,"target_va":5375967552,"va":5374451050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072a6e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: logo_overlay.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1772564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449920,"raw_hex":"e8140c1b00","size":5,"soft_fail":false,"target_va":5376222944,"va":5374450375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df6960
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: logo_overlay.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8899947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449920,"raw_hex":"e86bcd8700","size":5,"soft_fail":false,"target_va":5383350624,"va":5374450672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: logo_overlay.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29142587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449920,"raw_hex":"e83baebc01","size":5,"soft_fail":false,"target_va":5403593360,"va":5374450768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28527885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449920,"raw_hex":"e80d4db301","size":5,"soft_fail":false,"target_va":5402978880,"va":5374450990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1242164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449920,"raw_hex":"e834f41200","size":5,"soft_fail":false,"target_va":5375692416,"va":5374450247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_no_vao
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1253476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449920,"raw_hex":"e864201300","size":5,"soft_fail":false,"target_va":5375704400,"va":5374450919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1250068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449920,"raw_hex":"e814131300","size":5,"soft_fail":false,"target_va":5375700688,"va":5374450615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t945168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449920,"raw_hex":"e8106c0e00","size":5,"soft_fail":false,"target_va":5375395536,"va":5374450363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t947276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449920,"raw_hex":"e84c740e00","size":5,"soft_fail":false,"target_va":5375398240,"va":5374450959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1891484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449920,"raw_hex":"e89cdc1c00","size":5,"soft_fail":false,"target_va":5376341760,"va":5374450271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2915837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449920,"raw_hex":"e8fd7d2c00","size":5,"soft_fail":false,"target_va":5377366224,"va":5374450382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":412,"record_offset":22576,"parent_record_offset":22384,"ancestor_record_offset":22384,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":412,"record_offset":24080,"parent_record_offset":22384,"ancestor_record_offset":22384,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":412,"record_offset":24828,"parent_record_offset":22384,"ancestor_record_offset":22384,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":412,"record_offset":24296,"parent_record_offset":22384,"ancestor_record_offset":22384,"inlinee":21986,"source_line":105,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":412,"record_offset":24116,"parent_record_offset":22384,"ancestor_record_offset":22384,"inlinee":7327,"source_line":314,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":412,"record_offset":24960,"parent_record_offset":22384,"ancestor_record_offset":22384,"inlinee":7340,"source_line":626,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":412,"record_offset":24876,"parent_record_offset":22384,"ancestor_record_offset":22384,"inlinee":7336,"source_line":617,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":412,"record_offset":22604,"parent_record_offset":22576,"ancestor_record_offset":22384,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":412,"record_offset":24036,"parent_record_offset":24004,"ancestor_record_offset":22384,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":412,"record_offset":23980,"parent_record_offset":23592,"ancestor_record_offset":22384,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":412,"record_offset":24004,"parent_record_offset":23980,"ancestor_record_offset":22384,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":412,"record_offset":23620,"parent_record_offset":23592,"ancestor_record_offset":22384,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":412,"record_offset":23668,"parent_record_offset":23644,"ancestor_record_offset":22384,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":412,"record_offset":23644,"parent_record_offset":23620,"ancestor_record_offset":22384,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":412,"record_offset":23760,"parent_record_offset":23696,"ancestor_record_offset":22384,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":412,"record_offset":23848,"parent_record_offset":23824,"ancestor_record_offset":22384,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":412,"record_offset":23592,"parent_record_offset":22576,"ancestor_record_offset":22384,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":412,"record_offset":23324,"parent_record_offset":23300,"ancestor_record_offset":22384,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":412,"record_offset":24656,"parent_record_offset":24628,"ancestor_record_offset":22384,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":412,"record_offset":24708,"parent_record_offset":24684,"ancestor_record_offset":22384,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":412,"record_offset":24684,"parent_record_offset":24628,"ancestor_record_offset":22384,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":412,"record_offset":23372,"parent_record_offset":23348,"ancestor_record_offset":22384,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":412,"record_offset":23300,"parent_record_offset":23276,"ancestor_record_offset":22384,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":412,"record_offset":24148,"parent_record_offset":24116,"ancestor_record_offset":22384,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":412,"record_offset":22792,"parent_record_offset":22768,"ancestor_record_offset":22384,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":412,"record_offset":22876,"parent_record_offset":22852,"ancestor_record_offset":22384,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":412,"record_offset":22816,"parent_record_offset":22792,"ancestor_record_offset":22384,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":412,"record_offset":24732,"parent_record_offset":24708,"ancestor_record_offset":22384,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":412,"record_offset":22988,"parent_record_offset":22964,"ancestor_record_offset":22384,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":412,"record_offset":23784,"parent_record_offset":23760,"ancestor_record_offset":22384,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":412,"record_offset":23560,"parent_record_offset":22604,"ancestor_record_offset":22384,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":412,"record_offset":23824,"parent_record_offset":23644,"ancestor_record_offset":22384,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":412,"record_offset":23928,"parent_record_offset":23904,"ancestor_record_offset":22384,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":412,"record_offset":23212,"parent_record_offset":23188,"ancestor_record_offset":22384,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":412,"record_offset":23132,"parent_record_offset":23108,"ancestor_record_offset":22384,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":412,"record_offset":24756,"parent_record_offset":24732,"ancestor_record_offset":22384,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":412,"record_offset":23696,"parent_record_offset":23668,"ancestor_record_offset":22384,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":412,"record_offset":23732,"parent_record_offset":23696,"ancestor_record_offset":22384,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":412,"record_offset":23904,"parent_record_offset":23872,"ancestor_record_offset":22384,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":412,"record_offset":23348,"parent_record_offset":23324,"ancestor_record_offset":22384,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":412,"record_offset":24336,"parent_record_offset":24296,"ancestor_record_offset":22384,"inlinee":21988,"source_line":137,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::ColorAttachment::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":412,"record_offset":24224,"parent_record_offset":24196,"ancestor_record_offset":22384,"inlinee":23122,"source_line":89,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":412,"record_offset":24196,"parent_record_offset":24172,"ancestor_record_offset":22384,"inlinee":6964,"source_line":145,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$5::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":412,"record_offset":24172,"parent_record_offset":24148,"ancestor_record_offset":22384,"inlinee":7000,"source_line":179,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":412,"record_offset":23276,"parent_record_offset":23252,"ancestor_record_offset":22384,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":412,"record_offset":22628,"parent_record_offset":22604,"ancestor_record_offset":22384,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":412,"record_offset":22652,"parent_record_offset":22628,"ancestor_record_offset":22384,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":412,"record_offset":23252,"parent_record_offset":23056,"ancestor_record_offset":22384,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":412,"record_offset":23164,"parent_record_offset":23080,"ancestor_record_offset":22384,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":412,"record_offset":23108,"parent_record_offset":23080,"ancestor_record_offset":22384,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":412,"record_offset":23508,"parent_record_offset":22728,"ancestor_record_offset":22384,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":412,"record_offset":23080,"parent_record_offset":23056,"ancestor_record_offset":22384,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":412,"record_offset":22704,"parent_record_offset":22680,"ancestor_record_offset":22384,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":412,"record_offset":22680,"parent_record_offset":22652,"ancestor_record_offset":22384,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":412,"record_offset":22728,"parent_record_offset":22704,"ancestor_record_offset":22384,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":412,"record_offset":22936,"parent_record_offset":22908,"ancestor_record_offset":22384,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":412,"record_offset":22964,"parent_record_offset":22936,"ancestor_record_offset":22384,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":412,"record_offset":23024,"parent_record_offset":22908,"ancestor_record_offset":22384,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":412,"record_offset":22908,"parent_record_offset":22728,"ancestor_record_offset":22384,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":412,"record_offset":23056,"parent_record_offset":22728,"ancestor_record_offset":22384,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":412,"record_offset":22768,"parent_record_offset":22728,"ancestor_record_offset":22384,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":412,"record_offset":22852,"parent_record_offset":22728,"ancestor_record_offset":22384,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":412,"record_offset":23424,"parent_record_offset":22728,"ancestor_record_offset":22384,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":412,"record_offset":24924,"parent_record_offset":24900,"ancestor_record_offset":22384,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":412,"record_offset":23872,"parent_record_offset":23848,"ancestor_record_offset":22384,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":412,"record_offset":24984,"parent_record_offset":24960,"ancestor_record_offset":22384,"inlinee":7341,"source_line":626,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":412,"record_offset":24900,"parent_record_offset":24876,"ancestor_record_offset":22384,"inlinee":7337,"source_line":617,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":412,"record_offset":24248,"parent_record_offset":24224,"ancestor_record_offset":22384,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":412,"record_offset":24364,"parent_record_offset":24336,"ancestor_record_offset":22384,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":412,"record_offset":24420,"parent_record_offset":24392,"ancestor_record_offset":22384,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":412,"record_offset":24508,"parent_record_offset":24476,"ancestor_record_offset":22384,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":412,"record_offset":24628,"parent_record_offset":24476,"ancestor_record_offset":22384,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":412,"record_offset":24392,"parent_record_offset":24364,"ancestor_record_offset":22384,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":412,"record_offset":24532,"parent_record_offset":24508,"ancestor_record_offset":22384,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":412,"record_offset":24560,"parent_record_offset":24532,"ancestor_record_offset":22384,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":412,"record_offset":24476,"parent_record_offset":24448,"ancestor_record_offset":22384,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":412,"record_offset":24448,"parent_record_offset":24420,"ancestor_record_offset":22384,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00412.jsonl.gz"}
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
// 完整结构及原始类型记录: logo_overlay.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::draw_passes::logo_overlay::LogoOverlayPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$16::add_to_app<bevy_app::plugin::sealed::PluginMarker,feedback_reporter::FeedbackReporterPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::feedback_report_resources::FeedbackReportBackgroundPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::menu_background::MenuBackgroundPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::paths::PathPlugin,bevy_app::plugin::sealed::PluginMarker,system_water::WaterPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::garden::GardenPlugin,bevy_app::plugin::sealed::PluginMarker,system_tree::TreePlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::prefab::PrefabPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::vermeer::VermeerPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::draw_shaders::clouds::CloudPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::logo_overlay::LogoOverlayPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::photo_mode::PhotoModePlugin>", "constructor": "bevy_app::plugin::sealed::impl$16::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "feedback_reporter::FeedbackReporterPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::feedback_report_resources::FeedbackReportBackgroundPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::menu_background::MenuBackgroundPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::paths::PathPlugin", "bevy_app::plugin::sealed::PluginMarker", "system_water::WaterPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::garden::GardenPlugin", "bevy_app::plugin::sealed::PluginMarker", "system_tree::TreePlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::prefab::PrefabPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::vermeer::VermeerPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::draw_passes::draw_shaders::clouds::CloudPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::draw_passes::logo_overlay::LogoOverlayPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::photo_mode::PhotoModePlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::systems::render_loop::draw_passes::logo_overlay::LogoOverlayPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::systems::render_loop::draw_passes::logo_overlay::LogoOverlayPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<country_core::systems::render_loop::draw_passes::logo_overlay::LogoOverlayPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["country_core::systems::render_loop::draw_passes::logo_overlay::LogoOverlayPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<country_core::systems::render_loop::draw_passes::logo_overlay::LogoOverlayPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["country_core::systems::render_loop::draw_passes::logo_overlay::LogoOverlayPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<country_core::systems::render_loop::draw_passes::logo_overlay::LogoOverlayPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["country_core::systems::render_loop::draw_passes::logo_overlay::LogoOverlayPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::render_loop::draw_passes::logo_overlay::LogoOverlayPlugin>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::render_loop::draw_passes::logo_overlay::LogoOverlayPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<country_core::systems::render_loop::draw_passes::logo_overlay::LogoOverlayPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["country_core::systems::render_loop::draw_passes::logo_overlay::LogoOverlayPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: logo_overlay.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/logo_overlay.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: logo_overlay.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::logo_overlay::render_logo_overlay
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":412,"record_offset":24828,"parent_record_offset":22384,"ancestor_record_offset":22384,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00412.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [Shader 映射] {"shader_id":"90ae6796b3c85b35a3ba49a3","relative_path":"_logo_overlay.raster.hlsl_b903e43ffb3da915.bin","sha256":"56a582f5a341ae38976b39b732ae4a087c756459acf41adda460c83a7feb9d09","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"logo_overlay","prefix_candidates":["logo_overlay"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"ef09ce1b9df983fcfde9b64e","relative_path":"_logo_overlay.raster.hlsl_f0adff76827e73ff.bin","sha256":"cf726f7ebc3517b1ec43ee50f48925a85a592b7770bfaf03184d7b6d6921da3b","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"logo_overlay","prefix_candidates":["logo_overlay"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _logo_overlay.raster.hlsl_b903e43ffb3da915.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Vertex","function_id":1,"entry_name":"vs_main","interface_ids":[2,3,4]}],"execution_modes":[]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"$Globals","storage_class":"Uniform","type":{"id":36,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,6]},"decorations":[],"pointee":{"id":6,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[35,35]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"framebuffer_dims","type_id":35,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"input_tex_dims","type_id":35,"decorations":[{"decoration":"Offset","operands":[8]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"input_tex","storage_class":"UniformConstant","type":{"id":37,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,5]},"decorations":[],"pointee":{"id":5,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[11,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2,"name":"unknown","storage_class":"Input","type":{"id":38,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,13]},"decorations":[],"pointee":{"id":13,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":42,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[42]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Output","type":{"id":39,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,29]},"decorations":[],"pointee":{"id":29,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[11,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":0,"declared_direction":"stage_output","decorations":[{"decoration":"BuiltIn","operands":[0]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"out.var.C0","storage_class":"Output","type":{"id":40,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,12]},"decorations":[],"pointee":{"id":12,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[11,2]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Fragment","function_id":1,"entry_name":"ps_main","interface_ids":[2,3]}],"execution_modes":[{"entry_id":1,"mode":"OriginUpperLeft","operands":[],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"sampler_llc","storage_class":"UniformConstant","type":{"id":17,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,4]},"decorations":[],"pointee":{"id":4,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"input_tex","storage_class":"UniformConstant","type":{"id":19,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,6]},"decorations":[],"pointee":{"id":6,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[9,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2,"name":"in.var.C0","storage_class":"Input","type":{"id":20,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,10]},"decorations":[],"pointee":{"id":10,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[9,2]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"out.var.SV_TARGET","storage_class":"Output","type":{"id":21,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,18]},"decorations":[],"pointee":{"id":18,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[9,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _logo_overlay.raster.hlsl_f0adff76827e73ff.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Vertex","function_id":1,"entry_name":"vs_main","interface_ids":[2,3,4]}],"execution_modes":[]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"$Globals","storage_class":"Uniform","type":{"id":36,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,6]},"decorations":[],"pointee":{"id":6,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[35,35]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"framebuffer_dims","type_id":35,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"input_tex_dims","type_id":35,"decorations":[{"decoration":"Offset","operands":[8]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"input_tex","storage_class":"UniformConstant","type":{"id":37,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,5]},"decorations":[],"pointee":{"id":5,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[11,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2,"name":"unknown","storage_class":"Input","type":{"id":38,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,13]},"decorations":[],"pointee":{"id":13,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":42,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[42]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Output","type":{"id":39,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,29]},"decorations":[],"pointee":{"id":29,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[11,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":0,"declared_direction":"stage_output","decorations":[{"decoration":"BuiltIn","operands":[0]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"out.var.C0","storage_class":"Output","type":{"id":40,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,12]},"decorations":[],"pointee":{"id":12,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[11,2]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Fragment","function_id":1,"entry_name":"ps_main","interface_ids":[]}],"execution_modes":[{"entry_id":1,"mode":"OriginUpperLeft","operands":[],"operand_kind":"literal"}]}
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3144143,"position_among_mapped_render_targets":59,"mapped_render_target_count":70,"previous_callsite":3144035,"next_callsite":3144467,"previous_target_files":["crates/country-core/src/systems/render_loop/vermeer.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/session_selection/mod.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: logo_overlay.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 206; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<ref_mut$<country_core::resources::prefab::PrefabInstance>,tuple$<> >,bevy_ecs::event::EventReader<country_core::systems::prefab::DespawnPrefabCmd>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<ref_mut$<country_core::resources::prefab::PrefabInstance>,tuple$<> >", "bevy_ecs::event::EventReader<country_core::systems::prefab::DespawnPrefabCmd>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref_mut$<country_core::resources::prefab::PrefabInstance>,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::prefab::DespawnPrefabCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>,bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::event::EventWriter<country_core::resources::window::CursorMoved>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>", "bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::event::EventWriter<country_core::resources::window::CursorMoved>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::window::CursorMoved>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::event::EventReader<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::building_audio::BuildingAudioState>,bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::event::EventReader<country_core::systems::audio::building_audio::PlayBuildingAudio>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::building_audio::BuildingAudioState>", "bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::audio::building_audio::PlayBuildingAudio>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::building_audio::BuildingAudioState>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::logo_overlay::impl$0::build
// 信息 ID: 6112ee619418a8f5d89845ab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::logo_overlay::render_logo_overlay
// 信息 ID: b3e39abe84853a34ab3bc543
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2ff9cf -> 0x579900 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::logo_overlay::startup
// 信息 ID: c836ed92b7949a250520f84a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] f6e723bf3a293de264db1c5c: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)> >; module=1325, inline_record=1504488; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)>; module=1358, inline_record=21216; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
