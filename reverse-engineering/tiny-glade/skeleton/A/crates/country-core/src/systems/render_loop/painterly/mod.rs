// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/painterly/mod.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 8fadf2cc56323f95d8a5b3a32f96a7d83424c669521a9447f389c194cff3c147
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 413, 441, 1325
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
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"PDB_registry_path_only","version":"0.14.5"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
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
// 函数符号: country_core::systems::render_loop::painterly::PainterlyState::has_target
// 记录形式: compiled_procedure; instances=1
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":441,"record_offset":28388,"rva":6218288,"va":5374927408}
// 编译实例 RVAs: 0x5ee230
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::painterly::impl$1::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":441,"record_offset":28540,"rva":6218304,"va":5374927424}
// 编译实例 RVAs: 0x5ee240
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8156819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927424,"raw_hex":"e893767c00","size":5,"soft_fail":false,"target_va":5383084272,"va":5374927448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] bevy_app::sub_app::SubApp::add_systems
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t8120678","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5374927424,"raw_hex":"e966e97b00","size":5,"soft_fail":false,"target_va":5383048144,"va":5374927461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":441,"record_offset":28684,"parent_record_offset":28540,"ancestor_record_offset":28540,"inlinee":9705,"source_line":284,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::painterly::painterly_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":441,"record_offset":28716,"rva":6218352,"va":5374927472}
// 编译实例 RVAs: 0x5ee270
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=2
//     target_aliases=16 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2307420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e8a4cadcff","size":5,"soft_fail":false,"target_va":5372625024,"va":5374932439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2181745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e88fb5deff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374931436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2181782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e86ab5deff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374931489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140540a60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-716009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e81713f5ff","size":5,"soft_fail":false,"target_va":5374216800,"va":5374932804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140589d00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-412132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e81cb6f9ff","size":5,"soft_fail":false,"target_va":5374516480,"va":5374928607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405aafa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-277981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e823c2fbff","size":5,"soft_fail":false,"target_va":5374652320,"va":5374930296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405abac0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-273180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e8e4d4fbff","size":5,"soft_fail":false,"target_va":5374655168,"va":5374928343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140642d80
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t345491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e893450500","size":5,"soft_fail":false,"target_va":5375274368,"va":5374928872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140643320
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t346670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e82e4a0500","size":5,"soft_fail":false,"target_va":5375275808,"va":5374929133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406441b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t345817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e8d9460500","size":5,"soft_fail":false,"target_va":5375279536,"va":5374933714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064cbb0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t385264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e8f0e00500","size":5,"soft_fail":false,"target_va":5375314864,"va":5374929595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eb110
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1028234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e88ab00f00","size":5,"soft_fail":false,"target_va":5375963408,"va":5374935169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1032612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e8a4c10f00","size":5,"soft_fail":false,"target_va":5375967552,"va":5374934935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eec80
//     owner=unresolved_namespace; count=10
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1044045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e84dee0f00","size":5,"soft_fail":false,"target_va":5375978624,"va":5374934574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072a6e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1290537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e829b11300","size":5,"soft_fail":false,"target_va":5376222944,"va":5374932402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1310299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e85bfe1300","size":5,"soft_fail":false,"target_va":5376239920,"va":5374929616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dce0a0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8255724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e8ecf87d00","size":5,"soft_fail":false,"target_va":5383184544,"va":5374928815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df6180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8415857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e8716a8000","size":5,"soft_fail":false,"target_va":5383348608,"va":5374932746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df6960
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8417916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e87c728000","size":5,"soft_fail":false,"target_va":5383350624,"va":5374932703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=15
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28664452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e88462b501","size":5,"soft_fail":false,"target_va":5403593360,"va":5374928903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28043772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e8fce9ab01","size":5,"soft_fail":false,"target_va":5402978144,"va":5374934367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t2428525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e86d0e2500","size":5,"soft_fail":false,"target_va":5377364336,"va":5374935806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t28044787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e8f3edab01","size":5,"soft_fail":false,"target_va":5402978880,"va":5374934088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28045071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e80fefab01","size":5,"soft_fail":false,"target_va":5402979152,"va":5374934076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t28045918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e85ef2ab01","size":5,"soft_fail":false,"target_va":5402980128,"va":5374934205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t760161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e861990b00","size":5,"soft_fail":false,"target_va":5375692416,"va":5374932250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-721599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e841fdf4ff","size":5,"soft_fail":false,"target_va":5374209808,"va":5374931402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_no_vao
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t771465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e889c50b00","size":5,"soft_fail":false,"target_va":5375704400,"va":5374932930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t768034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e822b80b00","size":5,"soft_fail":false,"target_va":5375700688,"va":5374932649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::read_rg_storage_buffer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t769214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e8bebc0b00","size":5,"soft_fail":false,"target_va":5375702080,"va":5374932861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t463141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e825110700","size":5,"soft_fail":false,"target_va":5375395536,"va":5374932390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t465269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e875190700","size":5,"soft_fail":false,"target_va":5375398240,"va":5374932966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2425017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e8b9002500","size":5,"soft_fail":false,"target_va":5377353776,"va":5374928754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::BufferDesc::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1310516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e834ff1300","size":5,"soft_fail":false,"target_va":5376240784,"va":5374930263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t1413538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e8a2911500","size":5,"soft_fail":false,"target_va":5376341760,"va":5374928217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1412233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e8898c1500","size":5,"soft_fail":false,"target_va":5376340512,"va":5374928274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::sampled
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1412405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e8358d1500","size":5,"soft_fail":false,"target_va":5376340720,"va":5374928310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::storage
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1412455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e8678d1500","size":5,"soft_fail":false,"target_va":5376340752,"va":5374928292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1290900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e894b21300","size":5,"soft_fail":false,"target_va":5376219552,"va":5374928647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2433809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374927472,"raw_hex":"e811232500","size":5,"soft_fail":false,"target_va":5377366224,"va":5374932410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":441,"record_offset":30704,"parent_record_offset":28716,"ancestor_record_offset":28716,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":441,"record_offset":32884,"parent_record_offset":28716,"ancestor_record_offset":28716,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] core::f64::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":441,"record_offset":37232,"parent_record_offset":28716,"ancestor_record_offset":28716,"inlinee":11657,"source_line":955,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":441,"record_offset":32912,"parent_record_offset":28716,"ancestor_record_offset":28716,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":441,"record_offset":32648,"parent_record_offset":28716,"ancestor_record_offset":28716,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":441,"record_offset":32328,"parent_record_offset":28716,"ancestor_record_offset":28716,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":441,"record_offset":32496,"parent_record_offset":28716,"ancestor_record_offset":28716,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::frame_constants::FrameConstants::should_reset_temporal
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/frame_constants.rs
//     evidence={"module":441,"record_offset":37200,"parent_record_offset":28716,"ancestor_record_offset":28716,"inlinee":25418,"source_line":98,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_1d
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":441,"record_offset":38020,"parent_record_offset":28716,"ancestor_record_offset":28716,"inlinee":22896,"source_line":440,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":441,"record_offset":33208,"parent_record_offset":28716,"ancestor_record_offset":28716,"inlinee":22040,"source_line":444,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::push_constants
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":441,"record_offset":37356,"parent_record_offset":28716,"ancestor_record_offset":28716,"inlinee":8420,"source_line":407,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":441,"record_offset":33112,"parent_record_offset":28716,"ancestor_record_offset":28716,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":441,"record_offset":32968,"parent_record_offset":28716,"ancestor_record_offset":28716,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::painterly::raster_debug_target
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/painterly/mod.rs
//     evidence={"module":441,"record_offset":40960,"parent_record_offset":28716,"ancestor_record_offset":28716,"inlinee":25420,"source_line":310,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::painterly::raster_strokes
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/painterly/mod.rs
//     evidence={"module":441,"record_offset":38356,"parent_record_offset":28716,"ancestor_record_offset":28716,"inlinee":25419,"source_line":269,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":441,"record_offset":30760,"parent_record_offset":28716,"ancestor_record_offset":28716,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":441,"record_offset":28940,"parent_record_offset":28716,"ancestor_record_offset":28716,"inlinee":9929,"source_line":41,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":441,"record_offset":32940,"parent_record_offset":28716,"ancestor_record_offset":28716,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":441,"record_offset":32800,"parent_record_offset":28716,"ancestor_record_offset":28716,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":441,"record_offset":32704,"parent_record_offset":28716,"ancestor_record_offset":28716,"inlinee":9081,"source_line":40,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":441,"record_offset":37636,"parent_record_offset":37612,"ancestor_record_offset":28716,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":441,"record_offset":37660,"parent_record_offset":37636,"ancestor_record_offset":28716,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":441,"record_offset":37972,"parent_record_offset":37948,"ancestor_record_offset":28716,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":441,"record_offset":37612,"parent_record_offset":37584,"ancestor_record_offset":28716,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":441,"record_offset":37948,"parent_record_offset":37892,"ancestor_record_offset":28716,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":441,"record_offset":37920,"parent_record_offset":37892,"ancestor_record_offset":28716,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":441,"record_offset":37584,"parent_record_offset":37556,"ancestor_record_offset":28716,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":441,"record_offset":37556,"parent_record_offset":37528,"ancestor_record_offset":28716,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":441,"record_offset":37732,"parent_record_offset":37556,"ancestor_record_offset":28716,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":441,"record_offset":37892,"parent_record_offset":37864,"ancestor_record_offset":28716,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":441,"record_offset":37492,"parent_record_offset":37464,"ancestor_record_offset":28716,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":441,"record_offset":37464,"parent_record_offset":37436,"ancestor_record_offset":28716,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":441,"record_offset":37408,"parent_record_offset":37356,"ancestor_record_offset":28716,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":441,"record_offset":37436,"parent_record_offset":37408,"ancestor_record_offset":28716,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":441,"record_offset":37528,"parent_record_offset":37492,"ancestor_record_offset":28716,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":441,"record_offset":30728,"parent_record_offset":30704,"ancestor_record_offset":28716,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":441,"record_offset":28996,"parent_record_offset":28968,"ancestor_record_offset":28716,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":441,"record_offset":30508,"parent_record_offset":30484,"ancestor_record_offset":28716,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":441,"record_offset":30484,"parent_record_offset":30460,"ancestor_record_offset":28716,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":441,"record_offset":30432,"parent_record_offset":30400,"ancestor_record_offset":28716,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":441,"record_offset":32196,"parent_record_offset":31808,"ancestor_record_offset":28716,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":441,"record_offset":30376,"parent_record_offset":29984,"ancestor_record_offset":28716,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":441,"record_offset":30400,"parent_record_offset":30376,"ancestor_record_offset":28716,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":441,"record_offset":30016,"parent_record_offset":29984,"ancestor_record_offset":28716,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":441,"record_offset":30460,"parent_record_offset":30400,"ancestor_record_offset":28716,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":441,"record_offset":30064,"parent_record_offset":30040,"ancestor_record_offset":28716,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":441,"record_offset":30040,"parent_record_offset":30016,"ancestor_record_offset":28716,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":441,"record_offset":30156,"parent_record_offset":30092,"ancestor_record_offset":28716,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":441,"record_offset":30244,"parent_record_offset":30220,"ancestor_record_offset":28716,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":441,"record_offset":30576,"parent_record_offset":30552,"ancestor_record_offset":28716,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":441,"record_offset":30552,"parent_record_offset":29984,"ancestor_record_offset":28716,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":441,"record_offset":30792,"parent_record_offset":30760,"ancestor_record_offset":28716,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":441,"record_offset":31808,"parent_record_offset":30792,"ancestor_record_offset":28716,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":441,"record_offset":28968,"parent_record_offset":28940,"ancestor_record_offset":28716,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":441,"record_offset":29984,"parent_record_offset":28968,"ancestor_record_offset":28716,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":441,"record_offset":29716,"parent_record_offset":29692,"ancestor_record_offset":28716,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":441,"record_offset":40540,"parent_record_offset":40512,"ancestor_record_offset":28716,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":441,"record_offset":40592,"parent_record_offset":40568,"ancestor_record_offset":28716,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":441,"record_offset":40568,"parent_record_offset":40512,"ancestor_record_offset":28716,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":441,"record_offset":29764,"parent_record_offset":29740,"ancestor_record_offset":28716,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":441,"record_offset":29692,"parent_record_offset":29668,"ancestor_record_offset":28716,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":441,"record_offset":32432,"parent_record_offset":32364,"ancestor_record_offset":28716,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":441,"record_offset":29184,"parent_record_offset":29160,"ancestor_record_offset":28716,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":441,"record_offset":29268,"parent_record_offset":29244,"ancestor_record_offset":28716,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":441,"record_offset":29208,"parent_record_offset":29184,"ancestor_record_offset":28716,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":441,"record_offset":40616,"parent_record_offset":40592,"ancestor_record_offset":28716,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":441,"record_offset":29380,"parent_record_offset":29356,"ancestor_record_offset":28716,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":441,"record_offset":30180,"parent_record_offset":30156,"ancestor_record_offset":28716,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":441,"record_offset":29952,"parent_record_offset":28996,"ancestor_record_offset":28716,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":441,"record_offset":32672,"parent_record_offset":32648,"ancestor_record_offset":28716,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":441,"record_offset":30220,"parent_record_offset":30040,"ancestor_record_offset":28716,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":441,"record_offset":30672,"parent_record_offset":28940,"ancestor_record_offset":28716,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":441,"record_offset":30324,"parent_record_offset":30300,"ancestor_record_offset":28716,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":441,"record_offset":37768,"parent_record_offset":37492,"ancestor_record_offset":28716,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":441,"record_offset":32540,"parent_record_offset":32496,"ancestor_record_offset":28716,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":441,"record_offset":29604,"parent_record_offset":29580,"ancestor_record_offset":28716,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":441,"record_offset":29524,"parent_record_offset":29500,"ancestor_record_offset":28716,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":441,"record_offset":37688,"parent_record_offset":37660,"ancestor_record_offset":28716,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":441,"record_offset":40640,"parent_record_offset":40616,"ancestor_record_offset":28716,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":441,"record_offset":30092,"parent_record_offset":30064,"ancestor_record_offset":28716,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":441,"record_offset":30128,"parent_record_offset":30092,"ancestor_record_offset":28716,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":441,"record_offset":30300,"parent_record_offset":30268,"ancestor_record_offset":28716,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":441,"record_offset":30600,"parent_record_offset":30576,"ancestor_record_offset":28716,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":441,"record_offset":32824,"parent_record_offset":32800,"ancestor_record_offset":28716,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":441,"record_offset":30624,"parent_record_offset":30600,"ancestor_record_offset":28716,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":441,"record_offset":29740,"parent_record_offset":29716,"ancestor_record_offset":28716,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":441,"record_offset":33240,"parent_record_offset":33208,"ancestor_record_offset":28716,"inlinee":21934,"source_line":477,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":441,"record_offset":40712,"parent_record_offset":38356,"ancestor_record_offset":28716,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":441,"record_offset":40180,"parent_record_offset":38356,"ancestor_record_offset":28716,"inlinee":21986,"source_line":105,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":441,"record_offset":40000,"parent_record_offset":38356,"ancestor_record_offset":28716,"inlinee":7327,"source_line":314,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":441,"record_offset":40220,"parent_record_offset":40180,"ancestor_record_offset":28716,"inlinee":21988,"source_line":137,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::ColorAttachment::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":441,"record_offset":40108,"parent_record_offset":40080,"ancestor_record_offset":28716,"inlinee":23122,"source_line":89,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":441,"record_offset":40080,"parent_record_offset":40056,"ancestor_record_offset":28716,"inlinee":6964,"source_line":145,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$5::from
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":441,"record_offset":40056,"parent_record_offset":40032,"ancestor_record_offset":28716,"inlinee":7000,"source_line":179,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::painterly::painterly_pass::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/painterly/mod.rs
//     evidence={"module":441,"record_offset":32364,"parent_record_offset":32328,"ancestor_record_offset":28716,"inlinee":25416,"source_line":116,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::impl$65::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":441,"record_offset":32456,"parent_record_offset":32432,"ancestor_record_offset":28716,"inlinee":22963,"source_line":1123,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":441,"record_offset":29668,"parent_record_offset":29644,"ancestor_record_offset":28716,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":441,"record_offset":29020,"parent_record_offset":28996,"ancestor_record_offset":28716,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":441,"record_offset":29044,"parent_record_offset":29020,"ancestor_record_offset":28716,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":441,"record_offset":29644,"parent_record_offset":29448,"ancestor_record_offset":28716,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":29556,"parent_record_offset":29472,"ancestor_record_offset":28716,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":29500,"parent_record_offset":29472,"ancestor_record_offset":28716,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":29900,"parent_record_offset":29120,"ancestor_record_offset":28716,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":29472,"parent_record_offset":29448,"ancestor_record_offset":28716,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":29096,"parent_record_offset":29072,"ancestor_record_offset":28716,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":29072,"parent_record_offset":29044,"ancestor_record_offset":28716,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":29120,"parent_record_offset":29096,"ancestor_record_offset":28716,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":441,"record_offset":29328,"parent_record_offset":29300,"ancestor_record_offset":28716,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":441,"record_offset":29356,"parent_record_offset":29328,"ancestor_record_offset":28716,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":441,"record_offset":29416,"parent_record_offset":29300,"ancestor_record_offset":28716,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":441,"record_offset":29300,"parent_record_offset":29120,"ancestor_record_offset":28716,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":29448,"parent_record_offset":29120,"ancestor_record_offset":28716,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":441,"record_offset":29160,"parent_record_offset":29120,"ancestor_record_offset":28716,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":441,"record_offset":29244,"parent_record_offset":29120,"ancestor_record_offset":28716,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":441,"record_offset":29816,"parent_record_offset":29120,"ancestor_record_offset":28716,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":441,"record_offset":40804,"parent_record_offset":40780,"ancestor_record_offset":28716,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":441,"record_offset":30268,"parent_record_offset":30244,"ancestor_record_offset":28716,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":441,"record_offset":40840,"parent_record_offset":38356,"ancestor_record_offset":28716,"inlinee":7340,"source_line":626,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":441,"record_offset":40864,"parent_record_offset":40840,"ancestor_record_offset":28716,"inlinee":7341,"source_line":626,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":441,"record_offset":40756,"parent_record_offset":38356,"ancestor_record_offset":28716,"inlinee":7336,"source_line":617,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable_checked
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":441,"record_offset":40780,"parent_record_offset":40756,"ancestor_record_offset":28716,"inlinee":7337,"source_line":617,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":441,"record_offset":32740,"parent_record_offset":32704,"ancestor_record_offset":28716,"inlinee":9072,"source_line":67,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":441,"record_offset":32404,"parent_record_offset":32364,"ancestor_record_offset":28716,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":441,"record_offset":40248,"parent_record_offset":40220,"ancestor_record_offset":28716,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":441,"record_offset":40304,"parent_record_offset":40276,"ancestor_record_offset":28716,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":441,"record_offset":40392,"parent_record_offset":40360,"ancestor_record_offset":28716,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":441,"record_offset":40512,"parent_record_offset":40360,"ancestor_record_offset":28716,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":441,"record_offset":40276,"parent_record_offset":40248,"ancestor_record_offset":28716,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":441,"record_offset":40416,"parent_record_offset":40392,"ancestor_record_offset":28716,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":441,"record_offset":40444,"parent_record_offset":40416,"ancestor_record_offset":28716,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":441,"record_offset":40360,"parent_record_offset":40332,"ancestor_record_offset":28716,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":441,"record_offset":40332,"parent_record_offset":40304,"ancestor_record_offset":28716,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::painterly::painterly_pass::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 116
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6348
// 首条位置: {"module":441,"inlinee":25416}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":441,"record_offset":32432,"parent_record_offset":32364,"ancestor_record_offset":32364,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":441,"record_offset":32404,"parent_record_offset":32364,"ancestor_record_offset":32364,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::impl$65::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":441,"record_offset":32456,"parent_record_offset":32432,"ancestor_record_offset":32364,"inlinee":22963,"source_line":1123,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::painterly::raster_strokes
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 269
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x634B
// 首条位置: {"module":441,"inlinee":25419}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":441,"record_offset":38460,"parent_record_offset":38356,"ancestor_record_offset":38356,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":441,"record_offset":39964,"parent_record_offset":38356,"ancestor_record_offset":38356,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":441,"record_offset":40712,"parent_record_offset":38356,"ancestor_record_offset":38356,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":441,"record_offset":40180,"parent_record_offset":38356,"ancestor_record_offset":38356,"inlinee":21986,"source_line":105,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":441,"record_offset":40000,"parent_record_offset":38356,"ancestor_record_offset":38356,"inlinee":7327,"source_line":314,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":441,"record_offset":40840,"parent_record_offset":38356,"ancestor_record_offset":38356,"inlinee":7340,"source_line":626,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":441,"record_offset":40756,"parent_record_offset":38356,"ancestor_record_offset":38356,"inlinee":7336,"source_line":617,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":441,"record_offset":38488,"parent_record_offset":38460,"ancestor_record_offset":38356,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":441,"record_offset":39920,"parent_record_offset":39888,"ancestor_record_offset":38356,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":441,"record_offset":39864,"parent_record_offset":39476,"ancestor_record_offset":38356,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":441,"record_offset":39888,"parent_record_offset":39864,"ancestor_record_offset":38356,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":441,"record_offset":39504,"parent_record_offset":39476,"ancestor_record_offset":38356,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":441,"record_offset":39552,"parent_record_offset":39528,"ancestor_record_offset":38356,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":441,"record_offset":39528,"parent_record_offset":39504,"ancestor_record_offset":38356,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":441,"record_offset":39644,"parent_record_offset":39580,"ancestor_record_offset":38356,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":441,"record_offset":39732,"parent_record_offset":39708,"ancestor_record_offset":38356,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":441,"record_offset":39476,"parent_record_offset":38460,"ancestor_record_offset":38356,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":441,"record_offset":39208,"parent_record_offset":39184,"ancestor_record_offset":38356,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":441,"record_offset":40540,"parent_record_offset":40512,"ancestor_record_offset":38356,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":441,"record_offset":40592,"parent_record_offset":40568,"ancestor_record_offset":38356,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":441,"record_offset":40568,"parent_record_offset":40512,"ancestor_record_offset":38356,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":441,"record_offset":39256,"parent_record_offset":39232,"ancestor_record_offset":38356,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":441,"record_offset":39184,"parent_record_offset":39160,"ancestor_record_offset":38356,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":441,"record_offset":40032,"parent_record_offset":40000,"ancestor_record_offset":38356,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":441,"record_offset":38676,"parent_record_offset":38652,"ancestor_record_offset":38356,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":441,"record_offset":38760,"parent_record_offset":38736,"ancestor_record_offset":38356,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":441,"record_offset":38700,"parent_record_offset":38676,"ancestor_record_offset":38356,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":441,"record_offset":40616,"parent_record_offset":40592,"ancestor_record_offset":38356,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":441,"record_offset":38872,"parent_record_offset":38848,"ancestor_record_offset":38356,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":441,"record_offset":39668,"parent_record_offset":39644,"ancestor_record_offset":38356,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":441,"record_offset":39444,"parent_record_offset":38488,"ancestor_record_offset":38356,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":441,"record_offset":39708,"parent_record_offset":39528,"ancestor_record_offset":38356,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":441,"record_offset":39812,"parent_record_offset":39788,"ancestor_record_offset":38356,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":441,"record_offset":39096,"parent_record_offset":39072,"ancestor_record_offset":38356,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":441,"record_offset":39016,"parent_record_offset":38992,"ancestor_record_offset":38356,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":441,"record_offset":40640,"parent_record_offset":40616,"ancestor_record_offset":38356,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":441,"record_offset":39580,"parent_record_offset":39552,"ancestor_record_offset":38356,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":441,"record_offset":39616,"parent_record_offset":39580,"ancestor_record_offset":38356,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":441,"record_offset":39788,"parent_record_offset":39756,"ancestor_record_offset":38356,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":441,"record_offset":39232,"parent_record_offset":39208,"ancestor_record_offset":38356,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":441,"record_offset":40220,"parent_record_offset":40180,"ancestor_record_offset":38356,"inlinee":21988,"source_line":137,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::ColorAttachment::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":441,"record_offset":40108,"parent_record_offset":40080,"ancestor_record_offset":38356,"inlinee":23122,"source_line":89,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":441,"record_offset":40080,"parent_record_offset":40056,"ancestor_record_offset":38356,"inlinee":6964,"source_line":145,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$5::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":441,"record_offset":40056,"parent_record_offset":40032,"ancestor_record_offset":38356,"inlinee":7000,"source_line":179,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":441,"record_offset":39160,"parent_record_offset":39136,"ancestor_record_offset":38356,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":441,"record_offset":38512,"parent_record_offset":38488,"ancestor_record_offset":38356,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":441,"record_offset":38536,"parent_record_offset":38512,"ancestor_record_offset":38356,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":441,"record_offset":39136,"parent_record_offset":38940,"ancestor_record_offset":38356,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":39048,"parent_record_offset":38964,"ancestor_record_offset":38356,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":38992,"parent_record_offset":38964,"ancestor_record_offset":38356,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":39392,"parent_record_offset":38612,"ancestor_record_offset":38356,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":38964,"parent_record_offset":38940,"ancestor_record_offset":38356,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":38588,"parent_record_offset":38564,"ancestor_record_offset":38356,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":38564,"parent_record_offset":38536,"ancestor_record_offset":38356,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":38612,"parent_record_offset":38588,"ancestor_record_offset":38356,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":441,"record_offset":38820,"parent_record_offset":38792,"ancestor_record_offset":38356,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":441,"record_offset":38848,"parent_record_offset":38820,"ancestor_record_offset":38356,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":441,"record_offset":38908,"parent_record_offset":38792,"ancestor_record_offset":38356,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":441,"record_offset":38792,"parent_record_offset":38612,"ancestor_record_offset":38356,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":38940,"parent_record_offset":38612,"ancestor_record_offset":38356,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":441,"record_offset":38652,"parent_record_offset":38612,"ancestor_record_offset":38356,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":441,"record_offset":38736,"parent_record_offset":38612,"ancestor_record_offset":38356,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":441,"record_offset":39308,"parent_record_offset":38612,"ancestor_record_offset":38356,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":441,"record_offset":40804,"parent_record_offset":40780,"ancestor_record_offset":38356,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":441,"record_offset":39756,"parent_record_offset":39732,"ancestor_record_offset":38356,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":441,"record_offset":40864,"parent_record_offset":40840,"ancestor_record_offset":38356,"inlinee":7341,"source_line":626,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":441,"record_offset":40780,"parent_record_offset":40756,"ancestor_record_offset":38356,"inlinee":7337,"source_line":617,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":441,"record_offset":40132,"parent_record_offset":40108,"ancestor_record_offset":38356,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":441,"record_offset":40248,"parent_record_offset":40220,"ancestor_record_offset":38356,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":441,"record_offset":40304,"parent_record_offset":40276,"ancestor_record_offset":38356,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":441,"record_offset":40392,"parent_record_offset":40360,"ancestor_record_offset":38356,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":441,"record_offset":40512,"parent_record_offset":40360,"ancestor_record_offset":38356,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":441,"record_offset":40276,"parent_record_offset":40248,"ancestor_record_offset":38356,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":441,"record_offset":40416,"parent_record_offset":40392,"ancestor_record_offset":38356,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":441,"record_offset":40444,"parent_record_offset":40416,"ancestor_record_offset":38356,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":441,"record_offset":40360,"parent_record_offset":40332,"ancestor_record_offset":38356,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":441,"record_offset":40332,"parent_record_offset":40304,"ancestor_record_offset":38356,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::painterly::raster_debug_target
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 310
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x634C
// 首条位置: {"module":441,"inlinee":25420}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":441,"record_offset":41068,"parent_record_offset":40960,"ancestor_record_offset":40960,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":441,"record_offset":42572,"parent_record_offset":40960,"ancestor_record_offset":40960,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":441,"record_offset":43264,"parent_record_offset":40960,"ancestor_record_offset":40960,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":441,"record_offset":42732,"parent_record_offset":40960,"ancestor_record_offset":40960,"inlinee":21986,"source_line":105,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":441,"record_offset":42608,"parent_record_offset":40960,"ancestor_record_offset":40960,"inlinee":7327,"source_line":314,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":441,"record_offset":43396,"parent_record_offset":40960,"ancestor_record_offset":40960,"inlinee":7340,"source_line":626,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":441,"record_offset":43312,"parent_record_offset":40960,"ancestor_record_offset":40960,"inlinee":7336,"source_line":617,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":441,"record_offset":41096,"parent_record_offset":41068,"ancestor_record_offset":40960,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":441,"record_offset":42528,"parent_record_offset":42496,"ancestor_record_offset":40960,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":441,"record_offset":42472,"parent_record_offset":42084,"ancestor_record_offset":40960,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":441,"record_offset":42496,"parent_record_offset":42472,"ancestor_record_offset":40960,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":441,"record_offset":42112,"parent_record_offset":42084,"ancestor_record_offset":40960,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":441,"record_offset":42160,"parent_record_offset":42136,"ancestor_record_offset":40960,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":441,"record_offset":42136,"parent_record_offset":42112,"ancestor_record_offset":40960,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":441,"record_offset":42252,"parent_record_offset":42188,"ancestor_record_offset":40960,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":441,"record_offset":42340,"parent_record_offset":42316,"ancestor_record_offset":40960,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":441,"record_offset":42084,"parent_record_offset":41068,"ancestor_record_offset":40960,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":441,"record_offset":41816,"parent_record_offset":41792,"ancestor_record_offset":40960,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":441,"record_offset":43092,"parent_record_offset":43064,"ancestor_record_offset":40960,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":441,"record_offset":43144,"parent_record_offset":43120,"ancestor_record_offset":40960,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":441,"record_offset":43120,"parent_record_offset":43064,"ancestor_record_offset":40960,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":441,"record_offset":41864,"parent_record_offset":41840,"ancestor_record_offset":40960,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":441,"record_offset":41792,"parent_record_offset":41768,"ancestor_record_offset":40960,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":441,"record_offset":42640,"parent_record_offset":42608,"ancestor_record_offset":40960,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":441,"record_offset":41284,"parent_record_offset":41260,"ancestor_record_offset":40960,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":441,"record_offset":41368,"parent_record_offset":41344,"ancestor_record_offset":40960,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":441,"record_offset":41308,"parent_record_offset":41284,"ancestor_record_offset":40960,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":441,"record_offset":43168,"parent_record_offset":43144,"ancestor_record_offset":40960,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":441,"record_offset":41480,"parent_record_offset":41456,"ancestor_record_offset":40960,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":441,"record_offset":42276,"parent_record_offset":42252,"ancestor_record_offset":40960,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":441,"record_offset":42052,"parent_record_offset":41096,"ancestor_record_offset":40960,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":441,"record_offset":42316,"parent_record_offset":42136,"ancestor_record_offset":40960,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":441,"record_offset":42420,"parent_record_offset":42396,"ancestor_record_offset":40960,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":441,"record_offset":41704,"parent_record_offset":41680,"ancestor_record_offset":40960,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":441,"record_offset":41624,"parent_record_offset":41600,"ancestor_record_offset":40960,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":441,"record_offset":43192,"parent_record_offset":43168,"ancestor_record_offset":40960,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":441,"record_offset":42188,"parent_record_offset":42160,"ancestor_record_offset":40960,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":441,"record_offset":42224,"parent_record_offset":42188,"ancestor_record_offset":40960,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":441,"record_offset":42396,"parent_record_offset":42364,"ancestor_record_offset":40960,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":441,"record_offset":41840,"parent_record_offset":41816,"ancestor_record_offset":40960,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":441,"record_offset":42772,"parent_record_offset":42732,"ancestor_record_offset":40960,"inlinee":21988,"source_line":137,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":441,"record_offset":42688,"parent_record_offset":42664,"ancestor_record_offset":40960,"inlinee":6964,"source_line":145,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$5::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":441,"record_offset":42664,"parent_record_offset":42640,"ancestor_record_offset":40960,"inlinee":7000,"source_line":179,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":441,"record_offset":41768,"parent_record_offset":41744,"ancestor_record_offset":40960,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":441,"record_offset":41120,"parent_record_offset":41096,"ancestor_record_offset":40960,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":441,"record_offset":41144,"parent_record_offset":41120,"ancestor_record_offset":40960,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":441,"record_offset":41744,"parent_record_offset":41548,"ancestor_record_offset":40960,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":41656,"parent_record_offset":41572,"ancestor_record_offset":40960,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":41600,"parent_record_offset":41572,"ancestor_record_offset":40960,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":42000,"parent_record_offset":41220,"ancestor_record_offset":40960,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":41572,"parent_record_offset":41548,"ancestor_record_offset":40960,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":41196,"parent_record_offset":41172,"ancestor_record_offset":40960,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":41172,"parent_record_offset":41144,"ancestor_record_offset":40960,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":41220,"parent_record_offset":41196,"ancestor_record_offset":40960,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":441,"record_offset":41428,"parent_record_offset":41400,"ancestor_record_offset":40960,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":441,"record_offset":41456,"parent_record_offset":41428,"ancestor_record_offset":40960,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":441,"record_offset":41516,"parent_record_offset":41400,"ancestor_record_offset":40960,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":441,"record_offset":41400,"parent_record_offset":41220,"ancestor_record_offset":40960,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":441,"record_offset":41548,"parent_record_offset":41220,"ancestor_record_offset":40960,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":441,"record_offset":41260,"parent_record_offset":41220,"ancestor_record_offset":40960,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":441,"record_offset":41344,"parent_record_offset":41220,"ancestor_record_offset":40960,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":441,"record_offset":41916,"parent_record_offset":41220,"ancestor_record_offset":40960,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":441,"record_offset":43360,"parent_record_offset":43336,"ancestor_record_offset":40960,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":441,"record_offset":42364,"parent_record_offset":42340,"ancestor_record_offset":40960,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":441,"record_offset":43420,"parent_record_offset":43396,"ancestor_record_offset":40960,"inlinee":7341,"source_line":626,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":441,"record_offset":43336,"parent_record_offset":43312,"ancestor_record_offset":40960,"inlinee":7337,"source_line":617,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":441,"record_offset":42800,"parent_record_offset":42772,"ancestor_record_offset":40960,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":441,"record_offset":42856,"parent_record_offset":42828,"ancestor_record_offset":40960,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":441,"record_offset":42944,"parent_record_offset":42912,"ancestor_record_offset":40960,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":441,"record_offset":43064,"parent_record_offset":42912,"ancestor_record_offset":40960,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":441,"record_offset":42828,"parent_record_offset":42800,"ancestor_record_offset":40960,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":441,"record_offset":42968,"parent_record_offset":42944,"ancestor_record_offset":40960,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":441,"record_offset":42996,"parent_record_offset":42968,"ancestor_record_offset":40960,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":441,"record_offset":42912,"parent_record_offset":42884,"ancestor_record_offset":40960,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":441,"record_offset":42884,"parent_record_offset":42856,"ancestor_record_offset":40960,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::painterly::startup_painterly
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 347
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5C6D
// 首条位置: {"module":413,"inlinee":23661}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":413,"record_offset":12604,"parent_record_offset":12572,"ancestor_record_offset":12572,"inlinee":22680,"source_line":43,"archive":"_pdb/A/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":413,"record_offset":12628,"parent_record_offset":12604,"ancestor_record_offset":12572,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00413.jsonl.gz"}
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
// 完整结构及原始类型记录: mod.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::painterly::PainterlyPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::systems::render_loop::painterly::PainterlyPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::systems::render_loop::painterly::PainterlyPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<country_core::systems::render_loop::painterly::PainterlyPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["country_core::systems::render_loop::painterly::PainterlyPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<country_core::systems::render_loop::painterly::PainterlyPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["country_core::systems::render_loop::painterly::PainterlyPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<country_core::systems::render_loop::painterly::PainterlyPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["country_core::systems::render_loop::painterly::PainterlyPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<country_core::systems::render_loop::painterly::PainterlyPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["country_core::systems::render_loop::painterly::PainterlyPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::finish<country_core::systems::render_loop::painterly::PainterlyPlugin>", "constructor": "bevy_app::plugin::Plugin::finish", "arguments": ["country_core::systems::render_loop::painterly::PainterlyPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::is_unique<country_core::systems::render_loop::painterly::PainterlyPlugin>", "constructor": "bevy_app::plugin::Plugin::is_unique", "arguments": ["country_core::systems::render_loop::painterly::PainterlyPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$14::add_to_app<bevy_app::plugin::sealed::PluginMarker,tiny_glade::game_loop::GameLoopPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::main_camera::MainCameraPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::Ui2dPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::mei::core::MeiPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::ToolbarPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::session::SessionFlowPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::menu::MenuFlowPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::painterly::PainterlyPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::screenshot::ScreenshotPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::lut_gallery::LutGalleryPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::gui::settings::SettingsMenuPlugin>", "constructor": "bevy_app::plugin::sealed::impl$14::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "tiny_glade::game_loop::GameLoopPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::main_camera::MainCameraPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::Ui2dPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::mei::core::MeiPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::ToolbarPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::session::SessionFlowPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::menu::MenuFlowPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::painterly::PainterlyPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::screenshot::ScreenshotPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::lut_gallery::LutGalleryPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::gui::settings::SettingsMenuPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::painterly::PainterlyState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::systems::render_loop::painterly::PainterlyState>", "constructor": "enum2$", "arguments": ["country_core::systems::render_loop::painterly::PainterlyState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::app::App::insert_resource<enum2$<country_core::systems::render_loop::painterly::PainterlyState> >", "constructor": "bevy_app::app::App::insert_resource", "arguments": ["enum2$<country_core::systems::render_loop::painterly::PainterlyState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mod.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/painterly/mod.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: mod.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::painterly::painterly_pass
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":441,"record_offset":32968,"parent_record_offset":28716,"ancestor_record_offset":28716,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00441.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":441,"record_offset":33112,"parent_record_offset":28716,"ancestor_record_offset":28716,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00441.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":441,"record_offset":40712,"parent_record_offset":38356,"ancestor_record_offset":28716,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00441.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::painterly::raster_strokes
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":441,"record_offset":40712,"parent_record_offset":38356,"ancestor_record_offset":38356,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00441.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::painterly::raster_debug_target
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":441,"record_offset":43264,"parent_record_offset":40960,"ancestor_record_offset":40960,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00441.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3143493,"position_among_mapped_render_targets":57,"mapped_render_target_count":70,"previous_callsite":3143431,"next_callsite":3144035,"previous_target_files":["crates/country-core/src/systems/render_loop/menu_background.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/vermeer.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3147645,"position_among_mapped_render_targets":70,"mapped_render_target_count":70,"previous_callsite":3145773,"next_callsite":null,"previous_target_files":["crates/country-core/src/systems/render_loop/mod.rs"],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mod.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 206; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::event::EventReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::event::EventReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
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
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::painterly::PainterlyState::has_target
// 信息 ID: d31ae3df2bd1430923ca5f04
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x30077d -> 0x5ee230 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::painterly::impl$1::build
// 信息 ID: 4a010a1599734b110a3ce90c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::painterly::painterly_pass
// 信息 ID: 628e56f6b3a50c98781feb05
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2ff745 -> 0x5ee270 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::painterly::painterly_pass::closure$0
// 信息 ID: ddc3de10a5830e4ed1dbcc17
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::painterly::painterly_pass", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::painterly::painterly_pass; module=441, inline_record=32364; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::painterly::raster_debug_target
// 信息 ID: ed009efef8b5991af9f752c9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::painterly::painterly_pass; module=441, inline_record=40960; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::painterly::raster_strokes
// 信息 ID: b7bfed11ebcd239e46900d02
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::painterly::painterly_pass; module=441, inline_record=38356; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::painterly::startup_painterly
// 信息 ID: 921a38f2c91c5ec638416469
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] f6e723bf3a293de264db1c5c: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)>; module=413, inline_record=12572; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)> >; module=1325, inline_record=1559228; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
