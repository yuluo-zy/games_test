// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/draw_passes/object_grass_push.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 566069c235976656bcaa07e2424157d392271eb80073d41800d1e36845733000
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: object_grass_push.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 415, 1308
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
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"PDB_registry_path_only","version":"0.14.5"}]
// use modular_bitfield; // namespace candidate
//   package identities: [{"name":"modular-bitfield","source":"PDB_registry_path_only","version":"0.11.2"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":415,"record_offset":24256,"rva":5778192,"va":5374487312}
// 编译实例 RVAs: 0x582b10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140361320
//     owner=unresolved_namespace; count=2
//     target_aliases=228 candidates; complete list: object_grass_push.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2237542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374487312,"raw_hex":"e89adbddff","size":5,"soft_fail":false,"target_va":5372252960,"va":5374490497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: object_grass_push.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1865738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374487312,"raw_hex":"e8f687e3ff","size":5,"soft_fail":false,"target_va":5372625024,"va":5374490757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: object_grass_push.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1741373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374487312,"raw_hex":"e8c36de5ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374491080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eb110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: object_grass_push.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1471802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374487312,"raw_hex":"e83a751600","size":5,"soft_fail":false,"target_va":5375963408,"va":5374491601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072a6e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: object_grass_push.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1732223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374487312,"raw_hex":"e87f6e1a00","size":5,"soft_fail":false,"target_va":5376222944,"va":5374490716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d825a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: object_grass_push.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8384042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374487312,"raw_hex":"e82aee7f00","size":5,"soft_fail":false,"target_va":5382874528,"va":5374490481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: object_grass_push.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8820391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374487312,"raw_hex":"e8a7968600","size":5,"soft_fail":false,"target_va":5383308144,"va":5374487748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df67d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: object_grass_push.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8859240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374487312,"raw_hex":"e8682e8700","size":5,"soft_fail":false,"target_va":5383350224,"va":5374490979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: object_grass_push.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29102335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374487312,"raw_hex":"e8ff10bc01","size":5,"soft_fail":false,"target_va":5403593360,"va":5374491020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::state::QueryState$LT$D$C$F$GT$::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9697533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374487312,"raw_hex":"e8fdf89300","size":5,"soft_fail":false,"target_va":5384186960,"va":5374489422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::state::QueryState$LT$D$C$F$GT$::update_archetypes_unsafe_world_cell
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9690702","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374487312,"raw_hex":"e84ede9300","size":5,"soft_fail":false,"target_va":5384180144,"va":5374489437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28487379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374487312,"raw_hex":"e8d3aeb201","size":5,"soft_fail":false,"target_va":5402978688,"va":5374491304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t28487697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374487312,"raw_hex":"e811b0b201","size":5,"soft_fail":false,"target_va":5402978880,"va":5374491178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374487312,"raw_hex":"e8d8ddffff","size":5,"soft_fail":false,"target_va":5374482400,"va":5374491139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28492326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374487312,"raw_hex":"e826c2b201","size":5,"soft_fail":false,"target_va":5402980128,"va":5374487797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374487312,"raw_hex":"e845edffff","size":5,"soft_fail":false,"target_va":5374482592,"va":5374487382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t28493192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374487312,"raw_hex":"e888c5b201","size":5,"soft_fail":false,"target_va":5402980656,"va":5374487459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1204143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374487312,"raw_hex":"e8af5f1200","size":5,"soft_fail":false,"target_va":5375692416,"va":5374488268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_rg_texture
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-132848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374487312,"raw_hex":"e810f9fdff","size":5,"soft_fail":false,"target_va":5374355936,"va":5374488779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_no_vao
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1213362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374487312,"raw_hex":"e8b2831200","size":5,"soft_fail":false,"target_va":5375704400,"va":5374491033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1209735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374487312,"raw_hex":"e887751200","size":5,"soft_fail":false,"target_va":5375700688,"va":5374490948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t904908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374487312,"raw_hex":"e8ccce0d00","size":5,"soft_fail":false,"target_va":5375395536,"va":5374490623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t907115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374487312,"raw_hex":"e86bd70d00","size":5,"soft_fail":false,"target_va":5375398240,"va":5374491120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1868944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374487312,"raw_hex":"e890841c00","size":5,"soft_fail":false,"target_va":5376356912,"va":5374487963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1872827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374487312,"raw_hex":"e8bb931c00","size":5,"soft_fail":false,"target_va":5376360560,"va":5374487728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1868776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374487312,"raw_hex":"e8e8831c00","size":5,"soft_fail":false,"target_va":5376356144,"va":5374487363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1851239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374487312,"raw_hex":"e8673f1c00","size":5,"soft_fail":false,"target_va":5376341760,"va":5374490516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2875495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374487312,"raw_hex":"e867e02b00","size":5,"soft_fail":false,"target_va":5377366224,"va":5374490724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::camera::CameraMatrices::eye_position
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8671280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374487312,"raw_hex":"e830508400","size":5,"soft_fail":false,"target_va":5383161424,"va":5374490139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":415,"record_offset":40716,"parent_record_offset":24256,"ancestor_record_offset":24256,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::state::QueryState::iter
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":415,"record_offset":35156,"parent_record_offset":24256,"ancestor_record_offset":24256,"inlinee":22611,"source_line":972,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::query
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":415,"record_offset":35100,"parent_record_offset":24256,"ancestor_record_offset":24256,"inlinee":22609,"source_line":1220,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":415,"record_offset":27136,"parent_record_offset":24256,"ancestor_record_offset":24256,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":415,"record_offset":35388,"parent_record_offset":24256,"ancestor_record_offset":24256,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":415,"record_offset":40396,"parent_record_offset":24256,"ancestor_record_offset":24256,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":415,"record_offset":40452,"parent_record_offset":24256,"ancestor_record_offset":24256,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":415,"record_offset":28652,"parent_record_offset":24256,"ancestor_record_offset":24256,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":415,"record_offset":41548,"parent_record_offset":24256,"ancestor_record_offset":24256,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":415,"record_offset":40844,"parent_record_offset":24256,"ancestor_record_offset":24256,"inlinee":21986,"source_line":105,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":415,"record_offset":40536,"parent_record_offset":24256,"ancestor_record_offset":24256,"inlinee":7327,"source_line":314,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":415,"record_offset":31964,"parent_record_offset":24256,"ancestor_record_offset":24256,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":415,"record_offset":30204,"parent_record_offset":24256,"ancestor_record_offset":24256,"inlinee":9929,"source_line":41,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":415,"record_offset":26088,"parent_record_offset":24256,"ancestor_record_offset":24256,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":415,"record_offset":24500,"parent_record_offset":24256,"ancestor_record_offset":24256,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":415,"record_offset":25076,"parent_record_offset":24256,"ancestor_record_offset":24256,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":415,"record_offset":41460,"parent_record_offset":24256,"ancestor_record_offset":24256,"inlinee":7340,"source_line":626,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":415,"record_offset":41376,"parent_record_offset":24256,"ancestor_record_offset":24256,"inlinee":7336,"source_line":617,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":415,"record_offset":41784,"parent_record_offset":41760,"ancestor_record_offset":24256,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":415,"record_offset":41760,"parent_record_offset":41668,"ancestor_record_offset":24256,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":415,"record_offset":41696,"parent_record_offset":41668,"ancestor_record_offset":24256,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":415,"record_offset":41668,"parent_record_offset":41644,"ancestor_record_offset":24256,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":415,"record_offset":40476,"parent_record_offset":40452,"ancestor_record_offset":24256,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":415,"record_offset":41572,"parent_record_offset":41548,"ancestor_record_offset":24256,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":415,"record_offset":41620,"parent_record_offset":41596,"ancestor_record_offset":24256,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":415,"record_offset":27164,"parent_record_offset":27136,"ancestor_record_offset":24256,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":415,"record_offset":35324,"parent_record_offset":35300,"ancestor_record_offset":24256,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":415,"record_offset":35348,"parent_record_offset":35324,"ancestor_record_offset":24256,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":415,"record_offset":35300,"parent_record_offset":35156,"ancestor_record_offset":24256,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::update_archetypes
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":415,"record_offset":35188,"parent_record_offset":35156,"ancestor_record_offset":24256,"inlinee":22612,"source_line":324,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":415,"record_offset":31768,"parent_record_offset":31744,"ancestor_record_offset":24256,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":415,"record_offset":31744,"parent_record_offset":31720,"ancestor_record_offset":24256,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":415,"record_offset":28608,"parent_record_offset":28576,"ancestor_record_offset":24256,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":415,"record_offset":28552,"parent_record_offset":28164,"ancestor_record_offset":24256,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":415,"record_offset":31636,"parent_record_offset":31248,"ancestor_record_offset":24256,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":415,"record_offset":28576,"parent_record_offset":28552,"ancestor_record_offset":24256,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":415,"record_offset":28192,"parent_record_offset":28164,"ancestor_record_offset":24256,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":415,"record_offset":31720,"parent_record_offset":31660,"ancestor_record_offset":24256,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":415,"record_offset":28240,"parent_record_offset":28216,"ancestor_record_offset":24256,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":415,"record_offset":28216,"parent_record_offset":28192,"ancestor_record_offset":24256,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":415,"record_offset":28332,"parent_record_offset":28268,"ancestor_record_offset":24256,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":415,"record_offset":28420,"parent_record_offset":28396,"ancestor_record_offset":24256,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::query_filtered
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":415,"record_offset":35124,"parent_record_offset":35100,"ancestor_record_offset":24256,"inlinee":22610,"source_line":1244,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":415,"record_offset":31836,"parent_record_offset":31812,"ancestor_record_offset":24256,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":415,"record_offset":31812,"parent_record_offset":31248,"ancestor_record_offset":24256,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":415,"record_offset":31996,"parent_record_offset":31964,"ancestor_record_offset":24256,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":415,"record_offset":28164,"parent_record_offset":27136,"ancestor_record_offset":24256,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":415,"record_offset":30236,"parent_record_offset":30204,"ancestor_record_offset":24256,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":415,"record_offset":31248,"parent_record_offset":30236,"ancestor_record_offset":24256,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":415,"record_offset":27896,"parent_record_offset":27872,"ancestor_record_offset":24256,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":415,"record_offset":26488,"parent_record_offset":26460,"ancestor_record_offset":24256,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":415,"record_offset":41204,"parent_record_offset":41176,"ancestor_record_offset":24256,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":415,"record_offset":26540,"parent_record_offset":26516,"ancestor_record_offset":24256,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":415,"record_offset":26516,"parent_record_offset":26488,"ancestor_record_offset":24256,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":415,"record_offset":26428,"parent_record_offset":26400,"ancestor_record_offset":24256,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":415,"record_offset":26460,"parent_record_offset":26428,"ancestor_record_offset":24256,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":415,"record_offset":26632,"parent_record_offset":26460,"ancestor_record_offset":24256,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":415,"record_offset":26744,"parent_record_offset":26720,"ancestor_record_offset":24256,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":415,"record_offset":27944,"parent_record_offset":27920,"ancestor_record_offset":24256,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":415,"record_offset":27872,"parent_record_offset":27848,"ancestor_record_offset":24256,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":415,"record_offset":40568,"parent_record_offset":40536,"ancestor_record_offset":24256,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":415,"record_offset":27360,"parent_record_offset":27336,"ancestor_record_offset":24256,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":415,"record_offset":27444,"parent_record_offset":27420,"ancestor_record_offset":24256,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":415,"record_offset":27384,"parent_record_offset":27360,"ancestor_record_offset":24256,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":415,"record_offset":40420,"parent_record_offset":40396,"ancestor_record_offset":24256,"inlinee":7701,"source_line":36,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":415,"record_offset":40500,"parent_record_offset":40476,"ancestor_record_offset":24256,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":415,"record_offset":26564,"parent_record_offset":26540,"ancestor_record_offset":24256,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":415,"record_offset":41724,"parent_record_offset":41696,"ancestor_record_offset":24256,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":415,"record_offset":24712,"parent_record_offset":24616,"ancestor_record_offset":24256,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":415,"record_offset":27556,"parent_record_offset":27532,"ancestor_record_offset":24256,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":415,"record_offset":26324,"parent_record_offset":26296,"ancestor_record_offset":24256,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":415,"record_offset":28132,"parent_record_offset":27164,"ancestor_record_offset":24256,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":415,"record_offset":25672,"parent_record_offset":25212,"ancestor_record_offset":24256,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":415,"record_offset":26372,"parent_record_offset":26196,"ancestor_record_offset":24256,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":415,"record_offset":31932,"parent_record_offset":30204,"ancestor_record_offset":24256,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":415,"record_offset":40296,"parent_record_offset":40272,"ancestor_record_offset":24256,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":415,"record_offset":25024,"parent_record_offset":24996,"ancestor_record_offset":24256,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":415,"record_offset":26696,"parent_record_offset":26400,"ancestor_record_offset":24256,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":415,"record_offset":27784,"parent_record_offset":27760,"ancestor_record_offset":24256,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":415,"record_offset":27704,"parent_record_offset":27680,"ancestor_record_offset":24256,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":415,"record_offset":25448,"parent_record_offset":25424,"ancestor_record_offset":24256,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":415,"record_offset":40784,"parent_record_offset":40716,"ancestor_record_offset":24256,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":415,"record_offset":25424,"parent_record_offset":25400,"ancestor_record_offset":24256,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":415,"record_offset":25632,"parent_record_offset":25600,"ancestor_record_offset":24256,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":415,"record_offset":40808,"parent_record_offset":40784,"ancestor_record_offset":24256,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":415,"record_offset":26588,"parent_record_offset":26564,"ancestor_record_offset":24256,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":415,"record_offset":27096,"parent_record_offset":26160,"ancestor_record_offset":24256,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":415,"record_offset":24648,"parent_record_offset":24616,"ancestor_record_offset":24256,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":415,"record_offset":24676,"parent_record_offset":24648,"ancestor_record_offset":24256,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":415,"record_offset":28476,"parent_record_offset":28444,"ancestor_record_offset":24256,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":415,"record_offset":25400,"parent_record_offset":25376,"ancestor_record_offset":24256,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":415,"record_offset":25728,"parent_record_offset":25704,"ancestor_record_offset":24256,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":415,"record_offset":25376,"parent_record_offset":25268,"ancestor_record_offset":24256,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":415,"record_offset":24616,"parent_record_offset":24588,"ancestor_record_offset":24256,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":415,"record_offset":25108,"parent_record_offset":25076,"ancestor_record_offset":24256,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":415,"record_offset":25240,"parent_record_offset":25212,"ancestor_record_offset":24256,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":415,"record_offset":25212,"parent_record_offset":25180,"ancestor_record_offset":24256,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":415,"record_offset":25156,"parent_record_offset":25132,"ancestor_record_offset":24256,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":415,"record_offset":25180,"parent_record_offset":25156,"ancestor_record_offset":24256,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":415,"record_offset":25132,"parent_record_offset":25108,"ancestor_record_offset":24256,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":415,"record_offset":25704,"parent_record_offset":25180,"ancestor_record_offset":24256,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":415,"record_offset":24812,"parent_record_offset":24780,"ancestor_record_offset":24256,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":415,"record_offset":24972,"parent_record_offset":24812,"ancestor_record_offset":24256,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":415,"record_offset":24780,"parent_record_offset":24756,"ancestor_record_offset":24256,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":415,"record_offset":24532,"parent_record_offset":24500,"ancestor_record_offset":24256,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":415,"record_offset":24996,"parent_record_offset":24972,"ancestor_record_offset":24256,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":415,"record_offset":24588,"parent_record_offset":24556,"ancestor_record_offset":24256,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":415,"record_offset":24556,"parent_record_offset":24532,"ancestor_record_offset":24256,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":415,"record_offset":25268,"parent_record_offset":25240,"ancestor_record_offset":24256,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":415,"record_offset":25516,"parent_record_offset":25268,"ancestor_record_offset":24256,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":415,"record_offset":25488,"parent_record_offset":25268,"ancestor_record_offset":24256,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":415,"record_offset":25320,"parent_record_offset":25268,"ancestor_record_offset":24256,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":415,"record_offset":31860,"parent_record_offset":31836,"ancestor_record_offset":24256,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":415,"record_offset":31884,"parent_record_offset":31860,"ancestor_record_offset":24256,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":415,"record_offset":27920,"parent_record_offset":27896,"ancestor_record_offset":24256,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":415,"record_offset":40884,"parent_record_offset":40844,"ancestor_record_offset":24256,"inlinee":21988,"source_line":137,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::ColorAttachment::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":415,"record_offset":40644,"parent_record_offset":40616,"ancestor_record_offset":24256,"inlinee":23122,"source_line":89,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":415,"record_offset":40616,"parent_record_offset":40592,"ancestor_record_offset":24256,"inlinee":6964,"source_line":145,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$5::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":415,"record_offset":40592,"parent_record_offset":40568,"ancestor_record_offset":24256,"inlinee":7000,"source_line":179,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/cursor_terrain_raycast.rs
//     evidence={"module":415,"record_offset":40272,"parent_record_offset":35420,"ancestor_record_offset":24256,"inlinee":22044,"source_line":28,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::camera_rig::MainCamera::eye_position
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig.rs
//     evidence={"module":415,"record_offset":40188,"parent_record_offset":35420,"ancestor_record_offset":24256,"inlinee":21914,"source_line":702,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/object_grass_push.rs
//     evidence={"module":415,"record_offset":35420,"parent_record_offset":35388,"ancestor_record_offset":24256,"inlinee":23729,"source_line":65,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":415,"record_offset":35476,"parent_record_offset":35420,"ancestor_record_offset":24256,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":415,"record_offset":40240,"parent_record_offset":40216,"ancestor_record_offset":24256,"inlinee":14703,"source_line":92,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::extend
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":415,"record_offset":40216,"parent_record_offset":35420,"ancestor_record_offset":24256,"inlinee":14702,"source_line":154,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":415,"record_offset":27848,"parent_record_offset":27824,"ancestor_record_offset":24256,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":415,"record_offset":27188,"parent_record_offset":27164,"ancestor_record_offset":24256,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":415,"record_offset":27212,"parent_record_offset":27188,"ancestor_record_offset":24256,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":415,"record_offset":27824,"parent_record_offset":27624,"ancestor_record_offset":24256,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":415,"record_offset":27736,"parent_record_offset":27652,"ancestor_record_offset":24256,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":415,"record_offset":27680,"parent_record_offset":27652,"ancestor_record_offset":24256,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":415,"record_offset":28080,"parent_record_offset":27288,"ancestor_record_offset":24256,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":415,"record_offset":27652,"parent_record_offset":27624,"ancestor_record_offset":24256,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":415,"record_offset":27264,"parent_record_offset":27240,"ancestor_record_offset":24256,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":415,"record_offset":27240,"parent_record_offset":27212,"ancestor_record_offset":24256,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":415,"record_offset":27288,"parent_record_offset":27264,"ancestor_record_offset":24256,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":415,"record_offset":27504,"parent_record_offset":27476,"ancestor_record_offset":24256,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":415,"record_offset":27532,"parent_record_offset":27504,"ancestor_record_offset":24256,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":415,"record_offset":27592,"parent_record_offset":27476,"ancestor_record_offset":24256,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":415,"record_offset":27476,"parent_record_offset":27288,"ancestor_record_offset":24256,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":415,"record_offset":27624,"parent_record_offset":27288,"ancestor_record_offset":24256,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":415,"record_offset":27336,"parent_record_offset":27288,"ancestor_record_offset":24256,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":415,"record_offset":27420,"parent_record_offset":27288,"ancestor_record_offset":24256,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":415,"record_offset":27996,"parent_record_offset":27288,"ancestor_record_offset":24256,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":415,"record_offset":41424,"parent_record_offset":41400,"ancestor_record_offset":24256,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":415,"record_offset":28444,"parent_record_offset":28420,"ancestor_record_offset":24256,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":415,"record_offset":26124,"parent_record_offset":26088,"ancestor_record_offset":24256,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":415,"record_offset":26236,"parent_record_offset":26196,"ancestor_record_offset":24256,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":415,"record_offset":26400,"parent_record_offset":26196,"ancestor_record_offset":24256,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":415,"record_offset":26668,"parent_record_offset":26400,"ancestor_record_offset":24256,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":415,"record_offset":41484,"parent_record_offset":41460,"ancestor_record_offset":24256,"inlinee":7341,"source_line":626,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":415,"record_offset":41400,"parent_record_offset":41376,"ancestor_record_offset":24256,"inlinee":7337,"source_line":617,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":415,"record_offset":40668,"parent_record_offset":40644,"ancestor_record_offset":24256,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":415,"record_offset":40912,"parent_record_offset":40884,"ancestor_record_offset":24256,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":415,"record_offset":40968,"parent_record_offset":40940,"ancestor_record_offset":24256,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":415,"record_offset":41056,"parent_record_offset":41024,"ancestor_record_offset":24256,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":415,"record_offset":41176,"parent_record_offset":41024,"ancestor_record_offset":24256,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":415,"record_offset":40940,"parent_record_offset":40912,"ancestor_record_offset":24256,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":415,"record_offset":26264,"parent_record_offset":26236,"ancestor_record_offset":24256,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":415,"record_offset":26296,"parent_record_offset":26264,"ancestor_record_offset":24256,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":415,"record_offset":26196,"parent_record_offset":26160,"ancestor_record_offset":24256,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":415,"record_offset":26160,"parent_record_offset":26124,"ancestor_record_offset":24256,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5CA7
// 首条位置: {"module":415,"inlinee":23719}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::extend
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1308,"record_offset":317024,"parent_record_offset":317000,"ancestor_record_offset":317000,"inlinee":14702,"source_line":154,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":1308,"record_offset":317048,"parent_record_offset":317024,"ancestor_record_offset":317000,"inlinee":14703,"source_line":92,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5CB1
// 首条位置: {"module":415,"inlinee":23729}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/cursor_terrain_raycast.rs
//     evidence={"module":415,"record_offset":40272,"parent_record_offset":35420,"ancestor_record_offset":35420,"inlinee":22044,"source_line":28,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::camera_rig::MainCamera::eye_position
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig.rs
//     evidence={"module":415,"record_offset":40188,"parent_record_offset":35420,"ancestor_record_offset":35420,"inlinee":21914,"source_line":702,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":415,"record_offset":35476,"parent_record_offset":35420,"ancestor_record_offset":35420,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::extend
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":415,"record_offset":40216,"parent_record_offset":35420,"ancestor_record_offset":35420,"inlinee":14702,"source_line":154,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":415,"record_offset":35536,"parent_record_offset":35508,"ancestor_record_offset":35420,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":415,"record_offset":36968,"parent_record_offset":36936,"ancestor_record_offset":35420,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":415,"record_offset":36912,"parent_record_offset":36524,"ancestor_record_offset":35420,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":415,"record_offset":36936,"parent_record_offset":36912,"ancestor_record_offset":35420,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":415,"record_offset":36552,"parent_record_offset":36524,"ancestor_record_offset":35420,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":415,"record_offset":36600,"parent_record_offset":36576,"ancestor_record_offset":35420,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":415,"record_offset":36576,"parent_record_offset":36552,"ancestor_record_offset":35420,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":415,"record_offset":36692,"parent_record_offset":36628,"ancestor_record_offset":35420,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":415,"record_offset":36780,"parent_record_offset":36756,"ancestor_record_offset":35420,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":415,"record_offset":35508,"parent_record_offset":35476,"ancestor_record_offset":35420,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":415,"record_offset":36524,"parent_record_offset":35508,"ancestor_record_offset":35420,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":415,"record_offset":36256,"parent_record_offset":36232,"ancestor_record_offset":35420,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":415,"record_offset":36304,"parent_record_offset":36280,"ancestor_record_offset":35420,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":415,"record_offset":36232,"parent_record_offset":36208,"ancestor_record_offset":35420,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":415,"record_offset":35724,"parent_record_offset":35700,"ancestor_record_offset":35420,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":415,"record_offset":35808,"parent_record_offset":35784,"ancestor_record_offset":35420,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":415,"record_offset":35748,"parent_record_offset":35724,"ancestor_record_offset":35420,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":415,"record_offset":35920,"parent_record_offset":35896,"ancestor_record_offset":35420,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":415,"record_offset":36716,"parent_record_offset":36692,"ancestor_record_offset":35420,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":415,"record_offset":36492,"parent_record_offset":35536,"ancestor_record_offset":35420,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":415,"record_offset":36756,"parent_record_offset":36576,"ancestor_record_offset":35420,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":415,"record_offset":37012,"parent_record_offset":35476,"ancestor_record_offset":35420,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":415,"record_offset":40296,"parent_record_offset":40272,"ancestor_record_offset":35420,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":415,"record_offset":36860,"parent_record_offset":36836,"ancestor_record_offset":35420,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":415,"record_offset":36144,"parent_record_offset":36120,"ancestor_record_offset":35420,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":415,"record_offset":36064,"parent_record_offset":36040,"ancestor_record_offset":35420,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":415,"record_offset":36628,"parent_record_offset":36600,"ancestor_record_offset":35420,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":415,"record_offset":36664,"parent_record_offset":36628,"ancestor_record_offset":35420,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":415,"record_offset":36836,"parent_record_offset":36804,"ancestor_record_offset":35420,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":415,"record_offset":36280,"parent_record_offset":36256,"ancestor_record_offset":35420,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":415,"record_offset":40240,"parent_record_offset":40216,"ancestor_record_offset":35420,"inlinee":14703,"source_line":92,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":415,"record_offset":36208,"parent_record_offset":36184,"ancestor_record_offset":35420,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":415,"record_offset":35560,"parent_record_offset":35536,"ancestor_record_offset":35420,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":415,"record_offset":35584,"parent_record_offset":35560,"ancestor_record_offset":35420,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":415,"record_offset":36184,"parent_record_offset":35988,"ancestor_record_offset":35420,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":415,"record_offset":36096,"parent_record_offset":36012,"ancestor_record_offset":35420,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":415,"record_offset":36040,"parent_record_offset":36012,"ancestor_record_offset":35420,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":415,"record_offset":36440,"parent_record_offset":35660,"ancestor_record_offset":35420,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":415,"record_offset":36012,"parent_record_offset":35988,"ancestor_record_offset":35420,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":415,"record_offset":35636,"parent_record_offset":35612,"ancestor_record_offset":35420,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":415,"record_offset":35612,"parent_record_offset":35584,"ancestor_record_offset":35420,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":415,"record_offset":35660,"parent_record_offset":35636,"ancestor_record_offset":35420,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":415,"record_offset":35868,"parent_record_offset":35840,"ancestor_record_offset":35420,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":415,"record_offset":35896,"parent_record_offset":35868,"ancestor_record_offset":35420,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":415,"record_offset":35956,"parent_record_offset":35840,"ancestor_record_offset":35420,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":415,"record_offset":35840,"parent_record_offset":35660,"ancestor_record_offset":35420,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":415,"record_offset":35988,"parent_record_offset":35660,"ancestor_record_offset":35420,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":415,"record_offset":35700,"parent_record_offset":35660,"ancestor_record_offset":35420,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":415,"record_offset":35784,"parent_record_offset":35660,"ancestor_record_offset":35420,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":415,"record_offset":36356,"parent_record_offset":35660,"ancestor_record_offset":35420,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":415,"record_offset":36804,"parent_record_offset":36780,"ancestor_record_offset":35420,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00415.jsonl.gz"}
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
// 完整结构及原始类型记录: object_grass_push.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: object_grass_push.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/object_grass_push.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: object_grass_push.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3138128,"position_among_mapped_render_targets":11,"mapped_render_target_count":70,"previous_callsite":3138106,"next_callsite":3138150,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/highlight_active_terrain_stroke.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/draw_path_mask.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: object_grass_push.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 7; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] array$<f32,3> (*)(glam::f32::sse2::vec3a::Vec3A)
// 候选参数类型: ["glam::f32::sse2::vec3a::Vec3A"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(ref$<glam::f32::vec2::Vec2>)
// 候选参数类型: ["ref$<glam::f32::vec2::Vec2>"]

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass
// 信息 ID: 6df62ddf3b1828d70bd9060e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fe250 -> 0x582b10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure$0
// 信息 ID: 0a4717f15ab5e947b5352070
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >,country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure_env$0>,core::option::IntoIter<glam::f32::sse2::vec4::Vec4> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >,country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure_env$0>,core::option::IntoIter<glam::f32::sse2::vec4::Vec4> > >; module=1308, inline_record=317000; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >,country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure_env$0>,core::option::IntoIter<glam::f32::sse2::vec4::Vec4> > >; module=1308, inline_record=320988; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure$1
// 信息 ID: d3ad50ab0243a408937a709c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass; module=415, inline_record=35420; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
