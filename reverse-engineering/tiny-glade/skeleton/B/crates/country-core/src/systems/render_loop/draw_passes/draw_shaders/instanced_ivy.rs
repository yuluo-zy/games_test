// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/instanced_ivy.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 9a91e851f2457b1bfe3543972009d766091185f54f24ce5477273d214a1f0598
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: instanced_ivy.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 497
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::draw_instanced_ivy_branch_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":497,"record_offset":23844,"rva":9288784,"va":5377997904}
// 编译实例 RVAs: 0x8dbc50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=19 candidates; complete list: instanced_ivy.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8381868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377997904,"raw_hex":"e8541a80ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5377997975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: instanced_ivy.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-297522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377997904,"raw_hex":"e8ce75fbff","size":5,"soft_fail":false,"target_va":5377701312,"va":5377998829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908610
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: instanced_ivy.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t182082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377997904,"raw_hex":"e842c70200","size":5,"soft_fail":false,"target_va":5378180624,"va":5377998537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908c50
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: instanced_ivy.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t183920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377997904,"raw_hex":"e870ce0200","size":5,"soft_fail":false,"target_va":5378182224,"va":5377998299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141630490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_ivy.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13977318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377997904,"raw_hex":"e8e646d500","size":5,"soft_fail":false,"target_va":5391975568,"va":5377998245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: instanced_ivy.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33848947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377997904,"raw_hex":"e8737e0402","size":5,"soft_fail":false,"target_va":5411847168,"va":5377998216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33543007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377997904,"raw_hex":"e85fd3ff01","size":5,"soft_fail":false,"target_va":5411541776,"va":5377998764,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_instanced_w_gl_draw_flag
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t177407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377997904,"raw_hex":"e8ffb40200","size":5,"soft_fail":false,"target_va":5378176144,"va":5377998732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_instanced
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":497,"record_offset":26300,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":27909,"source_line":298,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":497,"record_offset":26124,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::ivy::ivy_storage::IvyStorage::total_segment_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":497,"record_offset":24892,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":26836,"source_line":189,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":497,"record_offset":26000,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":497,"record_offset":24056,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":24508,"parent_record_offset":24484,"ancestor_record_offset":23844,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":24556,"parent_record_offset":24532,"ancestor_record_offset":23844,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":24532,"parent_record_offset":24508,"ancestor_record_offset":23844,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":24484,"parent_record_offset":24460,"ancestor_record_offset":23844,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":24460,"parent_record_offset":24420,"ancestor_record_offset":23844,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":24420,"parent_record_offset":24396,"ancestor_record_offset":23844,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":497,"record_offset":24808,"parent_record_offset":24784,"ancestor_record_offset":23844,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":497,"record_offset":24784,"parent_record_offset":24760,"ancestor_record_offset":23844,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":497,"record_offset":24732,"parent_record_offset":24700,"ancestor_record_offset":23844,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":497,"record_offset":24676,"parent_record_offset":24148,"ancestor_record_offset":23844,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":497,"record_offset":24700,"parent_record_offset":24676,"ancestor_record_offset":23844,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":497,"record_offset":24176,"parent_record_offset":24148,"ancestor_record_offset":23844,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":497,"record_offset":24760,"parent_record_offset":24700,"ancestor_record_offset":23844,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":497,"record_offset":24224,"parent_record_offset":24200,"ancestor_record_offset":23844,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":497,"record_offset":24200,"parent_record_offset":24176,"ancestor_record_offset":23844,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":497,"record_offset":24396,"parent_record_offset":24372,"ancestor_record_offset":23844,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":497,"record_offset":24112,"parent_record_offset":24084,"ancestor_record_offset":23844,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":497,"record_offset":24084,"parent_record_offset":24056,"ancestor_record_offset":23844,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":497,"record_offset":24148,"parent_record_offset":24084,"ancestor_record_offset":23844,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":497,"record_offset":25104,"parent_record_offset":25076,"ancestor_record_offset":23844,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":497,"record_offset":25164,"parent_record_offset":25132,"ancestor_record_offset":23844,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":497,"record_offset":25704,"parent_record_offset":25652,"ancestor_record_offset":23844,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":497,"record_offset":25500,"parent_record_offset":25476,"ancestor_record_offset":23844,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":497,"record_offset":25476,"parent_record_offset":25452,"ancestor_record_offset":23844,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":497,"record_offset":24312,"parent_record_offset":24252,"ancestor_record_offset":23844,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":497,"record_offset":24332,"parent_record_offset":24312,"ancestor_record_offset":23844,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":497,"record_offset":24372,"parent_record_offset":24200,"ancestor_record_offset":23844,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":497,"record_offset":24860,"parent_record_offset":24056,"ancestor_record_offset":23844,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":497,"record_offset":25200,"parent_record_offset":25048,"ancestor_record_offset":23844,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":497,"record_offset":25596,"parent_record_offset":25572,"ancestor_record_offset":23844,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":497,"record_offset":24252,"parent_record_offset":24224,"ancestor_record_offset":23844,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":497,"record_offset":24624,"parent_record_offset":24600,"ancestor_record_offset":23844,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":497,"record_offset":24284,"parent_record_offset":24252,"ancestor_record_offset":23844,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":497,"record_offset":24600,"parent_record_offset":24420,"ancestor_record_offset":23844,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":497,"record_offset":26032,"parent_record_offset":26000,"ancestor_record_offset":23844,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::ivy::ivy_storage::Ivy::point_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":497,"record_offset":25676,"parent_record_offset":25652,"ancestor_record_offset":23844,"inlinee":26766,"source_line":57,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::ivy::ivy_storage::impl$2::total_segment_count::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":497,"record_offset":25652,"parent_record_offset":25628,"ancestor_record_offset":23844,"inlinee":26765,"source_line":192,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":497,"record_offset":25416,"parent_record_offset":25380,"ancestor_record_offset":23844,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":497,"record_offset":25452,"parent_record_offset":25416,"ancestor_record_offset":23844,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":497,"record_offset":25540,"parent_record_offset":25380,"ancestor_record_offset":23844,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":497,"record_offset":25380,"parent_record_offset":25320,"ancestor_record_offset":23844,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":497,"record_offset":25076,"parent_record_offset":25048,"ancestor_record_offset":23844,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":497,"record_offset":25132,"parent_record_offset":25076,"ancestor_record_offset":23844,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":497,"record_offset":25796,"parent_record_offset":25320,"ancestor_record_offset":23844,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":497,"record_offset":24940,"parent_record_offset":24916,"ancestor_record_offset":23844,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::fold
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":497,"record_offset":25272,"parent_record_offset":25248,"ancestor_record_offset":23844,"inlinee":18642,"source_line":3186,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::fold::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":497,"record_offset":25628,"parent_record_offset":25320,"ancestor_record_offset":23844,"inlinee":21980,"source_line":3191,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":497,"record_offset":25572,"parent_record_offset":25320,"ancestor_record_offset":23844,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::fold_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":497,"record_offset":25320,"parent_record_offset":25296,"ancestor_record_offset":23844,"inlinee":10108,"source_line":3514,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":497,"record_offset":25048,"parent_record_offset":24988,"ancestor_record_offset":23844,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":497,"record_offset":24964,"parent_record_offset":24940,"ancestor_record_offset":23844,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":497,"record_offset":25020,"parent_record_offset":24988,"ancestor_record_offset":23844,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":497,"record_offset":24988,"parent_record_offset":24964,"ancestor_record_offset":23844,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::fold
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":497,"record_offset":25296,"parent_record_offset":25272,"ancestor_record_offset":23844,"inlinee":18112,"source_line":3685,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":497,"record_offset":24916,"parent_record_offset":24892,"ancestor_record_offset":23844,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":497,"record_offset":25248,"parent_record_offset":24892,"ancestor_record_offset":23844,"inlinee":18748,"source_line":2404,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::draw_instanced_ivy_leaf_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":497,"record_offset":26332,"rva":9290112,"va":5377999232}
// 编译实例 RVAs: 0x8dc180
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::ivy_leaf_shader
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t43","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5377999232,"raw_hex":"e92b000000","size":5,"soft_fail":false,"target_va":5377999296,"va":5377999248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::draw_instanced_ivy_flower_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":497,"record_offset":26516,"rva":9290144,"va":5377999264}
// 编译实例 RVAs: 0x8dc1a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::ivy_leaf_shader
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t11","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5377999264,"raw_hex":"e90b000000","size":5,"soft_fail":false,"target_va":5377999296,"va":5377999280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::ivy_leaf_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":497,"record_offset":26704,"rva":9290176,"va":5377999296}
// 编译实例 RVAs: 0x8dc1c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=3
//     target_aliases=19 candidates; complete list: instanced_ivy.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8383289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377999296,"raw_hex":"e8c71480ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5377999396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: instanced_ivy.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-299590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377999296,"raw_hex":"e8ba6dfbff","size":5,"soft_fail":false,"target_va":5377701312,"va":5378000897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_ivy.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t180755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377999296,"raw_hex":"e813c20200","size":5,"soft_fail":false,"target_va":5378180624,"va":5377999864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908c50
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: instanced_ivy.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t182580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377999296,"raw_hex":"e834c90200","size":5,"soft_fail":false,"target_va":5378182224,"va":5377999639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140969720
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: instanced_ivy.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t578026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377999296,"raw_hex":"e8ead10800","size":5,"soft_fail":false,"target_va":5378578208,"va":5378000177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b660
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_ivy.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t586642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377999296,"raw_hex":"e892f30800","size":5,"soft_fail":false,"target_va":5378586208,"va":5377999561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_ivy.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t586764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377999296,"raw_hex":"e80cf40800","size":5,"soft_fail":false,"target_va":5378586704,"va":5377999935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162da90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_ivy.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13964459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377999296,"raw_hex":"e8ab14d500","size":5,"soft_fail":false,"target_va":5391964816,"va":5378000352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162f590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_ivy.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13971701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377999296,"raw_hex":"e8f530d500","size":5,"soft_fail":false,"target_va":5391971728,"va":5378000022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: instanced_ivy.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33846894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377999296,"raw_hex":"e86e760402","size":5,"soft_fail":false,"target_va":5411847152,"va":5378000253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=12
//     target_aliases=1 candidates; complete list: instanced_ivy.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33847806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377999296,"raw_hex":"e8fe790402","size":5,"soft_fail":false,"target_va":5411847168,"va":5377999357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t33540959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377999296,"raw_hex":"e85fcbff01","size":5,"soft_fail":false,"target_va":5411541776,"va":5378000812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33481693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377999296,"raw_hex":"e8dde3fe01","size":5,"soft_fail":false,"target_va":5411482433,"va":5378000735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_instanced_w_gl_draw_flag
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t175477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377999296,"raw_hex":"e875ad0200","size":5,"soft_fail":false,"target_va":5378176144,"va":5378000662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::camera::CameraMatrices::eye_position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3910328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377999296,"raw_hex":"e8b8aa3b00","size":5,"soft_fail":false,"target_va":5381909824,"va":5377999491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5377999296,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5378000284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":497,"record_offset":29204,"parent_record_offset":26704,"ancestor_record_offset":26704,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":497,"record_offset":28964,"parent_record_offset":26704,"ancestor_record_offset":26704,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_instanced
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":497,"record_offset":30336,"parent_record_offset":26704,"ancestor_record_offset":26704,"inlinee":27909,"source_line":298,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":497,"record_offset":27956,"parent_record_offset":26704,"ancestor_record_offset":26704,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":497,"record_offset":27748,"parent_record_offset":26704,"ancestor_record_offset":26704,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":497,"record_offset":27824,"parent_record_offset":26704,"ancestor_record_offset":26704,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":497,"record_offset":26908,"parent_record_offset":26704,"ancestor_record_offset":26704,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":27364,"parent_record_offset":27340,"ancestor_record_offset":26704,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":27412,"parent_record_offset":27388,"ancestor_record_offset":26704,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":27388,"parent_record_offset":27364,"ancestor_record_offset":26704,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":27340,"parent_record_offset":27316,"ancestor_record_offset":26704,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":27316,"parent_record_offset":27276,"ancestor_record_offset":26704,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":27276,"parent_record_offset":27252,"ancestor_record_offset":26704,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":497,"record_offset":27664,"parent_record_offset":27640,"ancestor_record_offset":26704,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":497,"record_offset":27640,"parent_record_offset":27616,"ancestor_record_offset":26704,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":497,"record_offset":27588,"parent_record_offset":27556,"ancestor_record_offset":26704,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":497,"record_offset":27532,"parent_record_offset":27000,"ancestor_record_offset":26704,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":497,"record_offset":27556,"parent_record_offset":27532,"ancestor_record_offset":26704,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":497,"record_offset":27028,"parent_record_offset":27000,"ancestor_record_offset":26704,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":497,"record_offset":27616,"parent_record_offset":27556,"ancestor_record_offset":26704,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":497,"record_offset":27076,"parent_record_offset":27052,"ancestor_record_offset":26704,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":497,"record_offset":27052,"parent_record_offset":27028,"ancestor_record_offset":26704,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":497,"record_offset":27252,"parent_record_offset":27228,"ancestor_record_offset":26704,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":497,"record_offset":26964,"parent_record_offset":26936,"ancestor_record_offset":26704,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":497,"record_offset":26936,"parent_record_offset":26908,"ancestor_record_offset":26704,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":497,"record_offset":27000,"parent_record_offset":26936,"ancestor_record_offset":26704,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":497,"record_offset":29032,"parent_record_offset":28992,"ancestor_record_offset":26704,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":497,"record_offset":27164,"parent_record_offset":27104,"ancestor_record_offset":26704,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":497,"record_offset":27792,"parent_record_offset":27748,"ancestor_record_offset":26704,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":497,"record_offset":27188,"parent_record_offset":27164,"ancestor_record_offset":26704,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":497,"record_offset":27228,"parent_record_offset":27052,"ancestor_record_offset":26704,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":497,"record_offset":27716,"parent_record_offset":26908,"ancestor_record_offset":26704,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":497,"record_offset":28184,"parent_record_offset":28156,"ancestor_record_offset":26704,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":497,"record_offset":29116,"parent_record_offset":29088,"ancestor_record_offset":26704,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":497,"record_offset":29144,"parent_record_offset":29116,"ancestor_record_offset":26704,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":497,"record_offset":27104,"parent_record_offset":27076,"ancestor_record_offset":26704,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":497,"record_offset":27480,"parent_record_offset":27456,"ancestor_record_offset":26704,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":497,"record_offset":27136,"parent_record_offset":27104,"ancestor_record_offset":26704,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":497,"record_offset":27456,"parent_record_offset":27276,"ancestor_record_offset":26704,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":497,"record_offset":29060,"parent_record_offset":29032,"ancestor_record_offset":26704,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":497,"record_offset":28992,"parent_record_offset":28964,"ancestor_record_offset":26704,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":497,"record_offset":27856,"parent_record_offset":27824,"ancestor_record_offset":26704,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00497.jsonl.gz"}
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
// 完整结构及原始类型记录: instanced_ivy.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: instanced_ivy.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/instanced_ivy.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: instanced_ivy.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::draw_instanced_ivy_branch_shader
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":497,"record_offset":26000,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00497.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":497,"record_offset":26032,"parent_record_offset":26000,"ancestor_record_offset":23844,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00497.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":497,"record_offset":26124,"parent_record_offset":23844,"ancestor_record_offset":23844,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00497.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::ivy_leaf_shader
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::uniform","relationship":"pdb_inline_direct","evidence":{"module":497,"record_offset":27748,"parent_record_offset":26704,"ancestor_record_offset":26704,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00497.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":497,"record_offset":27824,"parent_record_offset":26704,"ancestor_record_offset":26704,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00497.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":497,"record_offset":27856,"parent_record_offset":27824,"ancestor_record_offset":26704,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00497.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":497,"record_offset":27956,"parent_record_offset":26704,"ancestor_record_offset":26704,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00497.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":9290112,"caller_aliases":{"alias_group_rva":9290112,"count":1,"display_examples":["country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::draw_instanced_ivy_leaf_shader"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":9290128,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_jump_outside_procedure"}
// [父函数静态调用位置] {"caller_entry_rva":9290144,"caller_aliases":{"alias_group_rva":9290144,"count":1,"display_examples":["country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::draw_instanced_ivy_flower_shader"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":9290160,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_jump_outside_procedure"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: instanced_ivy.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::draw_instanced_ivy_branch_shader
// 信息 ID: 62844e2604a3ad6275d41232
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::draw_instanced_ivy_flower_shader
// 信息 ID: 0d8f0de73106608137a505aa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::draw_instanced_ivy_leaf_shader
// 信息 ID: 9339387ade404766de81a903
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::ivy_leaf_shader
// 信息 ID: e29d06752cf7f87ef7790809
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::draw_instanced_ivy_leaf_shader @ 0x8dc190 -> 0x8dc1c0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_ivy::draw_instanced_ivy_flower_shader @ 0x8dc1b0 -> 0x8dc1c0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
