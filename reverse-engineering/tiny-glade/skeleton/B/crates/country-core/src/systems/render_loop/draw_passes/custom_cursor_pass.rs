// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/draw_passes/custom_cursor_pass.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 6c7cc8756e3d1e07bc72f8b875da408aad8798fed76b0a3cdeecf9a39e650fc5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: custom_cursor_pass.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 531
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
// 函数符号: country_core::systems::render_loop::draw_passes::custom_cursor_pass::custom_cursor_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":531,"record_offset":65540,"rva":10295056,"va":5379004176}
// 编译实例 RVAs: 0x9d1710
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: custom_cursor_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9404515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379004176,"raw_hex":"e89d7f70ff","size":5,"soft_fail":false,"target_va":5369600800,"va":5379005310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=3
//     target_aliases=19 candidates; complete list: custom_cursor_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9388733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379004176,"raw_hex":"e843bd70ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5379004840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408923c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: custom_cursor_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1309726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379004176,"raw_hex":"e8e203ecff","size":5,"soft_fail":false,"target_va":5377696704,"va":5379006425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: custom_cursor_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1305215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379004176,"raw_hex":"e88115ecff","size":5,"soft_fail":false,"target_va":5377701312,"va":5379006522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: custom_cursor_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-825111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379004176,"raw_hex":"e8e968f3ff","size":5,"soft_fail":false,"target_va":5378180624,"va":5379005730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: custom_cursor_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-419180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379004176,"raw_hex":"e8949af9ff","size":5,"soft_fail":false,"target_va":5378586464,"va":5379005639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be6750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: custom_cursor_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2182109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379004176,"raw_hex":"e8dd4b2100","size":5,"soft_fail":false,"target_va":5381187408,"va":5379005294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162aa90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: custom_cursor_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12946733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379004176,"raw_hex":"e82d8dc500","size":5,"soft_fail":false,"target_va":5391952528,"va":5379005790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: custom_cursor_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32663967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379004176,"raw_hex":"e89f69f201","size":5,"soft_fail":false,"target_va":5411669968,"va":5379005996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: custom_cursor_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32841493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379004176,"raw_hex":"e8151ff501","size":5,"soft_fail":false,"target_va":5411847168,"va":5379005670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32535525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379004176,"raw_hex":"e8e573f001","size":5,"soft_fail":false,"target_va":5411541584,"va":5379006054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t32535705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379004176,"raw_hex":"e89974f001","size":5,"soft_fail":false,"target_va":5411541776,"va":5379006066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-16231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379004176,"raw_hex":"e899c0ffff","size":5,"soft_fail":false,"target_va":5378988032,"va":5379004258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32539027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379004176,"raw_hex":"e89381f001","size":5,"soft_fail":false,"target_va":5411543360,"va":5379004328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-630638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379004176,"raw_hex":"e89260f6ff","size":5,"soft_fail":false,"target_va":5378374304,"va":5379004937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_no_vao
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-832276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379004176,"raw_hex":"e8ec4cf3ff","size":5,"soft_fail":false,"target_va":5378173584,"va":5379005855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-835700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379004176,"raw_hex":"e88c3ff3ff","size":5,"soft_fail":false,"target_va":5378169824,"va":5379005519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1483255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379004176,"raw_hex":"e8095ee9ff","size":5,"soft_fail":false,"target_va":5377522032,"va":5379005282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1481301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379004176,"raw_hex":"e8ab65e9ff","size":5,"soft_fail":false,"target_va":5377524592,"va":5379005888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2380759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379004176,"raw_hex":"e8d7532400","size":5,"soft_fail":false,"target_va":5381385536,"va":5379004772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2385948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379004176,"raw_hex":"e81c682400","size":5,"soft_fail":false,"target_va":5381390640,"va":5379004687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2380521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379004176,"raw_hex":"e8e9522400","size":5,"soft_fail":false,"target_va":5381386480,"va":5379005954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2380476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379004176,"raw_hex":"e8bc522400","size":5,"soft_fail":false,"target_va":5381384704,"va":5379004223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2112645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379004176,"raw_hex":"e8853c2000","size":5,"soft_fail":false,"target_va":5381117840,"va":5379005190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9819004","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379004176,"raw_hex":"e87cd39500","size":5,"soft_fail":false,"target_va":5388825296,"va":5379006287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32531057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379004176,"raw_hex":"e87162f001","size":5,"soft_fail":false,"target_va":5411537104,"va":5379006042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":531,"record_offset":68180,"parent_record_offset":65540,"ancestor_record_offset":65540,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":531,"record_offset":68968,"parent_record_offset":65540,"ancestor_record_offset":65540,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":531,"record_offset":71584,"parent_record_offset":65540,"ancestor_record_offset":65540,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":531,"record_offset":71368,"parent_record_offset":65540,"ancestor_record_offset":65540,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":531,"record_offset":71296,"parent_record_offset":65540,"ancestor_record_offset":65540,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":531,"record_offset":70824,"parent_record_offset":65540,"ancestor_record_offset":65540,"inlinee":25985,"source_line":106,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":531,"record_offset":70644,"parent_record_offset":65540,"ancestor_record_offset":65540,"inlinee":8275,"source_line":337,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::window::WindowSize::as_vec2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/window.rs
//     evidence={"module":531,"record_offset":69780,"parent_record_offset":65540,"ancestor_record_offset":65540,"inlinee":26000,"source_line":45,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":531,"record_offset":69808,"parent_record_offset":65540,"ancestor_record_offset":65540,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":531,"record_offset":67196,"parent_record_offset":65540,"ancestor_record_offset":65540,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":531,"record_offset":65764,"parent_record_offset":65540,"ancestor_record_offset":65540,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":531,"record_offset":66220,"parent_record_offset":65540,"ancestor_record_offset":65540,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":531,"record_offset":71496,"parent_record_offset":65540,"ancestor_record_offset":65540,"inlinee":8289,"source_line":637,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":531,"record_offset":71412,"parent_record_offset":65540,"ancestor_record_offset":65540,"inlinee":8285,"source_line":628,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":531,"record_offset":68616,"parent_record_offset":68592,"ancestor_record_offset":65540,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":531,"record_offset":68664,"parent_record_offset":68640,"ancestor_record_offset":65540,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":531,"record_offset":68640,"parent_record_offset":68616,"ancestor_record_offset":65540,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":531,"record_offset":68592,"parent_record_offset":68568,"ancestor_record_offset":65540,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":531,"record_offset":68568,"parent_record_offset":68528,"ancestor_record_offset":65540,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":531,"record_offset":68528,"parent_record_offset":68504,"ancestor_record_offset":65540,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":531,"record_offset":68916,"parent_record_offset":68892,"ancestor_record_offset":65540,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":531,"record_offset":68892,"parent_record_offset":68868,"ancestor_record_offset":65540,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":531,"record_offset":68840,"parent_record_offset":68808,"ancestor_record_offset":65540,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":531,"record_offset":68784,"parent_record_offset":68248,"ancestor_record_offset":65540,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":531,"record_offset":68808,"parent_record_offset":68784,"ancestor_record_offset":65540,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":531,"record_offset":68276,"parent_record_offset":68248,"ancestor_record_offset":65540,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":531,"record_offset":68868,"parent_record_offset":68808,"ancestor_record_offset":65540,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":531,"record_offset":68324,"parent_record_offset":68300,"ancestor_record_offset":65540,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":531,"record_offset":68300,"parent_record_offset":68276,"ancestor_record_offset":65540,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":531,"record_offset":68504,"parent_record_offset":68480,"ancestor_record_offset":65540,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":531,"record_offset":68208,"parent_record_offset":68180,"ancestor_record_offset":65540,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":531,"record_offset":69836,"parent_record_offset":69808,"ancestor_record_offset":65540,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":531,"record_offset":68248,"parent_record_offset":68180,"ancestor_record_offset":65540,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":531,"record_offset":67532,"parent_record_offset":67504,"ancestor_record_offset":65540,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":531,"record_offset":71124,"parent_record_offset":71096,"ancestor_record_offset":65540,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":531,"record_offset":67584,"parent_record_offset":67560,"ancestor_record_offset":65540,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":531,"record_offset":67560,"parent_record_offset":67532,"ancestor_record_offset":65540,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":531,"record_offset":67480,"parent_record_offset":67452,"ancestor_record_offset":65540,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":531,"record_offset":67504,"parent_record_offset":67480,"ancestor_record_offset":65540,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":531,"record_offset":67676,"parent_record_offset":67504,"ancestor_record_offset":65540,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":531,"record_offset":67788,"parent_record_offset":67764,"ancestor_record_offset":65540,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":531,"record_offset":70676,"parent_record_offset":70644,"ancestor_record_offset":65540,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":531,"record_offset":67608,"parent_record_offset":67584,"ancestor_record_offset":65540,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":531,"record_offset":65920,"parent_record_offset":65888,"ancestor_record_offset":65540,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":531,"record_offset":67348,"parent_record_offset":67304,"ancestor_record_offset":65540,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":531,"record_offset":71336,"parent_record_offset":71296,"ancestor_record_offset":65540,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":531,"record_offset":68440,"parent_record_offset":68416,"ancestor_record_offset":65540,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":531,"record_offset":68480,"parent_record_offset":68300,"ancestor_record_offset":65540,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":531,"record_offset":70612,"parent_record_offset":69808,"ancestor_record_offset":65540,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":531,"record_offset":69032,"parent_record_offset":69004,"ancestor_record_offset":65540,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":531,"record_offset":66168,"parent_record_offset":66140,"ancestor_record_offset":65540,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":531,"record_offset":68136,"parent_record_offset":67304,"ancestor_record_offset":65540,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":531,"record_offset":67740,"parent_record_offset":67452,"ancestor_record_offset":65540,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":531,"record_offset":66632,"parent_record_offset":66608,"ancestor_record_offset":65540,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":531,"record_offset":66608,"parent_record_offset":66584,"ancestor_record_offset":65540,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":531,"record_offset":66816,"parent_record_offset":66788,"ancestor_record_offset":65540,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":531,"record_offset":67632,"parent_record_offset":67608,"ancestor_record_offset":65540,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":531,"record_offset":68352,"parent_record_offset":68324,"ancestor_record_offset":65540,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":531,"record_offset":68732,"parent_record_offset":68708,"ancestor_record_offset":65540,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":531,"record_offset":68388,"parent_record_offset":68352,"ancestor_record_offset":65540,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":531,"record_offset":68708,"parent_record_offset":68528,"ancestor_record_offset":65540,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":531,"record_offset":66560,"parent_record_offset":66536,"ancestor_record_offset":65540,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":531,"record_offset":66584,"parent_record_offset":66560,"ancestor_record_offset":65540,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":531,"record_offset":66888,"parent_record_offset":66860,"ancestor_record_offset":65540,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":531,"record_offset":66536,"parent_record_offset":66420,"ancestor_record_offset":65540,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":531,"record_offset":65888,"parent_record_offset":65860,"ancestor_record_offset":65540,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":531,"record_offset":66248,"parent_record_offset":66220,"ancestor_record_offset":65540,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":531,"record_offset":66388,"parent_record_offset":66356,"ancestor_record_offset":65540,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":531,"record_offset":66356,"parent_record_offset":66320,"ancestor_record_offset":65540,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":531,"record_offset":66296,"parent_record_offset":66272,"ancestor_record_offset":65540,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":531,"record_offset":66320,"parent_record_offset":66296,"ancestor_record_offset":65540,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":531,"record_offset":66272,"parent_record_offset":66248,"ancestor_record_offset":65540,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":531,"record_offset":66860,"parent_record_offset":66320,"ancestor_record_offset":65540,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":531,"record_offset":65860,"parent_record_offset":65828,"ancestor_record_offset":65540,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":531,"record_offset":65828,"parent_record_offset":65800,"ancestor_record_offset":65540,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":531,"record_offset":66020,"parent_record_offset":65988,"ancestor_record_offset":65540,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":531,"record_offset":66116,"parent_record_offset":66020,"ancestor_record_offset":65540,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":531,"record_offset":65988,"parent_record_offset":65964,"ancestor_record_offset":65540,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":531,"record_offset":65800,"parent_record_offset":65764,"ancestor_record_offset":65540,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":531,"record_offset":66140,"parent_record_offset":66116,"ancestor_record_offset":65540,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":531,"record_offset":66420,"parent_record_offset":66388,"ancestor_record_offset":65540,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":531,"record_offset":66704,"parent_record_offset":66420,"ancestor_record_offset":65540,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":531,"record_offset":66676,"parent_record_offset":66420,"ancestor_record_offset":65540,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":531,"record_offset":66480,"parent_record_offset":66420,"ancestor_record_offset":65540,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":531,"record_offset":70872,"parent_record_offset":70824,"ancestor_record_offset":65540,"inlinee":25987,"source_line":138,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::ColorAttachment::desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":531,"record_offset":70752,"parent_record_offset":70724,"ancestor_record_offset":65540,"inlinee":25998,"source_line":89,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":531,"record_offset":70724,"parent_record_offset":70700,"ancestor_record_offset":65540,"inlinee":8279,"source_line":145,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$5::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":531,"record_offset":70700,"parent_record_offset":70676,"ancestor_record_offset":65540,"inlinee":8277,"source_line":179,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":531,"record_offset":71460,"parent_record_offset":71436,"ancestor_record_offset":65540,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":531,"record_offset":67232,"parent_record_offset":67196,"ancestor_record_offset":65540,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":531,"record_offset":71648,"parent_record_offset":71616,"ancestor_record_offset":65540,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":531,"record_offset":72148,"parent_record_offset":71888,"ancestor_record_offset":65540,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":531,"record_offset":67376,"parent_record_offset":67348,"ancestor_record_offset":65540,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":531,"record_offset":67452,"parent_record_offset":67304,"ancestor_record_offset":65540,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":531,"record_offset":67712,"parent_record_offset":67452,"ancestor_record_offset":65540,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":531,"record_offset":71520,"parent_record_offset":71496,"ancestor_record_offset":65540,"inlinee":8290,"source_line":637,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":531,"record_offset":71436,"parent_record_offset":71412,"ancestor_record_offset":65540,"inlinee":8286,"source_line":628,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":531,"record_offset":70776,"parent_record_offset":70752,"ancestor_record_offset":65540,"inlinee":8273,"source_line":315,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":531,"record_offset":70904,"parent_record_offset":70872,"ancestor_record_offset":65540,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":531,"record_offset":70968,"parent_record_offset":70936,"ancestor_record_offset":65540,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":531,"record_offset":71096,"parent_record_offset":71036,"ancestor_record_offset":65540,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":531,"record_offset":70936,"parent_record_offset":70904,"ancestor_record_offset":65540,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":531,"record_offset":67404,"parent_record_offset":67376,"ancestor_record_offset":65540,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":531,"record_offset":67304,"parent_record_offset":67268,"ancestor_record_offset":65540,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":531,"record_offset":67268,"parent_record_offset":67232,"ancestor_record_offset":65540,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00531.jsonl.gz"}
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
// 完整结构及原始类型记录: custom_cursor_pass.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::read_access_buffer", "arguments": ["country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer<country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo>", "constructor": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer", "arguments": ["country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: custom_cursor_pass.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-camera/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/custom_cursor_pass.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: custom_cursor_pass.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::custom_cursor_pass::custom_cursor_pass
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::uniform","relationship":"pdb_inline_direct","evidence":{"module":531,"record_offset":71296,"parent_record_offset":65540,"ancestor_record_offset":65540,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00531.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":531,"record_offset":71368,"parent_record_offset":65540,"ancestor_record_offset":65540,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00531.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":1449168,"caller_aliases":{"alias_group_rva":1449168,"count":1,"display_examples":["rhapsody::rg::state::RenderGraphState::compile"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":1455213,"position_among_mapped_render_targets":21,"mapped_render_target_count":25,"previous_callsite":1455177,"next_callsite":1455274,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/ui_2d_pass.rs"],"next_target_files":["crates/tiny-glade/src/render_loop.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: custom_cursor_pass.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::systems::world_raster::RasterHistoryRecordResources,bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone1EditToHistoryCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone2EditToHistoryCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone3EditToHistoryCmd>)
// 候选参数类型: ["country_core::systems::world_raster::RasterHistoryRecordResources", "bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>", "bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone1EditToHistoryCmd>", "bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone2EditToHistoryCmd>", "bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone3EditToHistoryCmd>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone1EditToHistoryCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone2EditToHistoryCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone3EditToHistoryCmd>

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::custom_cursor_pass::custom_cursor_pass
// 信息 ID: 303d544e14d25ec07c1e723b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x16346d -> 0x9d1710 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
