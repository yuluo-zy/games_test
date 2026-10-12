// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/draw_passes/wireframe_pass.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 4c8c5bb54a849f81a98552eebba1b2b8421fc90d9371e64eb11139da7be02114
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wireframe_pass.rs.pdb.jsonl.gz
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
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::wireframe_pass::wireframe_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":530,"record_offset":96012,"rva":10275616,"va":5378984736}
// 编译实例 RVAs: 0x9ccb20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: wireframe_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9384373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378984736,"raw_hex":"e84bce70ff","size":5,"soft_fail":false,"target_va":5369600800,"va":5378985168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=19 candidates; complete list: wireframe_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9368818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378984736,"raw_hex":"e80e0b71ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5378984925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408923c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wireframe_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1288959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378984736,"raw_hex":"e80155ecff","size":5,"soft_fail":false,"target_va":5377696704,"va":5378985658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14090b460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wireframe_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-792930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378984736,"raw_hex":"e89ee6f3ff","size":5,"soft_fail":false,"target_va":5378192480,"va":5378985405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be6750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wireframe_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2202252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378984736,"raw_hex":"e88c9a2100","size":5,"soft_fail":false,"target_va":5381187408,"va":5378985151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: wireframe_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32861713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378984736,"raw_hex":"e8116ef501","size":5,"soft_fail":false,"target_va":5411847168,"va":5378985450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32556185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378984736,"raw_hex":"e899c4f001","size":5,"soft_fail":false,"target_va":5411541776,"va":5378985586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-815558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378984736,"raw_hex":"e83a8ef3ff","size":5,"soft_fail":false,"target_va":5378169824,"va":5378985377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1462855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378984736,"raw_hex":"e8b9ade9ff","size":5,"soft_fail":false,"target_va":5377522032,"va":5378984882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1460938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378984736,"raw_hex":"e836b5e9ff","size":5,"soft_fail":false,"target_va":5377524592,"va":5378985525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2133045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378984736,"raw_hex":"e8358c2000","size":5,"soft_fail":false,"target_va":5381117840,"va":5378984790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32551534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378984736,"raw_hex":"e86eb2f001","size":5,"soft_fail":false,"target_va":5411537104,"va":5378985565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rsi","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5378984736,"raw_hex":"ffd6","size":2,"soft_fail":false,"va":5378985475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":530,"record_offset":97288,"parent_record_offset":96012,"ancestor_record_offset":96012,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":530,"record_offset":97548,"parent_record_offset":96012,"ancestor_record_offset":96012,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_list::DrawListItem::to_draw_packet
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_list.rs
//     evidence={"module":530,"record_offset":97624,"parent_record_offset":96012,"ancestor_record_offset":96012,"inlinee":25983,"source_line":32,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":530,"record_offset":96200,"parent_record_offset":96012,"ancestor_record_offset":96012,"inlinee":8275,"source_line":337,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::common_draw_params::CommonDrawParams::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs
//     evidence={"module":530,"record_offset":96376,"parent_record_offset":96012,"ancestor_record_offset":96012,"inlinee":25981,"source_line":26,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":530,"record_offset":98216,"parent_record_offset":96012,"ancestor_record_offset":96012,"inlinee":8285,"source_line":628,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_polygon_mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":530,"record_offset":98132,"parent_record_offset":96012,"ancestor_record_offset":96012,"inlinee":30329,"source_line":638,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":530,"record_offset":96872,"parent_record_offset":96848,"ancestor_record_offset":96012,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":530,"record_offset":96920,"parent_record_offset":96896,"ancestor_record_offset":96012,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":530,"record_offset":96896,"parent_record_offset":96872,"ancestor_record_offset":96012,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":530,"record_offset":96848,"parent_record_offset":96824,"ancestor_record_offset":96012,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":530,"record_offset":96824,"parent_record_offset":96784,"ancestor_record_offset":96012,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":530,"record_offset":96784,"parent_record_offset":96760,"ancestor_record_offset":96012,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":530,"record_offset":97172,"parent_record_offset":97148,"ancestor_record_offset":96012,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":530,"record_offset":97148,"parent_record_offset":97124,"ancestor_record_offset":96012,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":530,"record_offset":97096,"parent_record_offset":97064,"ancestor_record_offset":96012,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":530,"record_offset":97040,"parent_record_offset":96504,"ancestor_record_offset":96012,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":530,"record_offset":97064,"parent_record_offset":97040,"ancestor_record_offset":96012,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":530,"record_offset":96532,"parent_record_offset":96504,"ancestor_record_offset":96012,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":530,"record_offset":97124,"parent_record_offset":97064,"ancestor_record_offset":96012,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":530,"record_offset":96580,"parent_record_offset":96556,"ancestor_record_offset":96012,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":530,"record_offset":96556,"parent_record_offset":96532,"ancestor_record_offset":96012,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":530,"record_offset":96760,"parent_record_offset":96736,"ancestor_record_offset":96012,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":530,"record_offset":96464,"parent_record_offset":96436,"ancestor_record_offset":96012,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":530,"record_offset":96436,"parent_record_offset":96408,"ancestor_record_offset":96012,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":530,"record_offset":96504,"parent_record_offset":96436,"ancestor_record_offset":96012,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":530,"record_offset":97956,"parent_record_offset":97928,"ancestor_record_offset":96012,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":530,"record_offset":98008,"parent_record_offset":97984,"ancestor_record_offset":96012,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":530,"record_offset":97984,"parent_record_offset":97928,"ancestor_record_offset":96012,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":530,"record_offset":96232,"parent_record_offset":96200,"ancestor_record_offset":96012,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":530,"record_offset":98032,"parent_record_offset":98008,"ancestor_record_offset":96012,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":530,"record_offset":96672,"parent_record_offset":96608,"ancestor_record_offset":96012,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":530,"record_offset":96696,"parent_record_offset":96672,"ancestor_record_offset":96012,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":530,"record_offset":96736,"parent_record_offset":96556,"ancestor_record_offset":96012,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":530,"record_offset":97224,"parent_record_offset":96408,"ancestor_record_offset":96012,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":530,"record_offset":97900,"parent_record_offset":97868,"ancestor_record_offset":96012,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":530,"record_offset":97508,"parent_record_offset":97484,"ancestor_record_offset":96012,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":530,"record_offset":97588,"parent_record_offset":97548,"ancestor_record_offset":96012,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":530,"record_offset":98056,"parent_record_offset":98032,"ancestor_record_offset":96012,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":530,"record_offset":96608,"parent_record_offset":96580,"ancestor_record_offset":96012,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":530,"record_offset":96988,"parent_record_offset":96964,"ancestor_record_offset":96012,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":530,"record_offset":96644,"parent_record_offset":96608,"ancestor_record_offset":96012,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":530,"record_offset":96964,"parent_record_offset":96784,"ancestor_record_offset":96012,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":530,"record_offset":97484,"parent_record_offset":97312,"ancestor_record_offset":96012,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::as_secs_f32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":530,"record_offset":97256,"parent_record_offset":96376,"ancestor_record_offset":96012,"inlinee":11237,"source_line":853,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":530,"record_offset":97656,"parent_record_offset":97624,"ancestor_record_offset":96012,"inlinee":25985,"source_line":106,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":530,"record_offset":97704,"parent_record_offset":97656,"ancestor_record_offset":96012,"inlinee":25987,"source_line":138,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::ColorAttachment::desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":530,"record_offset":96304,"parent_record_offset":96280,"ancestor_record_offset":96012,"inlinee":25998,"source_line":89,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":530,"record_offset":96280,"parent_record_offset":96256,"ancestor_record_offset":96012,"inlinee":8279,"source_line":145,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$5::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":530,"record_offset":96256,"parent_record_offset":96232,"ancestor_record_offset":96012,"inlinee":8277,"source_line":179,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":530,"record_offset":96408,"parent_record_offset":96376,"ancestor_record_offset":96012,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":530,"record_offset":98180,"parent_record_offset":98156,"ancestor_record_offset":96012,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":530,"record_offset":98240,"parent_record_offset":98216,"ancestor_record_offset":96012,"inlinee":8286,"source_line":628,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_polygon_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":530,"record_offset":98156,"parent_record_offset":98132,"ancestor_record_offset":96012,"inlinee":30330,"source_line":638,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":530,"record_offset":96328,"parent_record_offset":96304,"ancestor_record_offset":96012,"inlinee":8273,"source_line":315,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":530,"record_offset":97736,"parent_record_offset":97704,"ancestor_record_offset":96012,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":530,"record_offset":97800,"parent_record_offset":97768,"ancestor_record_offset":96012,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":530,"record_offset":97928,"parent_record_offset":97868,"ancestor_record_offset":96012,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":530,"record_offset":97768,"parent_record_offset":97736,"ancestor_record_offset":96012,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":530,"record_offset":97868,"parent_record_offset":97832,"ancestor_record_offset":96012,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":530,"record_offset":97832,"parent_record_offset":97800,"ancestor_record_offset":96012,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00530.jsonl.gz"}
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
// 完整结构及原始类型记录: wireframe_pass.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wireframe_pass.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/wireframe_pass.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: wireframe_pass.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":1449168,"caller_aliases":{"alias_group_rva":1449168,"count":1,"display_examples":["rhapsody::rg::state::RenderGraphState::compile"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":1455033,"position_among_mapped_render_targets":17,"mapped_render_target_count":25,"previous_callsite":1454981,"next_callsite":1455076,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/feedback_report_bg.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/ui_pass.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: wireframe_pass.rs.enriched.json.gz
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
// 函数: country_core::systems::render_loop::draw_passes::wireframe_pass::wireframe_pass
// 信息 ID: 458238402b1d586877a7159a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x1633b9 -> 0x9ccb20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
