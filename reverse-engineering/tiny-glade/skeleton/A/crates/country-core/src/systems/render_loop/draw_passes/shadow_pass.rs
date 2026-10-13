// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/draw_passes/shadow_pass.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 078c144d2b805d7849526198248ad4d961dd9a785ce9ee3fd2a0195b7ed82c30
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: shadow_pass.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 449
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
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: []
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
// 首条位置: {"module":449,"record_offset":7488,"rva":6324144,"va":5375033264}
// 编译实例 RVAs: 0x607fb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: shadow_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2409537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375033264,"raw_hex":"e8bf3bdbff","size":5,"soft_fail":false,"target_va":5372625024,"va":5375034556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ae440
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t679575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375033264,"raw_hex":"e8975e0a00","size":5,"soft_fail":false,"target_va":5375714368,"va":5375034788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eb110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t928274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375033264,"raw_hex":"e8122a0e00","size":5,"soft_fail":false,"target_va":5375963408,"va":5375035129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072a6e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1188428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375033264,"raw_hex":"e84c221200","size":5,"soft_fail":false,"target_va":5376222944,"va":5375034511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8274439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375033264,"raw_hex":"e807427e00","size":5,"soft_fail":false,"target_va":5383308144,"va":5375033700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: shadow_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28558539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375033264,"raw_hex":"e8cbc4b301","size":5,"soft_fail":false,"target_va":5403593360,"va":5375034816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27943679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375033264,"raw_hex":"e8ff62aa01","size":5,"soft_fail":false,"target_va":5402978688,"va":5375035004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27943885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375033264,"raw_hex":"e8cd63aa01","size":5,"soft_fail":false,"target_va":5402978880,"va":5375034990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27944171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375033264,"raw_hex":"e8eb64aa01","size":5,"soft_fail":false,"target_va":5402979152,"va":5375034976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-3681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375033264,"raw_hex":"e89ff1ffff","size":5,"soft_fail":false,"target_va":5375031248,"va":5375034924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27946371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375033264,"raw_hex":"e8836daa01","size":5,"soft_fail":false,"target_va":5402980128,"va":5375033752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375033264,"raw_hex":"e891f8ffff","size":5,"soft_fail":false,"target_va":5375031440,"va":5375033338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27947236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375033264,"raw_hex":"e8e470aa01","size":5,"soft_fail":false,"target_va":5402980656,"va":5375033415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_rg_texture
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-678286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375033264,"raw_hex":"e872a6f5ff","size":5,"soft_fail":false,"target_va":5374355936,"va":5375034217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t665923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375033264,"raw_hex":"e843290a00","size":5,"soft_fail":false,"target_va":5375700688,"va":5375034760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t361128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375033264,"raw_hex":"e8a8820500","size":5,"soft_fail":false,"target_va":5375395536,"va":5375034403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t363350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375033264,"raw_hex":"e8568b0500","size":5,"soft_fail":false,"target_va":5375398240,"va":5375034885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::common_draw_params::CommonDrawParams::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-115470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375033264,"raw_hex":"e8f23cfeff","size":5,"soft_fail":false,"target_va":5374918976,"va":5375034441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1322989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375033264,"raw_hex":"e8ed2f1400","size":5,"soft_fail":false,"target_va":5376356912,"va":5375033918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1326875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375033264,"raw_hex":"e81b3f1400","size":5,"soft_fail":false,"target_va":5376360560,"va":5375033680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1322824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375033264,"raw_hex":"e8482f1400","size":5,"soft_fail":false,"target_va":5376356144,"va":5375033315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1307413","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375033264,"raw_hex":"e815f31300","size":5,"soft_fail":false,"target_va":5376341760,"va":5375034342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2331700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375033264,"raw_hex":"e834942300","size":5,"soft_fail":false,"target_va":5377366224,"va":5375034519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%r14","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5375033264,"raw_hex":"41ffd6","size":3,"soft_fail":false,"va":5375034837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":449,"record_offset":12076,"parent_record_offset":7488,"ancestor_record_offset":7488,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":449,"record_offset":12352,"parent_record_offset":7488,"ancestor_record_offset":7488,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::frame_constants::RenderViewIndex::sun_cascade
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/frame_constants.rs
//     evidence={"module":449,"record_offset":12316,"parent_record_offset":7488,"ancestor_record_offset":7488,"inlinee":25794,"source_line":76,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_list::DrawListItem::to_draw_packet
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_list.rs
//     evidence={"module":449,"record_offset":12432,"parent_record_offset":7488,"ancestor_record_offset":7488,"inlinee":21984,"source_line":32,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":449,"record_offset":12200,"parent_record_offset":7488,"ancestor_record_offset":7488,"inlinee":7327,"source_line":314,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::framebuffer_binding::DepthAttachment::clear
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":449,"record_offset":12172,"parent_record_offset":7488,"ancestor_record_offset":7488,"inlinee":25789,"source_line":45,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":449,"record_offset":10308,"parent_record_offset":7488,"ancestor_record_offset":7488,"inlinee":9929,"source_line":41,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":449,"record_offset":9276,"parent_record_offset":7488,"ancestor_record_offset":7488,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":449,"record_offset":7688,"parent_record_offset":7488,"ancestor_record_offset":7488,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":449,"record_offset":8264,"parent_record_offset":7488,"ancestor_record_offset":7488,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_depth_compare_mode
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":449,"record_offset":12992,"parent_record_offset":7488,"ancestor_record_offset":7488,"inlinee":25795,"source_line":615,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":449,"record_offset":10368,"parent_record_offset":10340,"ancestor_record_offset":7488,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":449,"record_offset":11880,"parent_record_offset":11856,"ancestor_record_offset":7488,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":449,"record_offset":11856,"parent_record_offset":11832,"ancestor_record_offset":7488,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":449,"record_offset":11804,"parent_record_offset":11772,"ancestor_record_offset":7488,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":449,"record_offset":11748,"parent_record_offset":11356,"ancestor_record_offset":7488,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":449,"record_offset":11772,"parent_record_offset":11748,"ancestor_record_offset":7488,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":449,"record_offset":11388,"parent_record_offset":11356,"ancestor_record_offset":7488,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":449,"record_offset":11832,"parent_record_offset":11772,"ancestor_record_offset":7488,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":449,"record_offset":11436,"parent_record_offset":11412,"ancestor_record_offset":7488,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":449,"record_offset":11412,"parent_record_offset":11388,"ancestor_record_offset":7488,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":449,"record_offset":11528,"parent_record_offset":11464,"ancestor_record_offset":7488,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":449,"record_offset":11616,"parent_record_offset":11592,"ancestor_record_offset":7488,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":449,"record_offset":11948,"parent_record_offset":11924,"ancestor_record_offset":7488,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":449,"record_offset":11924,"parent_record_offset":11356,"ancestor_record_offset":7488,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":449,"record_offset":10340,"parent_record_offset":10308,"ancestor_record_offset":7488,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":449,"record_offset":11356,"parent_record_offset":10340,"ancestor_record_offset":7488,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":449,"record_offset":11088,"parent_record_offset":11064,"ancestor_record_offset":7488,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":449,"record_offset":9660,"parent_record_offset":9632,"ancestor_record_offset":7488,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":449,"record_offset":12816,"parent_record_offset":12788,"ancestor_record_offset":7488,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":449,"record_offset":9712,"parent_record_offset":9688,"ancestor_record_offset":7488,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":449,"record_offset":9688,"parent_record_offset":9660,"ancestor_record_offset":7488,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":449,"record_offset":9600,"parent_record_offset":9572,"ancestor_record_offset":7488,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":449,"record_offset":9632,"parent_record_offset":9600,"ancestor_record_offset":7488,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":449,"record_offset":9804,"parent_record_offset":9632,"ancestor_record_offset":7488,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":449,"record_offset":9916,"parent_record_offset":9892,"ancestor_record_offset":7488,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":449,"record_offset":11136,"parent_record_offset":11112,"ancestor_record_offset":7488,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":449,"record_offset":12132,"parent_record_offset":12104,"ancestor_record_offset":7488,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":449,"record_offset":11064,"parent_record_offset":11040,"ancestor_record_offset":7488,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":449,"record_offset":12228,"parent_record_offset":12200,"ancestor_record_offset":7488,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":449,"record_offset":10556,"parent_record_offset":10532,"ancestor_record_offset":7488,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":449,"record_offset":10640,"parent_record_offset":10616,"ancestor_record_offset":7488,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":449,"record_offset":10580,"parent_record_offset":10556,"ancestor_record_offset":7488,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":449,"record_offset":12104,"parent_record_offset":12076,"ancestor_record_offset":7488,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":449,"record_offset":9736,"parent_record_offset":9712,"ancestor_record_offset":7488,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":449,"record_offset":7900,"parent_record_offset":7804,"ancestor_record_offset":7488,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":449,"record_offset":10752,"parent_record_offset":10728,"ancestor_record_offset":7488,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":449,"record_offset":9496,"parent_record_offset":9468,"ancestor_record_offset":7488,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":449,"record_offset":11324,"parent_record_offset":10368,"ancestor_record_offset":7488,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":449,"record_offset":8860,"parent_record_offset":8400,"ancestor_record_offset":7488,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":449,"record_offset":9544,"parent_record_offset":9372,"ancestor_record_offset":7488,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":449,"record_offset":12044,"parent_record_offset":10308,"ancestor_record_offset":7488,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":449,"record_offset":8212,"parent_record_offset":8184,"ancestor_record_offset":7488,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":449,"record_offset":9868,"parent_record_offset":9572,"ancestor_record_offset":7488,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":449,"record_offset":10976,"parent_record_offset":10952,"ancestor_record_offset":7488,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":449,"record_offset":10896,"parent_record_offset":10872,"ancestor_record_offset":7488,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":449,"record_offset":8636,"parent_record_offset":8612,"ancestor_record_offset":7488,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":449,"record_offset":8612,"parent_record_offset":8588,"ancestor_record_offset":7488,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":449,"record_offset":8820,"parent_record_offset":8788,"ancestor_record_offset":7488,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":449,"record_offset":9760,"parent_record_offset":9736,"ancestor_record_offset":7488,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":449,"record_offset":10268,"parent_record_offset":9340,"ancestor_record_offset":7488,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":449,"record_offset":7836,"parent_record_offset":7804,"ancestor_record_offset":7488,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":449,"record_offset":7864,"parent_record_offset":7836,"ancestor_record_offset":7488,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":449,"record_offset":11672,"parent_record_offset":11640,"ancestor_record_offset":7488,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":449,"record_offset":8588,"parent_record_offset":8564,"ancestor_record_offset":7488,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":449,"record_offset":8916,"parent_record_offset":8892,"ancestor_record_offset":7488,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":449,"record_offset":8564,"parent_record_offset":8456,"ancestor_record_offset":7488,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":449,"record_offset":7804,"parent_record_offset":7776,"ancestor_record_offset":7488,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":449,"record_offset":8296,"parent_record_offset":8264,"ancestor_record_offset":7488,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":449,"record_offset":8428,"parent_record_offset":8400,"ancestor_record_offset":7488,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":449,"record_offset":8400,"parent_record_offset":8368,"ancestor_record_offset":7488,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":449,"record_offset":8344,"parent_record_offset":8320,"ancestor_record_offset":7488,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":449,"record_offset":8368,"parent_record_offset":8344,"ancestor_record_offset":7488,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":449,"record_offset":8320,"parent_record_offset":8296,"ancestor_record_offset":7488,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":449,"record_offset":8892,"parent_record_offset":8368,"ancestor_record_offset":7488,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":449,"record_offset":8000,"parent_record_offset":7968,"ancestor_record_offset":7488,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":449,"record_offset":8160,"parent_record_offset":8000,"ancestor_record_offset":7488,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":449,"record_offset":7968,"parent_record_offset":7944,"ancestor_record_offset":7488,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":449,"record_offset":7720,"parent_record_offset":7688,"ancestor_record_offset":7488,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":449,"record_offset":8184,"parent_record_offset":8160,"ancestor_record_offset":7488,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":449,"record_offset":7776,"parent_record_offset":7744,"ancestor_record_offset":7488,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":449,"record_offset":7744,"parent_record_offset":7720,"ancestor_record_offset":7488,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":449,"record_offset":8456,"parent_record_offset":8428,"ancestor_record_offset":7488,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":449,"record_offset":8704,"parent_record_offset":8456,"ancestor_record_offset":7488,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":449,"record_offset":8676,"parent_record_offset":8456,"ancestor_record_offset":7488,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":449,"record_offset":8508,"parent_record_offset":8456,"ancestor_record_offset":7488,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":449,"record_offset":11972,"parent_record_offset":11948,"ancestor_record_offset":7488,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":449,"record_offset":11996,"parent_record_offset":11972,"ancestor_record_offset":7488,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":449,"record_offset":11112,"parent_record_offset":11088,"ancestor_record_offset":7488,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":449,"record_offset":12456,"parent_record_offset":12432,"ancestor_record_offset":7488,"inlinee":21986,"source_line":105,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":449,"record_offset":12496,"parent_record_offset":12456,"ancestor_record_offset":7488,"inlinee":21988,"source_line":137,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_depth
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":449,"record_offset":12276,"parent_record_offset":12252,"ancestor_record_offset":7488,"inlinee":25792,"source_line":157,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$6::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":449,"record_offset":12252,"parent_record_offset":12228,"ancestor_record_offset":7488,"inlinee":25791,"source_line":185,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":449,"record_offset":11040,"parent_record_offset":11016,"ancestor_record_offset":7488,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":449,"record_offset":10392,"parent_record_offset":10368,"ancestor_record_offset":7488,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":449,"record_offset":10416,"parent_record_offset":10392,"ancestor_record_offset":7488,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":449,"record_offset":11016,"parent_record_offset":10820,"ancestor_record_offset":7488,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":449,"record_offset":10928,"parent_record_offset":10844,"ancestor_record_offset":7488,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":449,"record_offset":10872,"parent_record_offset":10844,"ancestor_record_offset":7488,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":449,"record_offset":11272,"parent_record_offset":10492,"ancestor_record_offset":7488,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":449,"record_offset":10844,"parent_record_offset":10820,"ancestor_record_offset":7488,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":449,"record_offset":10468,"parent_record_offset":10444,"ancestor_record_offset":7488,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":449,"record_offset":10444,"parent_record_offset":10416,"ancestor_record_offset":7488,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":449,"record_offset":10492,"parent_record_offset":10468,"ancestor_record_offset":7488,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":449,"record_offset":10700,"parent_record_offset":10672,"ancestor_record_offset":7488,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":449,"record_offset":10728,"parent_record_offset":10700,"ancestor_record_offset":7488,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":449,"record_offset":10788,"parent_record_offset":10672,"ancestor_record_offset":7488,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":449,"record_offset":10672,"parent_record_offset":10492,"ancestor_record_offset":7488,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":449,"record_offset":10820,"parent_record_offset":10492,"ancestor_record_offset":7488,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":449,"record_offset":10532,"parent_record_offset":10492,"ancestor_record_offset":7488,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":449,"record_offset":10616,"parent_record_offset":10492,"ancestor_record_offset":7488,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":449,"record_offset":11188,"parent_record_offset":10492,"ancestor_record_offset":7488,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":449,"record_offset":13040,"parent_record_offset":13016,"ancestor_record_offset":7488,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":449,"record_offset":11640,"parent_record_offset":11616,"ancestor_record_offset":7488,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":449,"record_offset":9308,"parent_record_offset":9276,"ancestor_record_offset":7488,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":449,"record_offset":9408,"parent_record_offset":9372,"ancestor_record_offset":7488,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":449,"record_offset":9572,"parent_record_offset":9372,"ancestor_record_offset":7488,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":449,"record_offset":9840,"parent_record_offset":9572,"ancestor_record_offset":7488,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_compare_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":449,"record_offset":13016,"parent_record_offset":12992,"ancestor_record_offset":7488,"inlinee":25796,"source_line":615,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":449,"record_offset":12524,"parent_record_offset":12496,"ancestor_record_offset":7488,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":449,"record_offset":12580,"parent_record_offset":12552,"ancestor_record_offset":7488,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":449,"record_offset":12668,"parent_record_offset":12636,"ancestor_record_offset":7488,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":449,"record_offset":12788,"parent_record_offset":12636,"ancestor_record_offset":7488,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":449,"record_offset":12552,"parent_record_offset":12524,"ancestor_record_offset":7488,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":449,"record_offset":9436,"parent_record_offset":9408,"ancestor_record_offset":7488,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":449,"record_offset":9468,"parent_record_offset":9436,"ancestor_record_offset":7488,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":449,"record_offset":9372,"parent_record_offset":9340,"ancestor_record_offset":7488,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":449,"record_offset":9340,"parent_record_offset":9308,"ancestor_record_offset":7488,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00449.jsonl.gz"}
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
// [Shader 映射] {"shader_id":"4afcfca80d551058c6504da1","relative_path":"_shadow_mask_mean.cs.hlsl_cs_main_0.bin","sha256":"53adb086ca7c166ca6b23f8821d2f6474ef0190333a9d71231f8352df04e6136","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"shadow_pass","prefix_candidates":["shadow_pass","shadow_"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _shadow_mask_mean.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[256,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"frame_constants","storage_class":"Uniform","type":{"id":34,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,4]},"decorations":[],"pointee":{"id":4,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[28,28,28,28,28,28,28,28,18,17,17,18,29,18,17,30,17,30,18,30,18,28,12]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":28,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":28,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":28,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":28,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":28,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":28,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":28,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":28,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":18,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":17,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":17,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":18,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":29,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":18,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":17,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":30,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":17,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":30,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":18,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":30,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":18,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"debug_tweakers","type_id":28,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"view_constants","type_id":12,"decorations":[{"decoration":"Offset","operands":[224]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"shadow_mask_histogram","storage_class":"Uniform","type":{"id":35,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,7]},"decorations":[],"pointee":{"id":7,"name":"type.RWStructuredBuffer.uint","record":{"opcode":"OpTypeStruct","operands":[13]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":13,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":11,"name":"shadow_mask_mean","storage_class":"Uniform","type":{"id":38,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,10]},"decorations":[],"pointee":{"id":10,"name":"type.RWStructuredBuffer.float","record":{"opcode":"OpTypeStruct","operands":[14]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":14,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":39,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,18]},"decorations":[],"pointee":{"id":18,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3138468,"position_among_mapped_render_targets":26,"mapped_render_target_count":70,"previous_callsite":3138439,"next_callsite":3138490,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/gpu_generate_draw_lists.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/terrain_lighting_texture.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: shadow_pass.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::shadow_pass::shadow_pass
// 信息 ID: 2fe869dd83f07e64a51a38af
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fe3a4 -> 0x607fb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
