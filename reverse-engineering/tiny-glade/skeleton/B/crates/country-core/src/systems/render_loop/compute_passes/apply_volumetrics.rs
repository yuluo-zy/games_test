// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/compute_passes/apply_volumetrics.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 9b97a11316baa2e3be2aa84178da87dd166934565de664e12d1b4674275de801
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: apply_volumetrics.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 487
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
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::apply_volumetrics::apply_volumetrics
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":487,"record_offset":35432,"rva":9061568,"va":5377770688}
// 编译实例 RVAs: 0x8a44c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=19 candidates; complete list: apply_volumetrics.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8154667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377770688,"raw_hex":"e8d59183ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5377770774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140896fb0
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: apply_volumetrics.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-55968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377770688,"raw_hex":"e86025ffff","size":5,"soft_fail":false,"target_va":5377716144,"va":5377772107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408edd00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_volumetrics.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t300561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377770688,"raw_hex":"e811960400","size":5,"soft_fail":false,"target_va":5378071808,"va":5377771242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408ee240
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_volumetrics.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t301830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377770688,"raw_hex":"e8069b0400","size":5,"soft_fail":false,"target_va":5378073152,"va":5377771317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140905260
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_volumetrics.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t396298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377770688,"raw_hex":"e80a0c0600","size":5,"soft_fail":false,"target_va":5378167392,"va":5377771089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409056d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_volumetrics.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t397350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377770688,"raw_hex":"e826100600","size":5,"soft_fail":false,"target_va":5378168528,"va":5377771173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_volumetrics.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t814575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377770688,"raw_hex":"e8ef6d0c00","size":5,"soft_fail":false,"target_va":5378585968,"va":5377771388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_volumetrics.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t815689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377770688,"raw_hex":"e849720c00","size":5,"soft_fail":false,"target_va":5378586704,"va":5377771010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096ba30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: apply_volumetrics.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t815720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377770688,"raw_hex":"e868720c00","size":5,"soft_fail":false,"target_va":5378587184,"va":5377771459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409dd4d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_volumetrics.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1280858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377770688,"raw_hex":"e85a8b1300","size":5,"soft_fail":false,"target_va":5379052752,"va":5377771889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: apply_volumetrics.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3398437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377770688,"raw_hex":"e825db3300","size":5,"soft_fail":false,"target_va":5381170352,"va":5377771910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416032a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: apply_volumetrics.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14019783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377770688,"raw_hex":"e8c7ecd500","size":5,"soft_fail":false,"target_va":5391790752,"va":5377770964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: apply_volumetrics.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34076125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377770688,"raw_hex":"e8ddf50702","size":5,"soft_fail":false,"target_va":5411847168,"va":5377771038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33769817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377770688,"raw_hex":"e859490302","size":5,"soft_fail":false,"target_va":5411541776,"va":5377771954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3346973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377770688,"raw_hex":"e81d123300","size":5,"soft_fail":false,"target_va":5381117840,"va":5377770862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::bind_resources::impl$0::bind_shadowmap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/bind_resources.rs
//     evidence={"module":487,"record_offset":36524,"parent_record_offset":35432,"ancestor_record_offset":35432,"inlinee":27050,"source_line":50,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":487,"record_offset":37056,"parent_record_offset":35432,"ancestor_record_offset":35432,"inlinee":27051,"source_line":460,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":487,"record_offset":36732,"parent_record_offset":35432,"ancestor_record_offset":35432,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":487,"record_offset":36684,"parent_record_offset":35432,"ancestor_record_offset":35432,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":487,"record_offset":36828,"parent_record_offset":35432,"ancestor_record_offset":35432,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::camera_rig::MainCamera::mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig.rs
//     evidence={"module":487,"record_offset":36496,"parent_record_offset":35432,"ancestor_record_offset":35432,"inlinee":27049,"source_line":432,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":487,"record_offset":35632,"parent_record_offset":35432,"ancestor_record_offset":35432,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":487,"record_offset":36468,"parent_record_offset":35432,"ancestor_record_offset":35432,"inlinee":8273,"source_line":315,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":487,"record_offset":36084,"parent_record_offset":36060,"ancestor_record_offset":35432,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":487,"record_offset":36132,"parent_record_offset":36108,"ancestor_record_offset":35432,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":487,"record_offset":36108,"parent_record_offset":36084,"ancestor_record_offset":35432,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":487,"record_offset":36060,"parent_record_offset":36036,"ancestor_record_offset":35432,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":487,"record_offset":36036,"parent_record_offset":35996,"ancestor_record_offset":35432,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":487,"record_offset":35996,"parent_record_offset":35972,"ancestor_record_offset":35432,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":487,"record_offset":36384,"parent_record_offset":36360,"ancestor_record_offset":35432,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":487,"record_offset":36360,"parent_record_offset":36336,"ancestor_record_offset":35432,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":487,"record_offset":36308,"parent_record_offset":36276,"ancestor_record_offset":35432,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":487,"record_offset":36252,"parent_record_offset":35724,"ancestor_record_offset":35432,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":487,"record_offset":36276,"parent_record_offset":36252,"ancestor_record_offset":35432,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":487,"record_offset":35752,"parent_record_offset":35724,"ancestor_record_offset":35432,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":487,"record_offset":36336,"parent_record_offset":36276,"ancestor_record_offset":35432,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":487,"record_offset":35800,"parent_record_offset":35776,"ancestor_record_offset":35432,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":487,"record_offset":35776,"parent_record_offset":35752,"ancestor_record_offset":35432,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":487,"record_offset":35972,"parent_record_offset":35948,"ancestor_record_offset":35432,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":487,"record_offset":35688,"parent_record_offset":35660,"ancestor_record_offset":35432,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":487,"record_offset":35660,"parent_record_offset":35632,"ancestor_record_offset":35432,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":487,"record_offset":35724,"parent_record_offset":35660,"ancestor_record_offset":35432,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":487,"record_offset":35888,"parent_record_offset":35828,"ancestor_record_offset":35432,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":487,"record_offset":36600,"parent_record_offset":36556,"ancestor_record_offset":35432,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":487,"record_offset":35908,"parent_record_offset":35888,"ancestor_record_offset":35432,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":487,"record_offset":35948,"parent_record_offset":35776,"ancestor_record_offset":35432,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":487,"record_offset":36436,"parent_record_offset":35632,"ancestor_record_offset":35432,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":487,"record_offset":37156,"parent_record_offset":37084,"ancestor_record_offset":35432,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":487,"record_offset":35828,"parent_record_offset":35800,"ancestor_record_offset":35432,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":487,"record_offset":36200,"parent_record_offset":36176,"ancestor_record_offset":35432,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":487,"record_offset":35860,"parent_record_offset":35828,"ancestor_record_offset":35432,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":487,"record_offset":36176,"parent_record_offset":35996,"ancestor_record_offset":35432,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":487,"record_offset":37084,"parent_record_offset":37056,"ancestor_record_offset":35432,"inlinee":26676,"source_line":485,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":487,"record_offset":36632,"parent_record_offset":36524,"ancestor_record_offset":35432,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":487,"record_offset":36556,"parent_record_offset":36524,"ancestor_record_offset":35432,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00487.jsonl.gz"}
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
// 完整结构及原始类型记录: apply_volumetrics.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: apply_volumetrics.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/apply_volumetrics.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: apply_volumetrics.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::apply_volumetrics::apply_volumetrics
// [资源接口动作；资源实例 unknown] {"role":"named_shadowmap_binding_api","symbol":"country_core::render::pass_builder::bind_resources::impl$0::bind_shadowmap","relationship":"pdb_inline_direct","evidence":{"module":487,"record_offset":36524,"parent_record_offset":35432,"ancestor_record_offset":35432,"inlinee":27050,"source_line":50,"archive":"_pdb/B/modules/00487.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::uniform","relationship":"pdb_inline_transitive","evidence":{"module":487,"record_offset":36556,"parent_record_offset":36524,"ancestor_record_offset":35432,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00487.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":487,"record_offset":36632,"parent_record_offset":36524,"ancestor_record_offset":35432,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00487.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":487,"record_offset":36684,"parent_record_offset":35432,"ancestor_record_offset":35432,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00487.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":487,"record_offset":36732,"parent_record_offset":35432,"ancestor_record_offset":35432,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00487.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::uniform","relationship":"pdb_inline_direct","evidence":{"module":487,"record_offset":36828,"parent_record_offset":35432,"ancestor_record_offset":35432,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00487.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [Shader 映射] {"shader_id":"b429e628cc0ec869f06c5f4e","relative_path":"_apply_volumetrics.cs.hlsl_cs_main_0.bin","sha256":"e41a602c3717ed0ff6ff4d12f45e950ad66d0ac63e047a9d333c481bd3c02474","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"apply_volumetrics","prefix_candidates":["apply_volumetrics","volumetric_","apply_volumetrics"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _apply_volumetrics.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":29,"name":"sampler_shadow_linear","storage_class":"UniformConstant","type":{"id":114,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,28]},"decorations":[],"pointee":{"id":28,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[12]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":32,"name":"frame_constants","storage_class":"Uniform","type":{"id":117,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,30]},"decorations":[],"pointee":{"id":30,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[97,97,97,97,97,97,97,97,53,51,51,53,75,53,51,59,51,59,53,59,53,51,53,53,53,97,43]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":97,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":97,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":97,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":97,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":97,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":97,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":97,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":97,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":53,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":51,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":51,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":53,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":75,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":53,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":51,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":59,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":51,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":59,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":53,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":59,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":53,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":51,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":53,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":53,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":53,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":97,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":43,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":35,"name":"$Globals","storage_class":"Uniform","type":{"id":118,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,34]},"decorations":[],"pointee":{"id":34,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[53,75,51,51,75,75]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"current_view_index","type_id":53,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"depth_tex_dims","type_id":75,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"global_density_scale","type_id":51,"decorations":[{"decoration":"Offset","operands":[12]}]},{"index":3,"name":"global_size_scale","type_id":51,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":4,"name":"blue_noise_texture_dims","type_id":75,"decorations":[{"decoration":"Offset","operands":[20]}]},{"index":5,"name":"color_tex_dims","type_id":75,"decorations":[{"decoration":"Offset","operands":[32]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":37,"name":"shadow_map","storage_class":"UniformConstant","type":{"id":119,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,36]},"decorations":[],"pointee":{"id":36,"name":"type.2d.image.array","record":{"opcode":"OpTypeImage","operands":[51,1,2,1,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":38,"name":"depth_tex","storage_class":"UniformConstant","type":{"id":120,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,33]},"decorations":[],"pointee":{"id":33,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[51,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":39,"name":"blue_noise_texture","storage_class":"UniformConstant","type":{"id":120,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,33]},"decorations":[],"pointee":{"id":33,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[51,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":17,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[17]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":41,"name":"color_tex","storage_class":"UniformConstant","type":{"id":121,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,40]},"decorations":[],"pointee":{"id":40,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[51,1,2,0,0,2,8]},"decorations":[]}},"descriptor_set":0,"binding":18,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[18]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":123,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,122]},"decorations":[],"pointee":{"id":122,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[53,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":5161232,"caller_aliases":{"alias_group_rva":5161232,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5164046,"position_among_mapped_render_targets":52,"mapped_render_target_count":59,"previous_callsite":5163992,"next_callsite":5164070,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/apply_screen_shadows.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/dof.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: apply_volumetrics.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::apply_volumetrics::apply_volumetrics
// 信息 ID: bf4e723f7886a56de7c0a14c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ecc0e -> 0x8a44c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
