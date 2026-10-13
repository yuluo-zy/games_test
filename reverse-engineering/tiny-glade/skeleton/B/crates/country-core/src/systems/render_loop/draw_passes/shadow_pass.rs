// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/draw_passes/shadow_pass.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: dcf0968ce581be486ff1459fd7a453915a6a0513eb64df6546533843b617d5da
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: shadow_pass.rs.pdb.jsonl.gz
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
// 函数符号: country_core::systems::render_loop::draw_passes::shadow_pass::shadow_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":530,"record_offset":89456,"rva":10273392,"va":5378982512}
// 编译实例 RVAs: 0x9cc270
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: shadow_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9382949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378982512,"raw_hex":"e8dbd370ff","size":5,"soft_fail":false,"target_va":5369600800,"va":5378983744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=2
//     target_aliases=19 candidates; complete list: shadow_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9367080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378982512,"raw_hex":"e8d81171ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5378983187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408923c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1287974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378982512,"raw_hex":"e8da58ecff","size":5,"soft_fail":false,"target_va":5377696704,"va":5378984673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14090b460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-791509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378982512,"raw_hex":"e82becf3ff","size":5,"soft_fail":false,"target_va":5378192480,"va":5378983984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be6750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2203676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378982512,"raw_hex":"e81ca02100","size":5,"soft_fail":false,"target_va":5381187408,"va":5378983727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shadow_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32685740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378982512,"raw_hex":"e8acbef201","size":5,"soft_fail":false,"target_va":5411669968,"va":5378984223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: shadow_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32863151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378982512,"raw_hex":"e8af73f501","size":5,"soft_fail":false,"target_va":5411847168,"va":5378984012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32557277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378982512,"raw_hex":"e8ddc8f001","size":5,"soft_fail":false,"target_va":5411541584,"va":5378984302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32557450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378982512,"raw_hex":"e88ac9f001","size":5,"soft_fail":false,"target_va":5411541776,"va":5378984321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-14684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378982512,"raw_hex":"e8a4c6ffff","size":5,"soft_fail":false,"target_va":5378967920,"va":5378982599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32560686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378982512,"raw_hex":"e82ed6f001","size":5,"soft_fail":false,"target_va":5411543360,"va":5378982669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_rg_texture
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-587341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378982512,"raw_hex":"e8b309f7ff","size":5,"soft_fail":false,"target_va":5378395952,"va":5378983288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-814137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378982512,"raw_hex":"e8c793f3ff","size":5,"soft_fail":false,"target_va":5378169824,"va":5378983956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1461440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378982512,"raw_hex":"e840b3e9ff","size":5,"soft_fail":false,"target_va":5377522032,"va":5378983467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1459496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378982512,"raw_hex":"e8d8bae9ff","size":5,"soft_fail":false,"target_va":5377524592,"va":5378984083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2402409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378982512,"raw_hex":"e869a82400","size":5,"soft_fail":false,"target_va":5381385536,"va":5378983122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2407598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378982512,"raw_hex":"e8aebc2400","size":5,"soft_fail":false,"target_va":5381390640,"va":5378983037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2402301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378982512,"raw_hex":"e8fda72400","size":5,"soft_fail":false,"target_va":5381386480,"va":5378984174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2402129","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378982512,"raw_hex":"e851a72400","size":5,"soft_fail":false,"target_va":5381384704,"va":5378982570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2134468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378982512,"raw_hex":"e8c4912000","size":5,"soft_fail":false,"target_va":5381117840,"va":5378983367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9840774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378982512,"raw_hex":"e886289600","size":5,"soft_fail":false,"target_va":5388825296,"va":5378984517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32552830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378982512,"raw_hex":"e87eb7f001","size":5,"soft_fail":false,"target_va":5411537104,"va":5378984269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rdi","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5378982512,"raw_hex":"ffd7","size":2,"soft_fail":false,"va":5378984033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":530,"record_offset":93024,"parent_record_offset":89456,"ancestor_record_offset":89456,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":530,"record_offset":94856,"parent_record_offset":89456,"ancestor_record_offset":89456,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":530,"record_offset":94188,"parent_record_offset":89456,"ancestor_record_offset":89456,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_list::DrawListItem::to_draw_packet
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_list.rs
//     evidence={"module":530,"record_offset":94264,"parent_record_offset":89456,"ancestor_record_offset":89456,"inlinee":25983,"source_line":32,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":530,"record_offset":93120,"parent_record_offset":89456,"ancestor_record_offset":89456,"inlinee":8275,"source_line":337,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::framebuffer_binding::DepthAttachment::layer
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":530,"record_offset":93252,"parent_record_offset":89456,"ancestor_record_offset":89456,"inlinee":28443,"source_line":36,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::common_draw_params::CommonDrawParams::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs
//     evidence={"module":530,"record_offset":93280,"parent_record_offset":89456,"ancestor_record_offset":89456,"inlinee":25981,"source_line":26,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":530,"record_offset":92060,"parent_record_offset":89456,"ancestor_record_offset":89456,"inlinee":11668,"source_line":41,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":530,"record_offset":91092,"parent_record_offset":89456,"ancestor_record_offset":89456,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":530,"record_offset":89660,"parent_record_offset":89456,"ancestor_record_offset":89456,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":530,"record_offset":90116,"parent_record_offset":89456,"ancestor_record_offset":89456,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_depth_compare_mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":530,"record_offset":94772,"parent_record_offset":89456,"ancestor_record_offset":89456,"inlinee":30327,"source_line":626,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":530,"record_offset":92528,"parent_record_offset":92504,"ancestor_record_offset":89456,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":530,"record_offset":92576,"parent_record_offset":92552,"ancestor_record_offset":89456,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":530,"record_offset":92552,"parent_record_offset":92528,"ancestor_record_offset":89456,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":530,"record_offset":92504,"parent_record_offset":92480,"ancestor_record_offset":89456,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":530,"record_offset":92480,"parent_record_offset":92440,"ancestor_record_offset":89456,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":530,"record_offset":92440,"parent_record_offset":92416,"ancestor_record_offset":89456,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":530,"record_offset":92828,"parent_record_offset":92804,"ancestor_record_offset":89456,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":530,"record_offset":92804,"parent_record_offset":92780,"ancestor_record_offset":89456,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":530,"record_offset":92752,"parent_record_offset":92720,"ancestor_record_offset":89456,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":530,"record_offset":93936,"parent_record_offset":93400,"ancestor_record_offset":89456,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":530,"record_offset":92696,"parent_record_offset":92156,"ancestor_record_offset":89456,"inlinee":4166,"source_line":134,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":530,"record_offset":92720,"parent_record_offset":92696,"ancestor_record_offset":89456,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":530,"record_offset":92188,"parent_record_offset":92156,"ancestor_record_offset":89456,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":530,"record_offset":94020,"parent_record_offset":93960,"ancestor_record_offset":89456,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":530,"record_offset":92780,"parent_record_offset":92720,"ancestor_record_offset":89456,"inlinee":4176,"source_line":141,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":530,"record_offset":92236,"parent_record_offset":92212,"ancestor_record_offset":89456,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":530,"record_offset":92212,"parent_record_offset":92188,"ancestor_record_offset":89456,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":530,"record_offset":92416,"parent_record_offset":92392,"ancestor_record_offset":89456,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":530,"record_offset":92896,"parent_record_offset":92872,"ancestor_record_offset":89456,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":530,"record_offset":92872,"parent_record_offset":92156,"ancestor_record_offset":89456,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":530,"record_offset":92116,"parent_record_offset":92088,"ancestor_record_offset":89456,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":530,"record_offset":93344,"parent_record_offset":93316,"ancestor_record_offset":89456,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":530,"record_offset":93400,"parent_record_offset":93344,"ancestor_record_offset":89456,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":530,"record_offset":92088,"parent_record_offset":92060,"ancestor_record_offset":89456,"inlinee":9175,"source_line":490,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":530,"record_offset":92156,"parent_record_offset":92088,"ancestor_record_offset":89456,"inlinee":4174,"source_line":515,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":530,"record_offset":91412,"parent_record_offset":91384,"ancestor_record_offset":89456,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":530,"record_offset":94596,"parent_record_offset":94568,"ancestor_record_offset":89456,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":530,"record_offset":91464,"parent_record_offset":91440,"ancestor_record_offset":89456,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":530,"record_offset":91440,"parent_record_offset":91412,"ancestor_record_offset":89456,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":530,"record_offset":91360,"parent_record_offset":91332,"ancestor_record_offset":89456,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":530,"record_offset":91384,"parent_record_offset":91360,"ancestor_record_offset":89456,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":530,"record_offset":91556,"parent_record_offset":91384,"ancestor_record_offset":89456,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":530,"record_offset":91668,"parent_record_offset":91644,"ancestor_record_offset":89456,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":530,"record_offset":93080,"parent_record_offset":93052,"ancestor_record_offset":89456,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":530,"record_offset":93152,"parent_record_offset":93120,"ancestor_record_offset":89456,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":530,"record_offset":93052,"parent_record_offset":93024,"ancestor_record_offset":89456,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":530,"record_offset":91488,"parent_record_offset":91464,"ancestor_record_offset":89456,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":530,"record_offset":89816,"parent_record_offset":89784,"ancestor_record_offset":89456,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":530,"record_offset":91228,"parent_record_offset":91188,"ancestor_record_offset":89456,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":530,"record_offset":92352,"parent_record_offset":92328,"ancestor_record_offset":89456,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":530,"record_offset":92392,"parent_record_offset":92212,"ancestor_record_offset":89456,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":530,"record_offset":92992,"parent_record_offset":92060,"ancestor_record_offset":89456,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":530,"record_offset":93372,"parent_record_offset":93344,"ancestor_record_offset":89456,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":530,"record_offset":90064,"parent_record_offset":90036,"ancestor_record_offset":89456,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":530,"record_offset":92016,"parent_record_offset":91188,"ancestor_record_offset":89456,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":530,"record_offset":91620,"parent_record_offset":91332,"ancestor_record_offset":89456,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":530,"record_offset":90528,"parent_record_offset":90504,"ancestor_record_offset":89456,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":530,"record_offset":90504,"parent_record_offset":90480,"ancestor_record_offset":89456,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":530,"record_offset":90712,"parent_record_offset":90684,"ancestor_record_offset":89456,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":530,"record_offset":91512,"parent_record_offset":91488,"ancestor_record_offset":89456,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":530,"record_offset":92264,"parent_record_offset":92236,"ancestor_record_offset":89456,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":530,"record_offset":92644,"parent_record_offset":92620,"ancestor_record_offset":89456,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":530,"record_offset":92300,"parent_record_offset":92264,"ancestor_record_offset":89456,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":530,"record_offset":92620,"parent_record_offset":92440,"ancestor_record_offset":89456,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":530,"record_offset":90456,"parent_record_offset":90432,"ancestor_record_offset":89456,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":530,"record_offset":90480,"parent_record_offset":90456,"ancestor_record_offset":89456,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":530,"record_offset":90784,"parent_record_offset":90756,"ancestor_record_offset":89456,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":530,"record_offset":90432,"parent_record_offset":90316,"ancestor_record_offset":89456,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":530,"record_offset":89784,"parent_record_offset":89756,"ancestor_record_offset":89456,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":530,"record_offset":90144,"parent_record_offset":90116,"ancestor_record_offset":89456,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":530,"record_offset":90284,"parent_record_offset":90252,"ancestor_record_offset":89456,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":530,"record_offset":90252,"parent_record_offset":90216,"ancestor_record_offset":89456,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":530,"record_offset":90192,"parent_record_offset":90168,"ancestor_record_offset":89456,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":530,"record_offset":90216,"parent_record_offset":90192,"ancestor_record_offset":89456,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":530,"record_offset":90168,"parent_record_offset":90144,"ancestor_record_offset":89456,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":530,"record_offset":90756,"parent_record_offset":90216,"ancestor_record_offset":89456,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":530,"record_offset":89756,"parent_record_offset":89724,"ancestor_record_offset":89456,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":530,"record_offset":89724,"parent_record_offset":89696,"ancestor_record_offset":89456,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":530,"record_offset":89916,"parent_record_offset":89884,"ancestor_record_offset":89456,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":530,"record_offset":90012,"parent_record_offset":89916,"ancestor_record_offset":89456,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":530,"record_offset":89884,"parent_record_offset":89860,"ancestor_record_offset":89456,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":530,"record_offset":89696,"parent_record_offset":89660,"ancestor_record_offset":89456,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":530,"record_offset":90036,"parent_record_offset":90012,"ancestor_record_offset":89456,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":530,"record_offset":90316,"parent_record_offset":90284,"ancestor_record_offset":89456,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":530,"record_offset":90600,"parent_record_offset":90316,"ancestor_record_offset":89456,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":530,"record_offset":90572,"parent_record_offset":90316,"ancestor_record_offset":89456,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":530,"record_offset":90376,"parent_record_offset":90316,"ancestor_record_offset":89456,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":530,"record_offset":92920,"parent_record_offset":92896,"ancestor_record_offset":89456,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":530,"record_offset":92944,"parent_record_offset":92920,"ancestor_record_offset":89456,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::as_secs_f32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":530,"record_offset":94152,"parent_record_offset":93280,"ancestor_record_offset":89456,"inlinee":11237,"source_line":853,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":530,"record_offset":94296,"parent_record_offset":94264,"ancestor_record_offset":89456,"inlinee":25985,"source_line":106,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":530,"record_offset":94344,"parent_record_offset":94296,"ancestor_record_offset":89456,"inlinee":25987,"source_line":138,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_depth
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":530,"record_offset":93208,"parent_record_offset":93180,"ancestor_record_offset":89456,"inlinee":28409,"source_line":157,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$6::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":530,"record_offset":93180,"parent_record_offset":93152,"ancestor_record_offset":89456,"inlinee":28448,"source_line":185,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":530,"record_offset":93316,"parent_record_offset":93280,"ancestor_record_offset":89456,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":530,"record_offset":94820,"parent_record_offset":94796,"ancestor_record_offset":89456,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":530,"record_offset":91124,"parent_record_offset":91092,"ancestor_record_offset":89456,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":530,"record_offset":94920,"parent_record_offset":94888,"ancestor_record_offset":89456,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":530,"record_offset":95420,"parent_record_offset":95160,"ancestor_record_offset":89456,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":530,"record_offset":91256,"parent_record_offset":91228,"ancestor_record_offset":89456,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":530,"record_offset":91332,"parent_record_offset":91188,"ancestor_record_offset":89456,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":530,"record_offset":91592,"parent_record_offset":91332,"ancestor_record_offset":89456,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_compare_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":530,"record_offset":94796,"parent_record_offset":94772,"ancestor_record_offset":89456,"inlinee":30328,"source_line":626,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":530,"record_offset":94376,"parent_record_offset":94344,"ancestor_record_offset":89456,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":530,"record_offset":94440,"parent_record_offset":94408,"ancestor_record_offset":89456,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":530,"record_offset":94568,"parent_record_offset":94508,"ancestor_record_offset":89456,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":530,"record_offset":94408,"parent_record_offset":94376,"ancestor_record_offset":89456,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":530,"record_offset":91284,"parent_record_offset":91256,"ancestor_record_offset":89456,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":530,"record_offset":91188,"parent_record_offset":91156,"ancestor_record_offset":89456,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":530,"record_offset":91156,"parent_record_offset":91124,"ancestor_record_offset":89456,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00530.jsonl.gz"}
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
// 完整结构及原始类型记录: shadow_pass.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: shadow_pass.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/shadow_pass.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: shadow_pass.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// [Shader 映射] {"shader_id":"78d00658517d567633a65fe0","relative_path":"_shadow_mask_mean.cs.hlsl_cs_main_0.bin","sha256":"30d8596b67a181dde5186f27bc9228ba6f348767c2d376d3ced744ef527960d0","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"shadow_pass","prefix_candidates":["shadow_pass","shadow_"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _shadow_mask_mean.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[256,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":16,"name":"frame_constants","storage_class":"Uniform","type":{"id":45,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,14]},"decorations":[],"pointee":{"id":14,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[39,39,39,39,39,39,39,39,28,27,27,28,40,28,27,41,27,41,28,41,28,27,28,28,28,39,22]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":39,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":39,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":39,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":39,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":39,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":39,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":39,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":39,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":28,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":27,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":27,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":28,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":40,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":28,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":27,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":41,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":27,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":41,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":28,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":41,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":28,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":27,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":28,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":28,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":28,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":39,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":22,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":18,"name":"__alias_shadow_mask_histogram","storage_class":"Uniform","type":{"id":46,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,17]},"decorations":[],"pointee":{"id":17,"name":"type.RWStructuredBuffer.uint","record":{"opcode":"OpTypeStruct","operands":[23]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":23,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":21,"name":"__alias_shadow_mask_mean","storage_class":"Uniform","type":{"id":49,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,20]},"decorations":[],"pointee":{"id":20,"name":"type.RWStructuredBuffer.float","record":{"opcode":"OpTypeStruct","operands":[24]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":24,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":50,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,28]},"decorations":[],"pointee":{"id":28,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":5161232,"caller_aliases":{"alias_group_rva":5161232,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5161962,"position_among_mapped_render_targets":28,"mapped_render_target_count":59,"previous_callsite":5161941,"next_callsite":5161973,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/gpu_generate_draw_lists.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/terrain_lighting_texture.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: shadow_pass.rs.enriched.json.gz
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
// 函数: country_core::systems::render_loop::draw_passes::shadow_pass::shadow_pass
// 信息 ID: 90c29e9cff83308b12b1f1d2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ec3ea -> 0x9cc270 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
