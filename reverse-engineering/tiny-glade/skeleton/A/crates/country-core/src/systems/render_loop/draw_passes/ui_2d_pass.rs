// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/draw_passes/ui_2d_pass.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 51fbd1eb2cc4b88d819fc1444e20f99492ff8f72bd91b95f58abab7b6b59dbe6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ui_2d_pass.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 482, 487, 1336
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
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":482,"record_offset":3056,"rva":6842640,"va":5375551760}
// 编译实例 RVAs: 0x686910
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140361320
//     owner=unresolved_namespace; count=2
//     target_aliases=228 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3300733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e883a2cdff","size":5,"soft_fail":false,"target_va":5372252960,"va":5375553688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=2
//     target_aliases=16 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2929034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e8764ed3ff","size":5,"soft_fail":false,"target_va":5372625024,"va":5375554053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2806250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e8162ed5ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375555957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140540a60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1338063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e83195ebff","size":5,"soft_fail":false,"target_va":5374216800,"va":5375554858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eb110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t406008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e8f8310600","size":5,"soft_fail":false,"target_va":5375963408,"va":5375557395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=10
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t411198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e83e460600","size":5,"soft_fail":false,"target_va":5375967552,"va":5375556349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072a6e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t668978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e832350a00","size":5,"soft_fail":false,"target_va":5376222944,"va":5375553961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d7f180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7307510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e8f6806f00","size":5,"soft_fail":false,"target_va":5382861184,"va":5375553669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ddb570
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7684697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e859427500","size":5,"soft_fail":false,"target_va":5383239024,"va":5375554322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7755926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e896587600","size":5,"soft_fail":false,"target_va":5383308144,"va":5375552213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df6180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7794070","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e896ed7600","size":5,"soft_fail":false,"target_va":5383348608,"va":5375554533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df6640
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7795256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e838f27600","size":5,"soft_fail":false,"target_va":5383349824,"va":5375554563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140eae1f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8548641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e821718200","size":5,"soft_fail":false,"target_va":5384102384,"va":5375553738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28039009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e861d7ab01","size":5,"soft_fail":false,"target_va":5403593360,"va":5375554346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::state::QueryState$LT$D$C$F$GT$::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8630354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e852b08300","size":5,"soft_fail":false,"target_va":5384183680,"va":5375553321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::state::QueryState$LT$D$C$F$GT$::update_archetypes_unsafe_world_cell
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8626800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e870a28300","size":5,"soft_fail":false,"target_va":5384180144,"va":5375553339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27422551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e8576fa201","size":5,"soft_fail":false,"target_va":5402978688,"va":5375556132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t27422813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e85d70a201","size":5,"soft_fail":false,"target_va":5402978880,"va":5375556062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e86beaffff","size":5,"soft_fail":false,"target_va":5375550496,"va":5375556016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27427861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e81584a201","size":5,"soft_fail":false,"target_va":5402980128,"va":5375552262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e870fbffff","size":5,"soft_fail":false,"target_va":5375550688,"va":5375551851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27428730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e87a87a201","size":5,"soft_fail":false,"target_va":5402980656,"va":5375551921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t139668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e894210200","size":5,"soft_fail":false,"target_va":5375692416,"va":5375552743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_no_vao
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t148493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e80d440200","size":5,"soft_fail":false,"target_va":5375704400,"va":5375555902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t146425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e8f93b0200","size":5,"soft_fail":false,"target_va":5375700688,"va":5375554258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-158322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e88e95fdff","size":5,"soft_fail":false,"target_va":5375395536,"va":5375553853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-157762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e8be97fdff","size":5,"soft_fail":false,"target_va":5375398240,"va":5375555997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t804478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e87e460c00","size":5,"soft_fail":false,"target_va":5376356912,"va":5375552429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t808362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e8aa550c00","size":5,"soft_fail":false,"target_va":5376360560,"va":5375552193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t804317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e8dd450c00","size":5,"soft_fail":false,"target_va":5376356144,"va":5375551822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t788001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e821060c00","size":5,"soft_fail":false,"target_va":5376341760,"va":5375553754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::base_layer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t787503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e82f040c00","size":5,"soft_fail":false,"target_va":5376342224,"va":5375554716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::layer_count
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t787533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e84d040c00","size":5,"soft_fail":false,"target_va":5376342288,"va":5375554750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::view_type
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t787142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e8c6020c00","size":5,"soft_fail":false,"target_va":5376341840,"va":5375554693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1812250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e81aa71b00","size":5,"soft_fail":false,"target_va":5377366224,"va":5375553969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::name_hash::NameHash::compute
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1391826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375551760,"raw_hex":"e8d23c1500","size":5,"soft_fail":false,"target_va":5376946416,"va":5375554585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":482,"record_offset":11088,"parent_record_offset":3056,"ancestor_record_offset":3056,"inlinee":6488,"source_line":263,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":482,"record_offset":11324,"parent_record_offset":3056,"ancestor_record_offset":3056,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":482,"record_offset":11380,"parent_record_offset":3056,"ancestor_record_offset":3056,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::state::QueryState::iter
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":482,"record_offset":10640,"parent_record_offset":3056,"ancestor_record_offset":3056,"inlinee":22611,"source_line":972,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::query
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":482,"record_offset":10584,"parent_record_offset":3056,"ancestor_record_offset":3056,"inlinee":22609,"source_line":1220,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":482,"record_offset":5936,"parent_record_offset":3056,"ancestor_record_offset":3056,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":482,"record_offset":11004,"parent_record_offset":3056,"ancestor_record_offset":3056,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":482,"record_offset":13092,"parent_record_offset":3056,"ancestor_record_offset":3056,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":482,"record_offset":13120,"parent_record_offset":3056,"ancestor_record_offset":3056,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":482,"record_offset":7452,"parent_record_offset":3056,"ancestor_record_offset":3056,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":482,"record_offset":13396,"parent_record_offset":3056,"ancestor_record_offset":3056,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":482,"record_offset":13180,"parent_record_offset":3056,"ancestor_record_offset":3056,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture_view
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":482,"record_offset":12832,"parent_record_offset":3056,"ancestor_record_offset":3056,"inlinee":27254,"source_line":497,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":482,"record_offset":12588,"parent_record_offset":3056,"ancestor_record_offset":3056,"inlinee":23127,"source_line":461,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":482,"record_offset":11452,"parent_record_offset":3056,"ancestor_record_offset":3056,"inlinee":21986,"source_line":105,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":482,"record_offset":11144,"parent_record_offset":3056,"ancestor_record_offset":3056,"inlinee":7327,"source_line":314,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::window::WindowSize::as_vec2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/window.rs
//     evidence={"module":482,"record_offset":10556,"parent_record_offset":3056,"ancestor_record_offset":3056,"inlinee":21913,"source_line":35,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":482,"record_offset":4888,"parent_record_offset":3056,"ancestor_record_offset":3056,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":482,"record_offset":3300,"parent_record_offset":3056,"ancestor_record_offset":3056,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":482,"record_offset":3876,"parent_record_offset":3056,"ancestor_record_offset":3056,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":482,"record_offset":13308,"parent_record_offset":3056,"ancestor_record_offset":3056,"inlinee":7340,"source_line":626,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":482,"record_offset":13224,"parent_record_offset":3056,"ancestor_record_offset":3056,"inlinee":7336,"source_line":617,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::impl$40::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":482,"record_offset":12804,"parent_record_offset":3056,"ancestor_record_offset":3056,"inlinee":26868,"source_line":251,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":482,"record_offset":13632,"parent_record_offset":13608,"ancestor_record_offset":3056,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":482,"record_offset":13608,"parent_record_offset":13516,"ancestor_record_offset":3056,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":482,"record_offset":13544,"parent_record_offset":13516,"ancestor_record_offset":3056,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":482,"record_offset":13516,"parent_record_offset":13492,"ancestor_record_offset":3056,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":482,"record_offset":11112,"parent_record_offset":11088,"ancestor_record_offset":3056,"inlinee":6490,"source_line":842,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":482,"record_offset":11028,"parent_record_offset":11004,"ancestor_record_offset":3056,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":482,"record_offset":13420,"parent_record_offset":13396,"ancestor_record_offset":3056,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":482,"record_offset":13468,"parent_record_offset":13444,"ancestor_record_offset":3056,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":482,"record_offset":5964,"parent_record_offset":5936,"ancestor_record_offset":3056,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":482,"record_offset":10808,"parent_record_offset":10784,"ancestor_record_offset":3056,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":482,"record_offset":10844,"parent_record_offset":10808,"ancestor_record_offset":3056,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":482,"record_offset":10784,"parent_record_offset":10640,"ancestor_record_offset":3056,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::update_archetypes
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":482,"record_offset":10672,"parent_record_offset":10640,"ancestor_record_offset":3056,"inlinee":22612,"source_line":324,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$6::init_fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":482,"record_offset":10876,"parent_record_offset":10844,"ancestor_record_offset":3056,"inlinee":27253,"source_line":170,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":482,"record_offset":7408,"parent_record_offset":7376,"ancestor_record_offset":3056,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":482,"record_offset":7352,"parent_record_offset":6964,"ancestor_record_offset":3056,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":482,"record_offset":7376,"parent_record_offset":7352,"ancestor_record_offset":3056,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":482,"record_offset":6992,"parent_record_offset":6964,"ancestor_record_offset":3056,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":482,"record_offset":7040,"parent_record_offset":7016,"ancestor_record_offset":3056,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":482,"record_offset":7016,"parent_record_offset":6992,"ancestor_record_offset":3056,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":482,"record_offset":7132,"parent_record_offset":7068,"ancestor_record_offset":3056,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":482,"record_offset":7220,"parent_record_offset":7196,"ancestor_record_offset":3056,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::query_filtered
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":482,"record_offset":10608,"parent_record_offset":10584,"ancestor_record_offset":3056,"inlinee":22610,"source_line":1244,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":482,"record_offset":10700,"parent_record_offset":10640,"ancestor_record_offset":3056,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":482,"record_offset":6964,"parent_record_offset":5936,"ancestor_record_offset":3056,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":482,"record_offset":6696,"parent_record_offset":6672,"ancestor_record_offset":3056,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":482,"record_offset":5288,"parent_record_offset":5260,"ancestor_record_offset":3056,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":482,"record_offset":11848,"parent_record_offset":11820,"ancestor_record_offset":3056,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":482,"record_offset":5340,"parent_record_offset":5316,"ancestor_record_offset":3056,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":482,"record_offset":5316,"parent_record_offset":5288,"ancestor_record_offset":3056,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":482,"record_offset":5228,"parent_record_offset":5200,"ancestor_record_offset":3056,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":482,"record_offset":5260,"parent_record_offset":5228,"ancestor_record_offset":3056,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":482,"record_offset":5432,"parent_record_offset":5260,"ancestor_record_offset":3056,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":482,"record_offset":5544,"parent_record_offset":5520,"ancestor_record_offset":3056,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":482,"record_offset":6744,"parent_record_offset":6720,"ancestor_record_offset":3056,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":482,"record_offset":6672,"parent_record_offset":6648,"ancestor_record_offset":3056,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":482,"record_offset":11176,"parent_record_offset":11144,"ancestor_record_offset":3056,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":482,"record_offset":6160,"parent_record_offset":6136,"ancestor_record_offset":3056,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":482,"record_offset":6244,"parent_record_offset":6220,"ancestor_record_offset":3056,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":482,"record_offset":6184,"parent_record_offset":6160,"ancestor_record_offset":3056,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":482,"record_offset":11052,"parent_record_offset":11028,"ancestor_record_offset":3056,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":482,"record_offset":5364,"parent_record_offset":5340,"ancestor_record_offset":3056,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":482,"record_offset":13572,"parent_record_offset":13544,"ancestor_record_offset":3056,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":482,"record_offset":3512,"parent_record_offset":3416,"ancestor_record_offset":3056,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":482,"record_offset":6356,"parent_record_offset":6332,"ancestor_record_offset":3056,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":482,"record_offset":12628,"parent_record_offset":12588,"ancestor_record_offset":3056,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":482,"record_offset":5124,"parent_record_offset":5096,"ancestor_record_offset":3056,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":482,"record_offset":6932,"parent_record_offset":5964,"ancestor_record_offset":3056,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":482,"record_offset":4472,"parent_record_offset":4012,"ancestor_record_offset":3056,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":482,"record_offset":5172,"parent_record_offset":4996,"ancestor_record_offset":3056,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":482,"record_offset":3824,"parent_record_offset":3796,"ancestor_record_offset":3056,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":482,"record_offset":5496,"parent_record_offset":5200,"ancestor_record_offset":3056,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":482,"record_offset":10952,"parent_record_offset":10928,"ancestor_record_offset":3056,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":482,"record_offset":6584,"parent_record_offset":6560,"ancestor_record_offset":3056,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":482,"record_offset":6504,"parent_record_offset":6480,"ancestor_record_offset":3056,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":482,"record_offset":4248,"parent_record_offset":4224,"ancestor_record_offset":3056,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":482,"record_offset":4224,"parent_record_offset":4200,"ancestor_record_offset":3056,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":482,"record_offset":4432,"parent_record_offset":4400,"ancestor_record_offset":3056,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":482,"record_offset":5388,"parent_record_offset":5364,"ancestor_record_offset":3056,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":482,"record_offset":5896,"parent_record_offset":4960,"ancestor_record_offset":3056,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":482,"record_offset":3448,"parent_record_offset":3416,"ancestor_record_offset":3056,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":482,"record_offset":10904,"parent_record_offset":10844,"ancestor_record_offset":3056,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":482,"record_offset":3476,"parent_record_offset":3448,"ancestor_record_offset":3056,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":482,"record_offset":7276,"parent_record_offset":7244,"ancestor_record_offset":3056,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":482,"record_offset":10928,"parent_record_offset":10904,"ancestor_record_offset":3056,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":482,"record_offset":4200,"parent_record_offset":4176,"ancestor_record_offset":3056,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":482,"record_offset":4528,"parent_record_offset":4504,"ancestor_record_offset":3056,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":482,"record_offset":4176,"parent_record_offset":4068,"ancestor_record_offset":3056,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":482,"record_offset":3416,"parent_record_offset":3388,"ancestor_record_offset":3056,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":482,"record_offset":3908,"parent_record_offset":3876,"ancestor_record_offset":3056,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":482,"record_offset":4040,"parent_record_offset":4012,"ancestor_record_offset":3056,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":482,"record_offset":4012,"parent_record_offset":3980,"ancestor_record_offset":3056,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":482,"record_offset":3956,"parent_record_offset":3932,"ancestor_record_offset":3056,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":482,"record_offset":3980,"parent_record_offset":3956,"ancestor_record_offset":3056,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":482,"record_offset":3932,"parent_record_offset":3908,"ancestor_record_offset":3056,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":482,"record_offset":4504,"parent_record_offset":3980,"ancestor_record_offset":3056,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":482,"record_offset":3612,"parent_record_offset":3580,"ancestor_record_offset":3056,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":482,"record_offset":3772,"parent_record_offset":3612,"ancestor_record_offset":3056,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":482,"record_offset":3580,"parent_record_offset":3556,"ancestor_record_offset":3056,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":482,"record_offset":3332,"parent_record_offset":3300,"ancestor_record_offset":3056,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":482,"record_offset":3796,"parent_record_offset":3772,"ancestor_record_offset":3056,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":482,"record_offset":3388,"parent_record_offset":3356,"ancestor_record_offset":3056,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":482,"record_offset":3356,"parent_record_offset":3332,"ancestor_record_offset":3056,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":482,"record_offset":4068,"parent_record_offset":4040,"ancestor_record_offset":3056,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":482,"record_offset":4316,"parent_record_offset":4068,"ancestor_record_offset":3056,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":482,"record_offset":4288,"parent_record_offset":4068,"ancestor_record_offset":3056,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":482,"record_offset":4120,"parent_record_offset":4068,"ancestor_record_offset":3056,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":482,"record_offset":10724,"parent_record_offset":10700,"ancestor_record_offset":3056,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":482,"record_offset":10748,"parent_record_offset":10724,"ancestor_record_offset":3056,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":482,"record_offset":6720,"parent_record_offset":6696,"ancestor_record_offset":3056,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":482,"record_offset":11496,"parent_record_offset":11452,"ancestor_record_offset":3056,"inlinee":21988,"source_line":137,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::ColorAttachment::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":482,"record_offset":11252,"parent_record_offset":11224,"ancestor_record_offset":3056,"inlinee":23122,"source_line":89,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":482,"record_offset":11224,"parent_record_offset":11200,"ancestor_record_offset":3056,"inlinee":6964,"source_line":145,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$5::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":482,"record_offset":11200,"parent_record_offset":11176,"ancestor_record_offset":3056,"inlinee":7000,"source_line":179,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/ui_2d_pass.rs
//     evidence={"module":482,"record_offset":13148,"parent_record_offset":13120,"ancestor_record_offset":3056,"inlinee":27256,"source_line":110,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":482,"record_offset":6648,"parent_record_offset":6624,"ancestor_record_offset":3056,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":482,"record_offset":5988,"parent_record_offset":5964,"ancestor_record_offset":3056,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":482,"record_offset":6012,"parent_record_offset":5988,"ancestor_record_offset":3056,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":482,"record_offset":6624,"parent_record_offset":6424,"ancestor_record_offset":3056,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":482,"record_offset":6536,"parent_record_offset":6452,"ancestor_record_offset":3056,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":482,"record_offset":6480,"parent_record_offset":6452,"ancestor_record_offset":3056,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":482,"record_offset":6880,"parent_record_offset":6088,"ancestor_record_offset":3056,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":482,"record_offset":6452,"parent_record_offset":6424,"ancestor_record_offset":3056,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":482,"record_offset":6064,"parent_record_offset":6040,"ancestor_record_offset":3056,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":482,"record_offset":6040,"parent_record_offset":6012,"ancestor_record_offset":3056,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":482,"record_offset":6088,"parent_record_offset":6064,"ancestor_record_offset":3056,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":482,"record_offset":6304,"parent_record_offset":6276,"ancestor_record_offset":3056,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":482,"record_offset":6332,"parent_record_offset":6304,"ancestor_record_offset":3056,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":482,"record_offset":6392,"parent_record_offset":6276,"ancestor_record_offset":3056,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":482,"record_offset":6276,"parent_record_offset":6088,"ancestor_record_offset":3056,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":482,"record_offset":6424,"parent_record_offset":6088,"ancestor_record_offset":3056,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":482,"record_offset":6136,"parent_record_offset":6088,"ancestor_record_offset":3056,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":482,"record_offset":6220,"parent_record_offset":6088,"ancestor_record_offset":3056,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":482,"record_offset":6796,"parent_record_offset":6088,"ancestor_record_offset":3056,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":482,"record_offset":13272,"parent_record_offset":13248,"ancestor_record_offset":3056,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":482,"record_offset":7244,"parent_record_offset":7220,"ancestor_record_offset":3056,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":482,"record_offset":4924,"parent_record_offset":4888,"ancestor_record_offset":3056,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":482,"record_offset":5036,"parent_record_offset":4996,"ancestor_record_offset":3056,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":482,"record_offset":5200,"parent_record_offset":4996,"ancestor_record_offset":3056,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":482,"record_offset":5468,"parent_record_offset":5200,"ancestor_record_offset":3056,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":482,"record_offset":13332,"parent_record_offset":13308,"ancestor_record_offset":3056,"inlinee":7341,"source_line":626,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":482,"record_offset":13248,"parent_record_offset":13224,"ancestor_record_offset":3056,"inlinee":7337,"source_line":617,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":482,"record_offset":11276,"parent_record_offset":11252,"ancestor_record_offset":3056,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":482,"record_offset":11528,"parent_record_offset":11496,"ancestor_record_offset":3056,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":482,"record_offset":11592,"parent_record_offset":11560,"ancestor_record_offset":3056,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":482,"record_offset":11692,"parent_record_offset":11656,"ancestor_record_offset":3056,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":482,"record_offset":11820,"parent_record_offset":11656,"ancestor_record_offset":3056,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":482,"record_offset":11560,"parent_record_offset":11528,"ancestor_record_offset":3056,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":482,"record_offset":5064,"parent_record_offset":5036,"ancestor_record_offset":3056,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":482,"record_offset":5096,"parent_record_offset":5064,"ancestor_record_offset":3056,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":482,"record_offset":4996,"parent_record_offset":4960,"ancestor_record_offset":3056,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":482,"record_offset":4960,"parent_record_offset":4924,"ancestor_record_offset":3056,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF338
// 首条位置: {"module":1336,"inlinee":62264}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$55::cmp
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1336,"record_offset":89736,"parent_record_offset":89632,"ancestor_record_offset":89632,"inlinee":7631,"source_line":1565,"archive":"_pdb/A/modules/01336.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1336,"record_offset":89668,"parent_record_offset":89632,"ancestor_record_offset":89632,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/01336.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 110
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6A78
// 首条位置: {"module":482,"inlinee":27256}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 154
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":482,"record_offset":14288,"rva":6848592,"va":5375557712}
// 编译实例 RVAs: 0x688050
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2934739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375557712,"raw_hex":"e82d38d3ff","size":5,"soft_fail":false,"target_va":5372625024,"va":5375559758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140540a60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1343319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375557712,"raw_hex":"e8a980ebff","size":5,"soft_fail":false,"target_va":5374216800,"va":5375560114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140589d00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1042886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375557712,"raw_hex":"e83a16f0ff","size":5,"soft_fail":false,"target_va":5374516480,"va":5375559361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140644950
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-278712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375557712,"raw_hex":"e848bffbff","size":5,"soft_fail":false,"target_va":5375281488,"va":5375560195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406a1ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t101891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375557712,"raw_hex":"e8038e0100","size":5,"soft_fail":false,"target_va":5375662768,"va":5375560872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eb110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t401857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375557712,"raw_hex":"e8c1210600","size":5,"soft_fail":false,"target_va":5375963408,"va":5375561546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t406432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375557712,"raw_hex":"e8a0330600","size":5,"soft_fail":false,"target_va":5375967552,"va":5375561115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072a6e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t663218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375557712,"raw_hex":"e8b21e0a00","size":5,"soft_fail":false,"target_va":5376222944,"va":5375559721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ddb570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7678996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375557712,"raw_hex":"e8142c7500","size":5,"soft_fail":false,"target_va":5383239024,"va":5375560023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7749927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375557712,"raw_hex":"e827417600","size":5,"soft_fail":false,"target_va":5383308144,"va":5375558212,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df7fa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7796060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375557712,"raw_hex":"e85cf57600","size":5,"soft_fail":false,"target_va":5383356320,"va":5375560255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dfc040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7812568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375557712,"raw_hex":"e8d8357700","size":5,"soft_fail":false,"target_va":5383372864,"va":5375560291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28033301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375557712,"raw_hex":"e815c1ab01","size":5,"soft_fail":false,"target_va":5403593360,"va":5375560054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27417661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375557712,"raw_hex":"e83d5ca201","size":5,"soft_fail":false,"target_va":5402978688,"va":5375561022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t27417930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375557712,"raw_hex":"e84a5da201","size":5,"soft_fail":false,"target_va":5402978880,"va":5375560945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375557712,"raw_hex":"e860d7ffff","size":5,"soft_fail":false,"target_va":5375550496,"va":5375560891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27421859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375557712,"raw_hex":"e8a36ca201","size":5,"soft_fail":false,"target_va":5402980128,"va":5375558264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-7153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375557712,"raw_hex":"e80fe4ffff","size":5,"soft_fail":false,"target_va":5375550688,"va":5375557836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27422745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375557712,"raw_hex":"e81970a201","size":5,"soft_fail":false,"target_va":5402980656,"va":5375557906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t133668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375557712,"raw_hex":"e8240a0200","size":5,"soft_fail":false,"target_va":5375692416,"va":5375558743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_no_vao
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t144030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375557712,"raw_hex":"e89e320200","size":5,"soft_fail":false,"target_va":5375704400,"va":5375560365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t140713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375557712,"raw_hex":"e8a9250200","size":5,"soft_fail":false,"target_va":5375700688,"va":5375559970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-164178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375557712,"raw_hex":"e8ae7efdff","size":5,"soft_fail":false,"target_va":5375395536,"va":5375559709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-162163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375557712,"raw_hex":"e88d86fdff","size":5,"soft_fail":false,"target_va":5375398240,"va":5375560398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t798477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375557712,"raw_hex":"e80d2f0c00","size":5,"soft_fail":false,"target_va":5376356912,"va":5375558430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t802363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375557712,"raw_hex":"e83b3e0c00","size":5,"soft_fail":false,"target_va":5376360560,"va":5375558192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t798346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375557712,"raw_hex":"e88a2e0c00","size":5,"soft_fail":false,"target_va":5376356144,"va":5375557793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t782464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375557712,"raw_hex":"e880f00b00","size":5,"soft_fail":false,"target_va":5376341760,"va":5375559291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t781195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375557712,"raw_hex":"e88beb0b00","size":5,"soft_fail":false,"target_va":5376340512,"va":5375559312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t660146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375557712,"raw_hex":"e8b2120a00","size":5,"soft_fail":false,"target_va":5376219552,"va":5375559401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1806490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375557712,"raw_hex":"e89a901b00","size":5,"soft_fail":false,"target_va":5377366224,"va":5375559729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextRenderer::render
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t478579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375557712,"raw_hex":"e8734d0700","size":5,"soft_fail":false,"target_va":5376037840,"va":5375559256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":482,"record_offset":17172,"parent_record_offset":14288,"ancestor_record_offset":14288,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":482,"record_offset":18688,"parent_record_offset":14288,"ancestor_record_offset":14288,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":482,"record_offset":22712,"parent_record_offset":14288,"ancestor_record_offset":14288,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":482,"record_offset":22640,"parent_record_offset":14288,"ancestor_record_offset":14288,"inlinee":23127,"source_line":461,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":482,"record_offset":22108,"parent_record_offset":14288,"ancestor_record_offset":14288,"inlinee":21986,"source_line":105,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":482,"record_offset":21904,"parent_record_offset":14288,"ancestor_record_offset":14288,"inlinee":7327,"source_line":314,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::window::WindowSize::as_vec2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/window.rs
//     evidence={"module":482,"record_offset":20240,"parent_record_offset":14288,"ancestor_record_offset":14288,"inlinee":21913,"source_line":35,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":482,"record_offset":16104,"parent_record_offset":14288,"ancestor_record_offset":14288,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":482,"record_offset":14496,"parent_record_offset":14288,"ancestor_record_offset":14288,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":482,"record_offset":15084,"parent_record_offset":14288,"ancestor_record_offset":14288,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":482,"record_offset":22884,"parent_record_offset":14288,"ancestor_record_offset":14288,"inlinee":7340,"source_line":626,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":482,"record_offset":22800,"parent_record_offset":14288,"ancestor_record_offset":14288,"inlinee":7336,"source_line":617,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":482,"record_offset":21804,"parent_record_offset":14288,"ancestor_record_offset":14288,"inlinee":9081,"source_line":40,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":482,"record_offset":21776,"parent_record_offset":14288,"ancestor_record_offset":14288,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":482,"record_offset":17200,"parent_record_offset":17172,"ancestor_record_offset":14288,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":482,"record_offset":18644,"parent_record_offset":18612,"ancestor_record_offset":14288,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":482,"record_offset":18588,"parent_record_offset":18200,"ancestor_record_offset":14288,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":482,"record_offset":18612,"parent_record_offset":18588,"ancestor_record_offset":14288,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":482,"record_offset":18228,"parent_record_offset":18200,"ancestor_record_offset":14288,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":482,"record_offset":18276,"parent_record_offset":18252,"ancestor_record_offset":14288,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":482,"record_offset":18252,"parent_record_offset":18228,"ancestor_record_offset":14288,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":482,"record_offset":18368,"parent_record_offset":18304,"ancestor_record_offset":14288,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":482,"record_offset":18456,"parent_record_offset":18432,"ancestor_record_offset":14288,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":482,"record_offset":18200,"parent_record_offset":17172,"ancestor_record_offset":14288,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":482,"record_offset":17932,"parent_record_offset":17908,"ancestor_record_offset":14288,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":482,"record_offset":16524,"parent_record_offset":16496,"ancestor_record_offset":14288,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":482,"record_offset":22468,"parent_record_offset":22440,"ancestor_record_offset":14288,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":482,"record_offset":16576,"parent_record_offset":16552,"ancestor_record_offset":14288,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":482,"record_offset":16552,"parent_record_offset":16524,"ancestor_record_offset":14288,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":482,"record_offset":16464,"parent_record_offset":16432,"ancestor_record_offset":14288,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":482,"record_offset":16496,"parent_record_offset":16464,"ancestor_record_offset":14288,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":482,"record_offset":16668,"parent_record_offset":16496,"ancestor_record_offset":14288,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":482,"record_offset":16780,"parent_record_offset":16756,"ancestor_record_offset":14288,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":482,"record_offset":17980,"parent_record_offset":17956,"ancestor_record_offset":14288,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":482,"record_offset":17908,"parent_record_offset":17884,"ancestor_record_offset":14288,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":482,"record_offset":21872,"parent_record_offset":21804,"ancestor_record_offset":14288,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":482,"record_offset":17396,"parent_record_offset":17372,"ancestor_record_offset":14288,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":482,"record_offset":17480,"parent_record_offset":17456,"ancestor_record_offset":14288,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":482,"record_offset":17420,"parent_record_offset":17396,"ancestor_record_offset":14288,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":482,"record_offset":16600,"parent_record_offset":16576,"ancestor_record_offset":14288,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":482,"record_offset":14712,"parent_record_offset":14616,"ancestor_record_offset":14288,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":482,"record_offset":17592,"parent_record_offset":17568,"ancestor_record_offset":14288,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":482,"record_offset":22680,"parent_record_offset":22640,"ancestor_record_offset":14288,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":482,"record_offset":16356,"parent_record_offset":16328,"ancestor_record_offset":14288,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":482,"record_offset":18168,"parent_record_offset":17200,"ancestor_record_offset":14288,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":482,"record_offset":15684,"parent_record_offset":15224,"ancestor_record_offset":14288,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":482,"record_offset":16404,"parent_record_offset":16224,"ancestor_record_offset":14288,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":482,"record_offset":15032,"parent_record_offset":15004,"ancestor_record_offset":14288,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":482,"record_offset":16732,"parent_record_offset":16432,"ancestor_record_offset":14288,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":482,"record_offset":17820,"parent_record_offset":17796,"ancestor_record_offset":14288,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":482,"record_offset":17740,"parent_record_offset":17716,"ancestor_record_offset":14288,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":482,"record_offset":15460,"parent_record_offset":15436,"ancestor_record_offset":14288,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":482,"record_offset":15436,"parent_record_offset":15412,"ancestor_record_offset":14288,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":482,"record_offset":15644,"parent_record_offset":15612,"ancestor_record_offset":14288,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":482,"record_offset":16624,"parent_record_offset":16600,"ancestor_record_offset":14288,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":482,"record_offset":17132,"parent_record_offset":16184,"ancestor_record_offset":14288,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":482,"record_offset":14648,"parent_record_offset":14616,"ancestor_record_offset":14288,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":482,"record_offset":14676,"parent_record_offset":14648,"ancestor_record_offset":14288,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":482,"record_offset":18512,"parent_record_offset":18480,"ancestor_record_offset":14288,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":482,"record_offset":15412,"parent_record_offset":15388,"ancestor_record_offset":14288,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":482,"record_offset":15740,"parent_record_offset":15716,"ancestor_record_offset":14288,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":482,"record_offset":15388,"parent_record_offset":15280,"ancestor_record_offset":14288,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":482,"record_offset":14616,"parent_record_offset":14588,"ancestor_record_offset":14288,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":482,"record_offset":15120,"parent_record_offset":15084,"ancestor_record_offset":14288,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":482,"record_offset":15252,"parent_record_offset":15224,"ancestor_record_offset":14288,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":482,"record_offset":15224,"parent_record_offset":15192,"ancestor_record_offset":14288,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":482,"record_offset":15168,"parent_record_offset":15144,"ancestor_record_offset":14288,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":482,"record_offset":15192,"parent_record_offset":15168,"ancestor_record_offset":14288,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":482,"record_offset":15144,"parent_record_offset":15120,"ancestor_record_offset":14288,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":482,"record_offset":15716,"parent_record_offset":15192,"ancestor_record_offset":14288,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":482,"record_offset":14820,"parent_record_offset":14784,"ancestor_record_offset":14288,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":482,"record_offset":14980,"parent_record_offset":14820,"ancestor_record_offset":14288,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":482,"record_offset":14784,"parent_record_offset":14756,"ancestor_record_offset":14288,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":482,"record_offset":14532,"parent_record_offset":14496,"ancestor_record_offset":14288,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":482,"record_offset":15004,"parent_record_offset":14980,"ancestor_record_offset":14288,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":482,"record_offset":14588,"parent_record_offset":14556,"ancestor_record_offset":14288,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":482,"record_offset":14556,"parent_record_offset":14532,"ancestor_record_offset":14288,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":482,"record_offset":15280,"parent_record_offset":15252,"ancestor_record_offset":14288,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":482,"record_offset":15528,"parent_record_offset":15280,"ancestor_record_offset":14288,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":482,"record_offset":15500,"parent_record_offset":15280,"ancestor_record_offset":14288,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":482,"record_offset":15332,"parent_record_offset":15280,"ancestor_record_offset":14288,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":482,"record_offset":17956,"parent_record_offset":17932,"ancestor_record_offset":14288,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":482,"record_offset":22148,"parent_record_offset":22108,"ancestor_record_offset":14288,"inlinee":21988,"source_line":137,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::ColorAttachment::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":482,"record_offset":22028,"parent_record_offset":21996,"ancestor_record_offset":14288,"inlinee":23122,"source_line":89,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":482,"record_offset":21996,"parent_record_offset":21968,"ancestor_record_offset":14288,"inlinee":6964,"source_line":145,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$5::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":482,"record_offset":21968,"parent_record_offset":21940,"ancestor_record_offset":14288,"inlinee":7000,"source_line":179,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":482,"record_offset":17884,"parent_record_offset":17860,"ancestor_record_offset":14288,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":482,"record_offset":17224,"parent_record_offset":17200,"ancestor_record_offset":14288,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":482,"record_offset":17248,"parent_record_offset":17224,"ancestor_record_offset":14288,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":482,"record_offset":17860,"parent_record_offset":17660,"ancestor_record_offset":14288,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":482,"record_offset":17772,"parent_record_offset":17688,"ancestor_record_offset":14288,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":482,"record_offset":17716,"parent_record_offset":17688,"ancestor_record_offset":14288,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":482,"record_offset":18116,"parent_record_offset":17324,"ancestor_record_offset":14288,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":482,"record_offset":17688,"parent_record_offset":17660,"ancestor_record_offset":14288,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":482,"record_offset":17300,"parent_record_offset":17276,"ancestor_record_offset":14288,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":482,"record_offset":17276,"parent_record_offset":17248,"ancestor_record_offset":14288,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":482,"record_offset":17324,"parent_record_offset":17300,"ancestor_record_offset":14288,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":482,"record_offset":17540,"parent_record_offset":17512,"ancestor_record_offset":14288,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":482,"record_offset":17568,"parent_record_offset":17540,"ancestor_record_offset":14288,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":482,"record_offset":17628,"parent_record_offset":17512,"ancestor_record_offset":14288,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":482,"record_offset":17512,"parent_record_offset":17324,"ancestor_record_offset":14288,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":482,"record_offset":17660,"parent_record_offset":17324,"ancestor_record_offset":14288,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":482,"record_offset":17372,"parent_record_offset":17324,"ancestor_record_offset":14288,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":482,"record_offset":17456,"parent_record_offset":17324,"ancestor_record_offset":14288,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":482,"record_offset":18032,"parent_record_offset":17324,"ancestor_record_offset":14288,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":482,"record_offset":22848,"parent_record_offset":22824,"ancestor_record_offset":14288,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":482,"record_offset":18480,"parent_record_offset":18456,"ancestor_record_offset":14288,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":482,"record_offset":16144,"parent_record_offset":16104,"ancestor_record_offset":14288,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":482,"record_offset":16268,"parent_record_offset":16224,"ancestor_record_offset":14288,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":482,"record_offset":16432,"parent_record_offset":16224,"ancestor_record_offset":14288,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":482,"record_offset":16704,"parent_record_offset":16432,"ancestor_record_offset":14288,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":482,"record_offset":22908,"parent_record_offset":22884,"ancestor_record_offset":14288,"inlinee":7341,"source_line":626,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":482,"record_offset":22824,"parent_record_offset":22800,"ancestor_record_offset":14288,"inlinee":7337,"source_line":617,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":482,"record_offset":21844,"parent_record_offset":21804,"ancestor_record_offset":14288,"inlinee":9072,"source_line":67,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":482,"record_offset":22060,"parent_record_offset":22028,"ancestor_record_offset":14288,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":482,"record_offset":22176,"parent_record_offset":22148,"ancestor_record_offset":14288,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":482,"record_offset":22232,"parent_record_offset":22204,"ancestor_record_offset":14288,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":482,"record_offset":22320,"parent_record_offset":22288,"ancestor_record_offset":14288,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":482,"record_offset":22440,"parent_record_offset":22288,"ancestor_record_offset":14288,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":482,"record_offset":22204,"parent_record_offset":22176,"ancestor_record_offset":14288,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":482,"record_offset":16296,"parent_record_offset":16268,"ancestor_record_offset":14288,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":482,"record_offset":16328,"parent_record_offset":16296,"ancestor_record_offset":14288,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":482,"record_offset":16224,"parent_record_offset":16184,"ancestor_record_offset":14288,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":482,"record_offset":16184,"parent_record_offset":16144,"ancestor_record_offset":14288,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00482.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 227
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B27
// 首条位置: {"module":487,"inlinee":27431}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":487,"record_offset":24868,"parent_record_offset":24816,"ancestor_record_offset":24816,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":487,"record_offset":25044,"parent_record_offset":24816,"ancestor_record_offset":24816,"inlinee":7340,"source_line":626,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":487,"record_offset":24960,"parent_record_offset":24816,"ancestor_record_offset":24816,"inlinee":7336,"source_line":617,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":487,"record_offset":25008,"parent_record_offset":24984,"ancestor_record_offset":24816,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":487,"record_offset":25068,"parent_record_offset":25044,"ancestor_record_offset":24816,"inlinee":7341,"source_line":626,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":487,"record_offset":24984,"parent_record_offset":24960,"ancestor_record_offset":24816,"inlinee":7337,"source_line":617,"archive":"_pdb/A/modules/00487.jsonl.gz"}
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
// 完整结构及原始类型记录: ui_2d_pass.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::read_access_buffer", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo>", "constructor": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::read_access_buffer", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo>", "constructor": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ui_2d_pass.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/ui_2d_pass.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: ui_2d_pass.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::uniform","relationship":"pdb_inline_direct","evidence":{"module":482,"record_offset":12588,"parent_record_offset":3056,"ancestor_record_offset":3056,"inlinee":23127,"source_line":461,"archive":"_pdb/A/modules/00482.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":482,"record_offset":13180,"parent_record_offset":3056,"ancestor_record_offset":3056,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00482.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::uniform","relationship":"pdb_inline_direct","evidence":{"module":482,"record_offset":22640,"parent_record_offset":14288,"ancestor_record_offset":14288,"inlinee":23127,"source_line":461,"archive":"_pdb/A/modules/00482.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":482,"record_offset":22712,"parent_record_offset":14288,"ancestor_record_offset":14288,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00482.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure$0
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":487,"record_offset":24868,"parent_record_offset":24816,"ancestor_record_offset":24816,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00487.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3144655,"position_among_mapped_render_targets":65,"mapped_render_target_count":70,"previous_callsite":3144622,"next_callsite":3144710,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/ui_pass.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/ui_2d_pass.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3144710,"position_among_mapped_render_targets":66,"mapped_render_target_count":70,"previous_callsite":3144655,"next_callsite":3145032,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/ui_2d_pass.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/ui_pass.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":3152112,"caller_aliases":{"alias_group_rva":3152112,"count":1,"display_examples":["tiny_glade::render_loop::render"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3159251,"position_among_mapped_render_targets":7,"mapped_render_target_count":9,"previous_callsite":3159209,"next_callsite":3159297,"previous_target_files":["crates/country-core/src/systems/render_loop/vermeer.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/ui_pass.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ui_2d_pass.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<ref$<alloc::string::String> >,ref$<ref$<alloc::string::String> >)
// 候选参数类型: ["ref$<ref$<alloc::string::String> >", "ref$<ref$<alloc::string::String> >"]
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell)
// 候选参数类型: ["country_core::render::pass_builder::draw_builder::DrawBuilder", "ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>", "bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<std::path::PathBuf>,ref$<std::path::PathBuf>)
// 候选参数类型: ["ref$<std::path::PathBuf>", "ref$<std::path::PathBuf>"]

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass
// 信息 ID: 9323819253b60e23062f67a1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2ffc06 -> 0x688050 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render @ 0x3034d3 -> 0x688050 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure$0
// 信息 ID: fe26c23f67f2bb29f91038ca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure_env$0 <- country_core::render::raster_single::raster_single_object<1,array$<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::render::raster_single::raster_single_object<1,array$<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure_env$0>; module=487, inline_record=24816; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass
// 信息 ID: a47b4546407a8e026984d829
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2ffbcf -> 0x686910 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure$0
// 信息 ID: 195318abcbe66a1c52114994
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0 <- core::slice::sort::merge_sort<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0>,alloc::slice::stable_sort::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0 <- core::slice::sort::merge_sort<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0>,alloc::slice::stable_sort::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0 <- core::slice::sort::merge_sort<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0>,alloc::slice::stable_sort::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0 <- core::slice::sort::merge_sort<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0>,alloc::slice::stable_sort::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0 <- core::slice::sort::merge_sort<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0>,alloc::slice::stable_sort::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0 <- core::slice::sort::insertion_sort_shift_left<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] core::slice::sort::merge_sort<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0>,alloc::slice::stable_sort::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> > >; module=1336, inline_record=89632; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0>,alloc::slice::stable_sort::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> > >; module=1336, inline_record=89804; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0>,alloc::slice::stable_sort::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> > >; module=1336, inline_record=89984; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0>,alloc::slice::stable_sort::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> > >; module=1336, inline_record=91084; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0>,alloc::slice::stable_sort::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> > >; module=1336, inline_record=91472; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >; module=1336, inline_record=102180; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >; module=1336, inline_record=102652; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure$1
// 信息 ID: eea8448ac9c0393877020847
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass; module=482, inline_record=13148; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
