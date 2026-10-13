// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/draw_passes/foliage_exclusion_mask.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: fefff6128adbfaacbdef3995316cb9b50b4653a7838a3ea68bb96ec0c0be19c0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: foliage_exclusion_mask.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 530
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use modular_bitfield; // namespace candidate
//   package identities: [{"name":"modular-bitfield","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::foliage_exclusion_mask::rasterize_foliage_exclusion_mask
// 记录形式: compiled_procedure; instances=1
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":530,"record_offset":81124,"rva":10270640,"va":5378979760}
// 编译实例 RVAs: 0x9cb7b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: foliage_exclusion_mask.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9380465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378979760,"raw_hex":"e88fdd70ff","size":5,"soft_fail":false,"target_va":5369600800,"va":5378981260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=4
//     target_aliases=19 candidates; complete list: foliage_exclusion_mask.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9363724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378979760,"raw_hex":"e8f41e71ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5378979831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408923c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: foliage_exclusion_mask.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1285601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378979760,"raw_hex":"e81f62ecff","size":5,"soft_fail":false,"target_va":5377696704,"va":5378982300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: foliage_exclusion_mask.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1281087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378979760,"raw_hex":"e8c173ecff","size":5,"soft_fail":false,"target_va":5377701312,"va":5378982394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908c50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: foliage_exclusion_mask.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-799380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378979760,"raw_hex":"e86ccdf3ff","size":5,"soft_fail":false,"target_va":5378182224,"va":5378981599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be6750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: foliage_exclusion_mask.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2206159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378979760,"raw_hex":"e8cfa92100","size":5,"soft_fail":false,"target_va":5381187408,"va":5378981244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: foliage_exclusion_mask.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32688099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378979760,"raw_hex":"e8e3c7f201","size":5,"soft_fail":false,"target_va":5411669968,"va":5378981864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: foliage_exclusion_mask.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32865656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378979760,"raw_hex":"e8787df501","size":5,"soft_fail":false,"target_va":5411847168,"va":5378981507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32559657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378979760,"raw_hex":"e829d2f001","size":5,"soft_fail":false,"target_va":5411541584,"va":5378981922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t32559837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378979760,"raw_hex":"e8ddd2f001","size":5,"soft_fail":false,"target_va":5411541776,"va":5378981934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-12034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378979760,"raw_hex":"e8fed0ffff","size":5,"soft_fail":false,"target_va":5378967920,"va":5378979949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32563324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378979760,"raw_hex":"e87ce0f001","size":5,"soft_fail":false,"target_va":5411543360,"va":5378980031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-606664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378979760,"raw_hex":"e838bef6ff","size":5,"soft_fail":false,"target_va":5378374304,"va":5378980963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_rg_texture
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-584228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378979760,"raw_hex":"e8dc15f7ff","size":5,"soft_fail":false,"target_va":5378395952,"va":5378980175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_external_storage_buffer
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-809643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378979760,"raw_hex":"e855a5f3ff","size":5,"soft_fail":false,"target_va":5378171904,"va":5378981542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_no_vao
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-808137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378979760,"raw_hex":"e837abf3ff","size":5,"soft_fail":false,"target_va":5378173584,"va":5378981716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-811653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378979760,"raw_hex":"e87b9df3ff","size":5,"soft_fail":false,"target_va":5378169824,"va":5378981472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1458275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378979760,"raw_hex":"e89dbfe9ff","size":5,"soft_fail":false,"target_va":5377522032,"va":5378980302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1455723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378979760,"raw_hex":"e895c9e9ff","size":5,"soft_fail":false,"target_va":5377524592,"va":5378980310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2404729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378979760,"raw_hex":"e879b12400","size":5,"soft_fail":false,"target_va":5381385536,"va":5378980802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2409918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378979760,"raw_hex":"e8bec52400","size":5,"soft_fail":false,"target_va":5381390640,"va":5378980717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2404638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378979760,"raw_hex":"e81eb12400","size":5,"soft_fail":false,"target_va":5381386480,"va":5378981837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2404784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378979760,"raw_hex":"e8b0b12400","size":5,"soft_fail":false,"target_va":5381384704,"va":5378979915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2137648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378979760,"raw_hex":"e8309e2000","size":5,"soft_fail":false,"target_va":5381117840,"va":5378980187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9843131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378979760,"raw_hex":"e8bb319600","size":5,"soft_fail":false,"target_va":5388825296,"va":5378982160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32555189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378979760,"raw_hex":"e8b5c0f001","size":5,"soft_fail":false,"target_va":5411537104,"va":5378981910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":530,"record_offset":81356,"parent_record_offset":81124,"ancestor_record_offset":81124,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":530,"record_offset":86444,"parent_record_offset":81124,"ancestor_record_offset":81124,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":530,"record_offset":88292,"parent_record_offset":81124,"ancestor_record_offset":81124,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":530,"record_offset":87612,"parent_record_offset":81124,"ancestor_record_offset":81124,"inlinee":25985,"source_line":106,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":530,"record_offset":83544,"parent_record_offset":81124,"ancestor_record_offset":81124,"inlinee":8275,"source_line":337,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":530,"record_offset":88168,"parent_record_offset":81124,"ancestor_record_offset":81124,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":530,"record_offset":82592,"parent_record_offset":81124,"ancestor_record_offset":81124,"inlinee":11668,"source_line":41,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":530,"record_offset":84700,"parent_record_offset":81124,"ancestor_record_offset":81124,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":530,"record_offset":82132,"parent_record_offset":81124,"ancestor_record_offset":81124,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":530,"record_offset":83724,"parent_record_offset":81124,"ancestor_record_offset":81124,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":530,"record_offset":88084,"parent_record_offset":81124,"ancestor_record_offset":81124,"inlinee":27055,"source_line":624,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":530,"record_offset":81780,"parent_record_offset":81756,"ancestor_record_offset":81124,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":530,"record_offset":81828,"parent_record_offset":81804,"ancestor_record_offset":81124,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":530,"record_offset":81804,"parent_record_offset":81780,"ancestor_record_offset":81124,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":530,"record_offset":81756,"parent_record_offset":81732,"ancestor_record_offset":81124,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":530,"record_offset":81732,"parent_record_offset":81692,"ancestor_record_offset":81124,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":530,"record_offset":81692,"parent_record_offset":81668,"ancestor_record_offset":81124,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":530,"record_offset":82080,"parent_record_offset":82056,"ancestor_record_offset":81124,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":530,"record_offset":82056,"parent_record_offset":82032,"ancestor_record_offset":81124,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":530,"record_offset":82004,"parent_record_offset":81972,"ancestor_record_offset":81124,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":530,"record_offset":81948,"parent_record_offset":81420,"ancestor_record_offset":81124,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":530,"record_offset":83216,"parent_record_offset":82676,"ancestor_record_offset":81124,"inlinee":4166,"source_line":134,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":530,"record_offset":81972,"parent_record_offset":81948,"ancestor_record_offset":81124,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":530,"record_offset":81448,"parent_record_offset":81420,"ancestor_record_offset":81124,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":530,"record_offset":82032,"parent_record_offset":81972,"ancestor_record_offset":81124,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":530,"record_offset":83300,"parent_record_offset":83240,"ancestor_record_offset":81124,"inlinee":4176,"source_line":141,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":530,"record_offset":81496,"parent_record_offset":81472,"ancestor_record_offset":81124,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":530,"record_offset":81472,"parent_record_offset":81448,"ancestor_record_offset":81124,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":530,"record_offset":81668,"parent_record_offset":81644,"ancestor_record_offset":81124,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":530,"record_offset":83416,"parent_record_offset":83392,"ancestor_record_offset":81124,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":530,"record_offset":83392,"parent_record_offset":82676,"ancestor_record_offset":81124,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":530,"record_offset":81384,"parent_record_offset":81356,"ancestor_record_offset":81124,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":530,"record_offset":81420,"parent_record_offset":81356,"ancestor_record_offset":81124,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":530,"record_offset":82620,"parent_record_offset":82592,"ancestor_record_offset":81124,"inlinee":9175,"source_line":490,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":530,"record_offset":82676,"parent_record_offset":82620,"ancestor_record_offset":81124,"inlinee":4174,"source_line":515,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":530,"record_offset":85020,"parent_record_offset":84992,"ancestor_record_offset":81124,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":530,"record_offset":87912,"parent_record_offset":87884,"ancestor_record_offset":81124,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":530,"record_offset":85072,"parent_record_offset":85048,"ancestor_record_offset":81124,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":530,"record_offset":85048,"parent_record_offset":85020,"ancestor_record_offset":81124,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":530,"record_offset":84968,"parent_record_offset":84940,"ancestor_record_offset":81124,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":530,"record_offset":84992,"parent_record_offset":84968,"ancestor_record_offset":81124,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":530,"record_offset":85164,"parent_record_offset":84992,"ancestor_record_offset":81124,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":530,"record_offset":85276,"parent_record_offset":85252,"ancestor_record_offset":81124,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":530,"record_offset":83576,"parent_record_offset":83544,"ancestor_record_offset":81124,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":530,"record_offset":85096,"parent_record_offset":85072,"ancestor_record_offset":81124,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":530,"record_offset":82292,"parent_record_offset":82260,"ancestor_record_offset":81124,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":530,"record_offset":81584,"parent_record_offset":81524,"ancestor_record_offset":81124,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":530,"record_offset":81604,"parent_record_offset":81584,"ancestor_record_offset":81124,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":530,"record_offset":81644,"parent_record_offset":81472,"ancestor_record_offset":81124,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":530,"record_offset":83512,"parent_record_offset":82592,"ancestor_record_offset":81124,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":530,"record_offset":82648,"parent_record_offset":82620,"ancestor_record_offset":81124,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":530,"record_offset":82540,"parent_record_offset":82512,"ancestor_record_offset":81124,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":530,"record_offset":85624,"parent_record_offset":84796,"ancestor_record_offset":81124,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":530,"record_offset":85228,"parent_record_offset":84940,"ancestor_record_offset":81124,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":530,"record_offset":84136,"parent_record_offset":84112,"ancestor_record_offset":81124,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":530,"record_offset":84112,"parent_record_offset":84088,"ancestor_record_offset":81124,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":530,"record_offset":84320,"parent_record_offset":84292,"ancestor_record_offset":81124,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":530,"record_offset":85120,"parent_record_offset":85096,"ancestor_record_offset":81124,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":530,"record_offset":81524,"parent_record_offset":81496,"ancestor_record_offset":81124,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":530,"record_offset":81896,"parent_record_offset":81872,"ancestor_record_offset":81124,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":530,"record_offset":81556,"parent_record_offset":81524,"ancestor_record_offset":81124,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":530,"record_offset":81872,"parent_record_offset":81692,"ancestor_record_offset":81124,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":530,"record_offset":84064,"parent_record_offset":84040,"ancestor_record_offset":81124,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":530,"record_offset":84088,"parent_record_offset":84064,"ancestor_record_offset":81124,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":530,"record_offset":84392,"parent_record_offset":84364,"ancestor_record_offset":81124,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":530,"record_offset":84040,"parent_record_offset":83924,"ancestor_record_offset":81124,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":530,"record_offset":82260,"parent_record_offset":82232,"ancestor_record_offset":81124,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":530,"record_offset":83752,"parent_record_offset":83724,"ancestor_record_offset":81124,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":530,"record_offset":83892,"parent_record_offset":83860,"ancestor_record_offset":81124,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":530,"record_offset":83860,"parent_record_offset":83824,"ancestor_record_offset":81124,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":530,"record_offset":83800,"parent_record_offset":83776,"ancestor_record_offset":81124,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":530,"record_offset":83824,"parent_record_offset":83800,"ancestor_record_offset":81124,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":530,"record_offset":83776,"parent_record_offset":83752,"ancestor_record_offset":81124,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":530,"record_offset":84364,"parent_record_offset":83824,"ancestor_record_offset":81124,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":530,"record_offset":82232,"parent_record_offset":82196,"ancestor_record_offset":81124,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":530,"record_offset":82196,"parent_record_offset":82168,"ancestor_record_offset":81124,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":530,"record_offset":82392,"parent_record_offset":82360,"ancestor_record_offset":81124,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":530,"record_offset":82488,"parent_record_offset":82392,"ancestor_record_offset":81124,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":530,"record_offset":82360,"parent_record_offset":82336,"ancestor_record_offset":81124,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":530,"record_offset":82168,"parent_record_offset":82132,"ancestor_record_offset":81124,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":530,"record_offset":82512,"parent_record_offset":82488,"ancestor_record_offset":81124,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":530,"record_offset":83924,"parent_record_offset":83892,"ancestor_record_offset":81124,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":530,"record_offset":84208,"parent_record_offset":83924,"ancestor_record_offset":81124,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":530,"record_offset":84180,"parent_record_offset":83924,"ancestor_record_offset":81124,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":530,"record_offset":83984,"parent_record_offset":83924,"ancestor_record_offset":81124,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":530,"record_offset":83440,"parent_record_offset":83416,"ancestor_record_offset":81124,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":530,"record_offset":83464,"parent_record_offset":83440,"ancestor_record_offset":81124,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":530,"record_offset":88200,"parent_record_offset":88168,"ancestor_record_offset":81124,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":530,"record_offset":87660,"parent_record_offset":87612,"ancestor_record_offset":81124,"inlinee":25987,"source_line":138,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::ColorAttachment::desc
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":530,"record_offset":83652,"parent_record_offset":83624,"ancestor_record_offset":81124,"inlinee":25998,"source_line":89,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":530,"record_offset":83624,"parent_record_offset":83600,"ancestor_record_offset":81124,"inlinee":8279,"source_line":145,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$5::from
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":530,"record_offset":83600,"parent_record_offset":83576,"ancestor_record_offset":81124,"inlinee":8277,"source_line":179,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":530,"record_offset":88132,"parent_record_offset":88108,"ancestor_record_offset":81124,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":530,"record_offset":84732,"parent_record_offset":84700,"ancestor_record_offset":81124,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":530,"record_offset":88348,"parent_record_offset":88320,"ancestor_record_offset":81124,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":530,"record_offset":88836,"parent_record_offset":88576,"ancestor_record_offset":81124,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":530,"record_offset":84864,"parent_record_offset":84836,"ancestor_record_offset":81124,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":530,"record_offset":84940,"parent_record_offset":84796,"ancestor_record_offset":81124,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":530,"record_offset":85200,"parent_record_offset":84940,"ancestor_record_offset":81124,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":530,"record_offset":88108,"parent_record_offset":88084,"ancestor_record_offset":81124,"inlinee":27056,"source_line":624,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":530,"record_offset":83676,"parent_record_offset":83652,"ancestor_record_offset":81124,"inlinee":8273,"source_line":315,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":530,"record_offset":87692,"parent_record_offset":87660,"ancestor_record_offset":81124,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":530,"record_offset":87756,"parent_record_offset":87724,"ancestor_record_offset":81124,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":530,"record_offset":87884,"parent_record_offset":87824,"ancestor_record_offset":81124,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":530,"record_offset":87724,"parent_record_offset":87692,"ancestor_record_offset":81124,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":530,"record_offset":84892,"parent_record_offset":84864,"ancestor_record_offset":81124,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":530,"record_offset":84796,"parent_record_offset":84764,"ancestor_record_offset":81124,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":530,"record_offset":84764,"parent_record_offset":84732,"ancestor_record_offset":81124,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00530.jsonl.gz"}
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
// 完整结构及原始类型记录: foliage_exclusion_mask.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: foliage_exclusion_mask.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/foliage_exclusion_mask.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: foliage_exclusion_mask.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::foliage_exclusion_mask::rasterize_foliage_exclusion_mask
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":530,"record_offset":88168,"parent_record_offset":81124,"ancestor_record_offset":81124,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00530.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":530,"record_offset":88200,"parent_record_offset":88168,"ancestor_record_offset":81124,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00530.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":5161232,"caller_aliases":{"alias_group_rva":5161232,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5161773,"position_among_mapped_render_targets":12,"mapped_render_target_count":59,"previous_callsite":5161762,"next_callsite":5161784,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/mouse_read_terrain_stroke_index.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/highlight_active_terrain_stroke.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: foliage_exclusion_mask.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::foliage_exclusion_mask::rasterize_foliage_exclusion_mask
// 信息 ID: e1a8416550f446df0a2e2b40
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ec32d -> 0x9cb7b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
