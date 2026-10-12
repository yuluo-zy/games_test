// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/draw_passes/transparency_pass.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 36419f48ed0b4433975b54418429a8e047efda39622c7b2f87ab8be75c7e555b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: transparency_pass.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 529
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
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
// 函数符号: country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":529,"record_offset":31364,"rva":10220320,"va":5378929440}
// 编译实例 RVAs: 0x9bf320
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9329140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378929440,"raw_hex":"e80ca671ff","size":5,"soft_fail":false,"target_va":5369600800,"va":5378929935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=19 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9313565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378929440,"raw_hex":"e8e3e271ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5378929672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408923c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1233887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378929440,"raw_hex":"e8212cedff","size":5,"soft_fail":false,"target_va":5377696704,"va":5378930586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1229343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378929440,"raw_hex":"e8e13dedff","size":5,"soft_fail":false,"target_va":5377701312,"va":5378930650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408ee800
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-855621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378929440,"raw_hex":"e8bbf1f2ff","size":5,"soft_fail":false,"target_va":5378074624,"va":5378930240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14090b460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-737704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378929440,"raw_hex":"e858bef4ff","size":5,"soft_fail":false,"target_va":5378192480,"va":5378930179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be6750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2257484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378929440,"raw_hex":"e84c722200","size":5,"soft_fail":false,"target_va":5381187408,"va":5378929919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32916899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378929440,"raw_hex":"e8a345f601","size":5,"soft_fail":false,"target_va":5411847168,"va":5378930264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32611255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378929440,"raw_hex":"e8b79bf101","size":5,"soft_fail":false,"target_va":5411541776,"va":5378930516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-760328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378929440,"raw_hex":"e8f865f4ff","size":5,"soft_fail":false,"target_va":5378169824,"va":5378930147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1407602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378929440,"raw_hex":"e88e85eaff","size":5,"soft_fail":false,"target_va":5377522032,"va":5378929629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1405868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378929440,"raw_hex":"e8548ceaff","size":5,"soft_fail":false,"target_va":5377524592,"va":5378930455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2188320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378929440,"raw_hex":"e820642100","size":5,"soft_fail":false,"target_va":5381117840,"va":5378929515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32606604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378929440,"raw_hex":"e88c89f101","size":5,"soft_fail":false,"target_va":5411537104,"va":5378930495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%r9","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5378929440,"raw_hex":"41ffd1","size":3,"soft_fail":false,"va":5378930400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":32596,"parent_record_offset":31364,"ancestor_record_offset":31364,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":529,"record_offset":32856,"parent_record_offset":31364,"ancestor_record_offset":31364,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":529,"record_offset":33440,"parent_record_offset":31364,"ancestor_record_offset":31364,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_list::DrawListItem::to_draw_packet
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_list.rs
//     evidence={"module":529,"record_offset":32932,"parent_record_offset":31364,"ancestor_record_offset":31364,"inlinee":25983,"source_line":32,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":529,"record_offset":31560,"parent_record_offset":31364,"ancestor_record_offset":31364,"inlinee":8275,"source_line":337,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::common_draw_params::CommonDrawParams::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs
//     evidence={"module":529,"record_offset":31680,"parent_record_offset":31364,"ancestor_record_offset":31364,"inlinee":25981,"source_line":26,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_depth_write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":529,"record_offset":33484,"parent_record_offset":31364,"ancestor_record_offset":31364,"inlinee":30288,"source_line":627,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":529,"record_offset":32180,"parent_record_offset":32156,"ancestor_record_offset":31364,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":529,"record_offset":32228,"parent_record_offset":32204,"ancestor_record_offset":31364,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":529,"record_offset":32204,"parent_record_offset":32180,"ancestor_record_offset":31364,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":32156,"parent_record_offset":32132,"ancestor_record_offset":31364,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":32132,"parent_record_offset":32092,"ancestor_record_offset":31364,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":32092,"parent_record_offset":32068,"ancestor_record_offset":31364,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":529,"record_offset":32480,"parent_record_offset":32456,"ancestor_record_offset":31364,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":529,"record_offset":32456,"parent_record_offset":32432,"ancestor_record_offset":31364,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":529,"record_offset":32404,"parent_record_offset":32372,"ancestor_record_offset":31364,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":529,"record_offset":32348,"parent_record_offset":31812,"ancestor_record_offset":31364,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":529,"record_offset":32372,"parent_record_offset":32348,"ancestor_record_offset":31364,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":529,"record_offset":31840,"parent_record_offset":31812,"ancestor_record_offset":31364,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":529,"record_offset":32432,"parent_record_offset":32372,"ancestor_record_offset":31364,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":529,"record_offset":31888,"parent_record_offset":31864,"ancestor_record_offset":31364,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":529,"record_offset":31864,"parent_record_offset":31840,"ancestor_record_offset":31364,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":529,"record_offset":32068,"parent_record_offset":32044,"ancestor_record_offset":31364,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":529,"record_offset":31772,"parent_record_offset":31744,"ancestor_record_offset":31364,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":529,"record_offset":31744,"parent_record_offset":31712,"ancestor_record_offset":31364,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":529,"record_offset":31812,"parent_record_offset":31744,"ancestor_record_offset":31364,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":529,"record_offset":33264,"parent_record_offset":33236,"ancestor_record_offset":31364,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":529,"record_offset":33316,"parent_record_offset":33292,"ancestor_record_offset":31364,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":529,"record_offset":33292,"parent_record_offset":33236,"ancestor_record_offset":31364,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":529,"record_offset":31592,"parent_record_offset":31560,"ancestor_record_offset":31364,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":529,"record_offset":33340,"parent_record_offset":33316,"ancestor_record_offset":31364,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":529,"record_offset":31980,"parent_record_offset":31916,"ancestor_record_offset":31364,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":529,"record_offset":32004,"parent_record_offset":31980,"ancestor_record_offset":31364,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":529,"record_offset":32044,"parent_record_offset":31864,"ancestor_record_offset":31364,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":529,"record_offset":32532,"parent_record_offset":31712,"ancestor_record_offset":31364,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":529,"record_offset":33208,"parent_record_offset":33176,"ancestor_record_offset":31364,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":529,"record_offset":32816,"parent_record_offset":32792,"ancestor_record_offset":31364,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":529,"record_offset":32896,"parent_record_offset":32856,"ancestor_record_offset":31364,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":529,"record_offset":33364,"parent_record_offset":33340,"ancestor_record_offset":31364,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":529,"record_offset":31916,"parent_record_offset":31888,"ancestor_record_offset":31364,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":529,"record_offset":32296,"parent_record_offset":32272,"ancestor_record_offset":31364,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":529,"record_offset":31952,"parent_record_offset":31916,"ancestor_record_offset":31364,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":529,"record_offset":32272,"parent_record_offset":32092,"ancestor_record_offset":31364,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":529,"record_offset":32792,"parent_record_offset":32620,"ancestor_record_offset":31364,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::as_secs_f32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":529,"record_offset":32564,"parent_record_offset":31680,"ancestor_record_offset":31364,"inlinee":11237,"source_line":853,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":529,"record_offset":32964,"parent_record_offset":32932,"ancestor_record_offset":31364,"inlinee":25985,"source_line":106,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":529,"record_offset":33012,"parent_record_offset":32964,"ancestor_record_offset":31364,"inlinee":25987,"source_line":138,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color_depth
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":529,"record_offset":31640,"parent_record_offset":31616,"ancestor_record_offset":31364,"inlinee":25979,"source_line":132,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$4::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":529,"record_offset":31616,"parent_record_offset":31592,"ancestor_record_offset":31364,"inlinee":25978,"source_line":171,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":529,"record_offset":31712,"parent_record_offset":31680,"ancestor_record_offset":31364,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":529,"record_offset":33532,"parent_record_offset":33508,"ancestor_record_offset":31364,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_write_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":529,"record_offset":33508,"parent_record_offset":33484,"ancestor_record_offset":31364,"inlinee":30289,"source_line":627,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":529,"record_offset":33044,"parent_record_offset":33012,"ancestor_record_offset":31364,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":529,"record_offset":33108,"parent_record_offset":33076,"ancestor_record_offset":31364,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":529,"record_offset":33236,"parent_record_offset":33176,"ancestor_record_offset":31364,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":529,"record_offset":33076,"parent_record_offset":33044,"ancestor_record_offset":31364,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":529,"record_offset":33176,"parent_record_offset":33140,"ancestor_record_offset":31364,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":529,"record_offset":33140,"parent_record_offset":33108,"ancestor_record_offset":31364,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":529,"record_offset":33572,"rva":10221568,"va":5378930688}
// 编译实例 RVAs: 0x9bf800
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9330677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378930688,"raw_hex":"e80ba071ff","size":5,"soft_fail":false,"target_va":5369600800,"va":5378931472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=19 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9315087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378930688,"raw_hex":"e8f1dc71ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5378931194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408923c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1237874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378930688,"raw_hex":"e88e1cedff","size":5,"soft_fail":false,"target_va":5377696704,"va":5378934573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1233327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378930688,"raw_hex":"e8512eedff","size":5,"soft_fail":false,"target_va":5377701312,"va":5378934634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140896fb0
//     owner=unresolved_namespace; count=10
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1217768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378930688,"raw_hex":"e8186bedff","size":5,"soft_fail":false,"target_va":5377716144,"va":5378933907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408edd00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-861881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378930688,"raw_hex":"e847d9f2ff","size":5,"soft_fail":false,"target_va":5378071808,"va":5378933684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408ee240
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-859807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378930688,"raw_hex":"e861e1f2ff","size":5,"soft_fail":false,"target_va":5378073152,"va":5378932954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408ee800
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-857149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378930688,"raw_hex":"e8c3ebf2ff","size":5,"soft_fail":false,"target_va":5378074624,"va":5378931768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140905260
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-765965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378930688,"raw_hex":"e8f34ff4ff","size":5,"soft_fail":false,"target_va":5378167392,"va":5378933352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409056d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-764759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378930688,"raw_hex":"e8a954f4ff","size":5,"soft_fail":false,"target_va":5378168528,"va":5378933282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14090b460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-739237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378930688,"raw_hex":"e85bb8f4ff","size":5,"soft_fail":false,"target_va":5378192480,"va":5378931712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b570
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-345949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378930688,"raw_hex":"e8a3b8faff","size":5,"soft_fail":false,"target_va":5378585968,"va":5378931912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409dd4d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t119981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378930688,"raw_hex":"e8add40100","size":5,"soft_fail":false,"target_va":5379052752,"va":5378932766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6d00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2192527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378930688,"raw_hex":"e88f742100","size":5,"soft_fail":false,"target_va":5381123328,"va":5378930796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2237560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378930688,"raw_hex":"e878242200","size":5,"soft_fail":false,"target_va":5381170352,"va":5378932787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be6750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2255948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378930688,"raw_hex":"e84c6c2200","size":5,"soft_fail":false,"target_va":5381187408,"va":5378931455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415fc230
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12828336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378930688,"raw_hex":"e8b0bec300","size":5,"soft_fail":false,"target_va":5391761968,"va":5378933627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416032a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12858340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378930688,"raw_hex":"e8e433c400","size":5,"soft_fail":false,"target_va":5391790752,"va":5378932407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=16
//     target_aliases=1 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32915378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378930688,"raw_hex":"e8b23ff601","size":5,"soft_fail":false,"target_va":5411847168,"va":5378931785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32607987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378930688,"raw_hex":"e8f38ef101","size":5,"soft_fail":false,"target_va":5411541776,"va":5378933784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_indirect_raw
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-334688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378930688,"raw_hex":"e8a0e4faff","size":5,"soft_fail":false,"target_va":5378599040,"va":5378933723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::read_rg_storage_buffer
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-337354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378930688,"raw_hex":"e836dafaff","size":5,"soft_fail":false,"target_va":5378595520,"va":5378932869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-336252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378930688,"raw_hex":"e884defaff","size":5,"soft_fail":false,"target_va":5378596192,"va":5378932439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-761865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378930688,"raw_hex":"e8f75ff4ff","size":5,"soft_fail":false,"target_va":5378169824,"va":5378931684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::write_rg_storage_buffer
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-759150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378930688,"raw_hex":"e8926af4ff","size":5,"soft_fail":false,"target_va":5378172896,"va":5378932041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1409124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378930688,"raw_hex":"e89c7feaff","size":5,"soft_fail":false,"target_va":5377522032,"va":5378931151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1407759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378930688,"raw_hex":"e8f184eaff","size":5,"soft_fail":false,"target_va":5377524592,"va":5378932346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::clear_rg_buffer
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t123633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378930688,"raw_hex":"e8f1e20100","size":5,"soft_fail":false,"target_va":5379054512,"va":5378930874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::BufferDesc::new_gpu_only
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2184656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378930688,"raw_hex":"e8d0552100","size":5,"soft_fail":false,"target_va":5381115424,"va":5378930763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2186952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378930688,"raw_hex":"e8c85e2100","size":5,"soft_fail":false,"target_va":5381117840,"va":5378930883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2268215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378930688,"raw_hex":"e8379c2200","size":5,"soft_fail":false,"target_va":5381199056,"va":5378930836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32603336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378930688,"raw_hex":"e8c87cf101","size":5,"soft_fail":false,"target_va":5411537104,"va":5378933763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%r9","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5378930688,"raw_hex":"41ffd1","size":3,"soft_fail":false,"va":5378932288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":35432,"parent_record_offset":33572,"ancestor_record_offset":33572,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::product
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":529,"record_offset":33956,"parent_record_offset":33572,"ancestor_record_offset":33572,"inlinee":12554,"source_line":3553,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":529,"record_offset":35692,"parent_record_offset":33572,"ancestor_record_offset":33572,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_1d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":529,"record_offset":36612,"parent_record_offset":33572,"ancestor_record_offset":33572,"inlinee":26675,"source_line":448,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":529,"record_offset":37480,"parent_record_offset":33572,"ancestor_record_offset":33572,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":529,"record_offset":36948,"parent_record_offset":33572,"ancestor_record_offset":33572,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":529,"record_offset":37044,"parent_record_offset":33572,"ancestor_record_offset":33572,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":529,"record_offset":36276,"parent_record_offset":33572,"ancestor_record_offset":33572,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":529,"record_offset":36364,"parent_record_offset":33572,"ancestor_record_offset":33572,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_list::DrawListItem::to_draw_packet
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_list.rs
//     evidence={"module":529,"record_offset":35768,"parent_record_offset":33572,"ancestor_record_offset":33572,"inlinee":25983,"source_line":32,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":529,"record_offset":34392,"parent_record_offset":33572,"ancestor_record_offset":33572,"inlinee":8275,"source_line":337,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::common_draw_params::CommonDrawParams::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs
//     evidence={"module":529,"record_offset":34516,"parent_record_offset":33572,"ancestor_record_offset":33572,"inlinee":25981,"source_line":26,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":529,"record_offset":37216,"parent_record_offset":33572,"ancestor_record_offset":33572,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$1::bind_terrain_lighting
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":529,"record_offset":37348,"parent_record_offset":33572,"ancestor_record_offset":33572,"inlinee":30287,"source_line":30,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":529,"record_offset":36524,"parent_record_offset":33572,"ancestor_record_offset":33572,"inlinee":8289,"source_line":637,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":529,"record_offset":33808,"parent_record_offset":33572,"ancestor_record_offset":33572,"inlinee":9694,"source_line":34,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":529,"record_offset":33928,"parent_record_offset":33572,"ancestor_record_offset":33572,"inlinee":8273,"source_line":315,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":529,"record_offset":35016,"parent_record_offset":34992,"ancestor_record_offset":33572,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":529,"record_offset":35064,"parent_record_offset":35040,"ancestor_record_offset":33572,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":529,"record_offset":35040,"parent_record_offset":35016,"ancestor_record_offset":33572,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":34992,"parent_record_offset":34968,"ancestor_record_offset":33572,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":34968,"parent_record_offset":34928,"ancestor_record_offset":33572,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":34928,"parent_record_offset":34904,"ancestor_record_offset":33572,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":529,"record_offset":35316,"parent_record_offset":35292,"ancestor_record_offset":33572,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":529,"record_offset":35292,"parent_record_offset":35268,"ancestor_record_offset":33572,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":529,"record_offset":35240,"parent_record_offset":35208,"ancestor_record_offset":33572,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":529,"record_offset":35184,"parent_record_offset":34648,"ancestor_record_offset":33572,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":529,"record_offset":35208,"parent_record_offset":35184,"ancestor_record_offset":33572,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":529,"record_offset":34676,"parent_record_offset":34648,"ancestor_record_offset":33572,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":529,"record_offset":35268,"parent_record_offset":35208,"ancestor_record_offset":33572,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":529,"record_offset":34724,"parent_record_offset":34700,"ancestor_record_offset":33572,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":529,"record_offset":34700,"parent_record_offset":34676,"ancestor_record_offset":33572,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":529,"record_offset":34904,"parent_record_offset":34880,"ancestor_record_offset":33572,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":529,"record_offset":34608,"parent_record_offset":34580,"ancestor_record_offset":33572,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":529,"record_offset":34580,"parent_record_offset":34548,"ancestor_record_offset":33572,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":529,"record_offset":34648,"parent_record_offset":34580,"ancestor_record_offset":33572,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":529,"record_offset":34004,"parent_record_offset":33980,"ancestor_record_offset":33572,"inlinee":9468,"source_line":258,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":529,"record_offset":34100,"parent_record_offset":34076,"ancestor_record_offset":33572,"inlinee":9474,"source_line":263,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":529,"record_offset":36100,"parent_record_offset":36072,"ancestor_record_offset":33572,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":529,"record_offset":36152,"parent_record_offset":36128,"ancestor_record_offset":33572,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":529,"record_offset":36128,"parent_record_offset":36072,"ancestor_record_offset":33572,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":529,"record_offset":33868,"parent_record_offset":33808,"ancestor_record_offset":33572,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::by_ref_sized::impl$0::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\by_ref_sized.rs
//     evidence={"module":529,"record_offset":34028,"parent_record_offset":34004,"ancestor_record_offset":33572,"inlinee":9470,"source_line":40,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$37::product
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":529,"record_offset":33980,"parent_record_offset":33956,"ancestor_record_offset":33572,"inlinee":9487,"source_line":60,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$37::product::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":529,"record_offset":34208,"parent_record_offset":34100,"ancestor_record_offset":33572,"inlinee":9476,"source_line":64,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":529,"record_offset":34052,"parent_record_offset":34028,"ancestor_record_offset":33572,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":529,"record_offset":34124,"parent_record_offset":34100,"ancestor_record_offset":33572,"inlinee":4566,"source_line":720,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":529,"record_offset":36176,"parent_record_offset":36152,"ancestor_record_offset":33572,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":529,"record_offset":34816,"parent_record_offset":34752,"ancestor_record_offset":33572,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$0::wrap_mut_2::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\try_trait.rs
//     evidence={"module":529,"record_offset":34076,"parent_record_offset":34052,"ancestor_record_offset":33572,"inlinee":9472,"source_line":395,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":529,"record_offset":36404,"parent_record_offset":36364,"ancestor_record_offset":33572,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":529,"record_offset":34840,"parent_record_offset":34816,"ancestor_record_offset":33572,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":529,"record_offset":34880,"parent_record_offset":34700,"ancestor_record_offset":33572,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":529,"record_offset":35368,"parent_record_offset":34548,"ancestor_record_offset":33572,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":529,"record_offset":36044,"parent_record_offset":36012,"ancestor_record_offset":33572,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":529,"record_offset":34148,"parent_record_offset":34124,"ancestor_record_offset":33572,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":529,"record_offset":36716,"parent_record_offset":36644,"ancestor_record_offset":33572,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":529,"record_offset":35652,"parent_record_offset":35628,"ancestor_record_offset":33572,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":529,"record_offset":35732,"parent_record_offset":35692,"ancestor_record_offset":33572,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":529,"record_offset":34172,"parent_record_offset":34148,"ancestor_record_offset":33572,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":529,"record_offset":36200,"parent_record_offset":36176,"ancestor_record_offset":33572,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":529,"record_offset":34752,"parent_record_offset":34724,"ancestor_record_offset":33572,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":529,"record_offset":35132,"parent_record_offset":35108,"ancestor_record_offset":33572,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":529,"record_offset":34788,"parent_record_offset":34752,"ancestor_record_offset":33572,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":529,"record_offset":35108,"parent_record_offset":34928,"ancestor_record_offset":33572,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":529,"record_offset":35628,"parent_record_offset":35456,"ancestor_record_offset":33572,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::as_secs_f32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":529,"record_offset":35400,"parent_record_offset":34516,"ancestor_record_offset":33572,"inlinee":11237,"source_line":853,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":529,"record_offset":36644,"parent_record_offset":36612,"ancestor_record_offset":33572,"inlinee":26676,"source_line":485,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":529,"record_offset":37248,"parent_record_offset":37216,"ancestor_record_offset":33572,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":529,"record_offset":35800,"parent_record_offset":35768,"ancestor_record_offset":33572,"inlinee":25985,"source_line":106,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":529,"record_offset":35848,"parent_record_offset":35800,"ancestor_record_offset":33572,"inlinee":25987,"source_line":138,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color_depth
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":529,"record_offset":34472,"parent_record_offset":34448,"ancestor_record_offset":33572,"inlinee":25979,"source_line":132,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$4::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":529,"record_offset":34448,"parent_record_offset":34424,"ancestor_record_offset":33572,"inlinee":25978,"source_line":171,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":529,"record_offset":34548,"parent_record_offset":34516,"ancestor_record_offset":33572,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":529,"record_offset":36572,"parent_record_offset":36548,"ancestor_record_offset":33572,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::impl$8::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":529,"record_offset":33844,"parent_record_offset":33808,"ancestor_record_offset":33572,"inlinee":29807,"source_line":66,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":529,"record_offset":36548,"parent_record_offset":36524,"ancestor_record_offset":33572,"inlinee":8290,"source_line":637,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::util::impl$0::same
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/util.rs
//     evidence={"module":529,"record_offset":33896,"parent_record_offset":33808,"ancestor_record_offset":33572,"inlinee":9666,"source_line":8,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":529,"record_offset":35880,"parent_record_offset":35848,"ancestor_record_offset":33572,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":529,"record_offset":35944,"parent_record_offset":35912,"ancestor_record_offset":33572,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":529,"record_offset":36072,"parent_record_offset":36012,"ancestor_record_offset":33572,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":529,"record_offset":35912,"parent_record_offset":35880,"ancestor_record_offset":33572,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":529,"record_offset":36012,"parent_record_offset":35976,"ancestor_record_offset":33572,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":529,"record_offset":35976,"parent_record_offset":35944,"ancestor_record_offset":33572,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 174
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":529,"record_offset":37532,"rva":10225872,"va":5378934992}
// 编译实例 RVAs: 0x9c08d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9336148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e8ac8a71ff","size":5,"soft_fail":false,"target_va":5369600800,"va":5378936943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=19 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9320557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e893c771ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5378936664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408923c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1250018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e81eedecff","size":5,"soft_fail":false,"target_va":5377696704,"va":5378946717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1245807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e891fdecff","size":5,"soft_fail":false,"target_va":5377701312,"va":5378947114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140896fb0
//     owner=unresolved_namespace; count=42
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1227784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e8f843edff","size":5,"soft_fail":false,"target_va":5377716144,"va":5378943923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408edd00
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-867558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e81ac3f2ff","size":5,"soft_fail":false,"target_va":5378071808,"va":5378939361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408ee240
//     owner=unresolved_namespace; count=15
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-865397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e88bcbf2ff","size":5,"soft_fail":false,"target_va":5378073152,"va":5378938544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408ee800
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-862749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e8e3d5f2ff","size":5,"soft_fail":false,"target_va":5378074624,"va":5378937368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140905260
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-771657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e8b739f4ff","size":5,"soft_fail":false,"target_va":5378167392,"va":5378939044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409056d0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-770366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e8c23ef4ff","size":5,"soft_fail":false,"target_va":5378168528,"va":5378938889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908c50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-755079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e8797af4ff","size":5,"soft_fail":false,"target_va":5378182224,"va":5378937298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14090b460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-744707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e8fda2f4ff","size":5,"soft_fail":false,"target_va":5378192480,"va":5378937182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14090d9a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-733792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e8a0cdf4ff","size":5,"soft_fail":false,"target_va":5378202016,"va":5378935803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b570
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-351616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e880a2faff","size":5,"soft_fail":false,"target_va":5378585968,"va":5378937579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-350532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e8bca6faff","size":5,"soft_fail":false,"target_va":5378586704,"va":5378937231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096d490
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-342261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e80bc7faff","size":5,"soft_fail":false,"target_va":5378593936,"va":5378936192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096f090
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-341885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e883c8faff","size":5,"soft_fail":false,"target_va":5378601104,"va":5378942984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096f650
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-334937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e8a7e3faff","size":5,"soft_fail":false,"target_va":5378602576,"va":5378937508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409dd4d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t110750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e89eb00100","size":5,"soft_fail":false,"target_va":5379052752,"va":5378941997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6ce0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2188156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e87c632100","size":5,"soft_fail":false,"target_va":5381123296,"va":5378935135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6d00
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2187733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e8d5612100","size":5,"soft_fail":false,"target_va":5381123328,"va":5378935590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2228329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e869002200","size":5,"soft_fail":false,"target_va":5381170352,"va":5378942018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be6750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2250476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e8ec562200","size":5,"soft_fail":false,"target_va":5381187408,"va":5378936927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415fc230
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12822652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e87ca8c300","size":5,"soft_fail":false,"target_va":5391761968,"va":5378939311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416032a0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12852323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e8631cc400","size":5,"soft_fail":false,"target_va":5391790752,"va":5378938424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=50
//     target_aliases=1 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32909915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e85b2af601","size":5,"soft_fail":false,"target_va":5411847168,"va":5378937248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3709624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e8b89a3800","size":5,"soft_fail":false,"target_va":5382655024,"va":5378945395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32598076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e83c68f101","size":5,"soft_fail":false,"target_va":5411541776,"va":5378943695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32599118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e84e6cf101","size":5,"soft_fail":false,"target_va":5411542880,"va":5378943757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_indirect_raw
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-340528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e8d0cdfaff","size":5,"soft_fail":false,"target_va":5378599040,"va":5378939563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::read_rg_storage_buffer
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-342944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e860c4faff","size":5,"soft_fail":false,"target_va":5378595520,"va":5378938459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-343317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e8ebc2faff","size":5,"soft_fail":false,"target_va":5378596192,"va":5378939504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-767335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e8994af4ff","size":5,"soft_fail":false,"target_va":5378169824,"va":5378937154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::write_rg_storage_buffer
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-764881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e82f54f4ff","size":5,"soft_fail":false,"target_va":5378172896,"va":5378937772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1414594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e83e6aeaff","size":5,"soft_fail":false,"target_va":5377522032,"va":5378936621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1413340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e8246feaff","size":5,"soft_fail":false,"target_va":5377524592,"va":5378937927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::clear_rg_color_image
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t116880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e890c80100","size":5,"soft_fail":false,"target_va":5379055248,"va":5378938363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::BufferDesc::new_gpu_only
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2179862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e816432100","size":5,"soft_fail":false,"target_va":5381115424,"va":5378935557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::color_attachment
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2180716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e86c462100","size":5,"soft_fail":false,"target_va":5381116880,"va":5378936159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2182779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e87b4e2100","size":5,"soft_fail":false,"target_va":5381117840,"va":5378935056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::half_res
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2179762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e8b2422100","size":5,"soft_fail":false,"target_va":5381117744,"va":5378937977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2181483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e86b492100","size":5,"soft_fail":false,"target_va":5381116576,"va":5378935088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::sampled
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2180631","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e817462100","size":5,"soft_fail":false,"target_va":5381116784,"va":5378936148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::sampled_with_different_format
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2181526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e896492100","size":5,"soft_fail":false,"target_va":5381116944,"va":5378935413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::storage
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2176102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e866342100","size":5,"soft_fail":false,"target_va":5381116816,"va":5378940709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::format
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2176715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e8cb362100","size":5,"soft_fail":false,"target_va":5381118048,"va":5378941328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2176551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e827362100","size":5,"soft_fail":false,"target_va":5381117856,"va":5378941300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::usage
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2176619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e86b362100","size":5,"soft_fail":false,"target_va":5381117984,"va":5378941360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2263876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e8448b2200","size":5,"soft_fail":false,"target_va":5381199056,"va":5378935175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32593428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378934992,"raw_hex":"e81456f101","size":5,"soft_fail":false,"target_va":5411537104,"va":5378943671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rsi","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5378934992,"raw_hex":"ffd6","size":2,"soft_fail":false,"va":5378937870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":40152,"parent_record_offset":37532,"ancestor_record_offset":37532,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::product
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":529,"record_offset":38472,"parent_record_offset":37532,"ancestor_record_offset":37532,"inlinee":12554,"source_line":3553,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":529,"record_offset":39036,"parent_record_offset":37532,"ancestor_record_offset":37532,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":529,"record_offset":40412,"parent_record_offset":37532,"ancestor_record_offset":37532,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::bind_resources::impl$0::bind_shadowmap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/bind_resources.rs
//     evidence={"module":529,"record_offset":40996,"parent_record_offset":37532,"ancestor_record_offset":37532,"inlinee":27050,"source_line":50,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":529,"record_offset":43456,"parent_record_offset":37532,"ancestor_record_offset":37532,"inlinee":26886,"source_line":452,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":529,"record_offset":42364,"parent_record_offset":37532,"ancestor_record_offset":37532,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=19
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":529,"record_offset":41784,"parent_record_offset":37532,"ancestor_record_offset":37532,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture_view
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":529,"record_offset":43316,"parent_record_offset":37532,"ancestor_record_offset":37532,"inlinee":28228,"source_line":718,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":529,"record_offset":41880,"parent_record_offset":37532,"ancestor_record_offset":37532,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":529,"record_offset":41148,"parent_record_offset":37532,"ancestor_record_offset":37532,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":529,"record_offset":41280,"parent_record_offset":37532,"ancestor_record_offset":37532,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_list::DrawListItem::to_draw_packet
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_list.rs
//     evidence={"module":529,"record_offset":40488,"parent_record_offset":37532,"ancestor_record_offset":37532,"inlinee":25983,"source_line":32,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":529,"record_offset":39112,"parent_record_offset":37532,"ancestor_record_offset":37532,"inlinee":8275,"source_line":337,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::common_draw_params::CommonDrawParams::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs
//     evidence={"module":529,"record_offset":39236,"parent_record_offset":37532,"ancestor_record_offset":37532,"inlinee":25981,"source_line":26,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":529,"record_offset":42100,"parent_record_offset":37532,"ancestor_record_offset":37532,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$1::bind_terrain_lighting
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":529,"record_offset":42232,"parent_record_offset":37532,"ancestor_record_offset":37532,"inlinee":30287,"source_line":30,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":529,"record_offset":41440,"parent_record_offset":37532,"ancestor_record_offset":37532,"inlinee":8289,"source_line":637,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":529,"record_offset":37808,"parent_record_offset":37532,"ancestor_record_offset":37532,"inlinee":9694,"source_line":34,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":529,"record_offset":37780,"parent_record_offset":37532,"ancestor_record_offset":37532,"inlinee":8273,"source_line":315,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::temporal::Temporal::into_inner
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/temporal.rs
//     evidence={"module":529,"record_offset":43240,"parent_record_offset":37532,"ancestor_record_offset":37532,"inlinee":27065,"source_line":141,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":529,"record_offset":39736,"parent_record_offset":39712,"ancestor_record_offset":37532,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":529,"record_offset":39784,"parent_record_offset":39760,"ancestor_record_offset":37532,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":529,"record_offset":39760,"parent_record_offset":39736,"ancestor_record_offset":37532,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":39712,"parent_record_offset":39688,"ancestor_record_offset":37532,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":39688,"parent_record_offset":39648,"ancestor_record_offset":37532,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":39648,"parent_record_offset":39624,"ancestor_record_offset":37532,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":529,"record_offset":40036,"parent_record_offset":40012,"ancestor_record_offset":37532,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":529,"record_offset":40012,"parent_record_offset":39988,"ancestor_record_offset":37532,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":529,"record_offset":39960,"parent_record_offset":39928,"ancestor_record_offset":37532,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":529,"record_offset":39904,"parent_record_offset":39368,"ancestor_record_offset":37532,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":529,"record_offset":39928,"parent_record_offset":39904,"ancestor_record_offset":37532,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":529,"record_offset":39396,"parent_record_offset":39368,"ancestor_record_offset":37532,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":529,"record_offset":39988,"parent_record_offset":39928,"ancestor_record_offset":37532,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":529,"record_offset":39444,"parent_record_offset":39420,"ancestor_record_offset":37532,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":529,"record_offset":39420,"parent_record_offset":39396,"ancestor_record_offset":37532,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":529,"record_offset":39624,"parent_record_offset":39600,"ancestor_record_offset":37532,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":529,"record_offset":39328,"parent_record_offset":39300,"ancestor_record_offset":37532,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":529,"record_offset":39300,"parent_record_offset":39268,"ancestor_record_offset":37532,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":529,"record_offset":39368,"parent_record_offset":39300,"ancestor_record_offset":37532,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":529,"record_offset":38520,"parent_record_offset":38496,"ancestor_record_offset":37532,"inlinee":9468,"source_line":258,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":529,"record_offset":38616,"parent_record_offset":38592,"ancestor_record_offset":37532,"inlinee":9474,"source_line":263,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":529,"record_offset":40820,"parent_record_offset":40792,"ancestor_record_offset":37532,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":529,"record_offset":40872,"parent_record_offset":40848,"ancestor_record_offset":37532,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":529,"record_offset":40848,"parent_record_offset":40792,"ancestor_record_offset":37532,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":529,"record_offset":37872,"parent_record_offset":37808,"ancestor_record_offset":37532,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::by_ref_sized::impl$0::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\by_ref_sized.rs
//     evidence={"module":529,"record_offset":38544,"parent_record_offset":38520,"ancestor_record_offset":37532,"inlinee":9470,"source_line":40,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$37::product
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":529,"record_offset":38496,"parent_record_offset":38472,"ancestor_record_offset":37532,"inlinee":9487,"source_line":60,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$37::product::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":529,"record_offset":38724,"parent_record_offset":38616,"ancestor_record_offset":37532,"inlinee":9476,"source_line":64,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":529,"record_offset":38568,"parent_record_offset":38544,"ancestor_record_offset":37532,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":529,"record_offset":38640,"parent_record_offset":38616,"ancestor_record_offset":37532,"inlinee":4566,"source_line":720,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":529,"record_offset":40896,"parent_record_offset":40872,"ancestor_record_offset":37532,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":529,"record_offset":39536,"parent_record_offset":39472,"ancestor_record_offset":37532,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$0::wrap_mut_2::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\try_trait.rs
//     evidence={"module":529,"record_offset":38592,"parent_record_offset":38568,"ancestor_record_offset":37532,"inlinee":9472,"source_line":395,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":529,"record_offset":41068,"parent_record_offset":41028,"ancestor_record_offset":37532,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":529,"record_offset":39560,"parent_record_offset":39536,"ancestor_record_offset":37532,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":529,"record_offset":39600,"parent_record_offset":39420,"ancestor_record_offset":37532,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":529,"record_offset":40088,"parent_record_offset":39268,"ancestor_record_offset":37532,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":529,"record_offset":40764,"parent_record_offset":40732,"ancestor_record_offset":37532,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":529,"record_offset":38664,"parent_record_offset":38640,"ancestor_record_offset":37532,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":529,"record_offset":39080,"parent_record_offset":39036,"ancestor_record_offset":37532,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":529,"record_offset":40372,"parent_record_offset":40348,"ancestor_record_offset":37532,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":529,"record_offset":40452,"parent_record_offset":40412,"ancestor_record_offset":37532,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":529,"record_offset":38688,"parent_record_offset":38664,"ancestor_record_offset":37532,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":529,"record_offset":40920,"parent_record_offset":40896,"ancestor_record_offset":37532,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":529,"record_offset":39472,"parent_record_offset":39444,"ancestor_record_offset":37532,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":529,"record_offset":39852,"parent_record_offset":39828,"ancestor_record_offset":37532,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":529,"record_offset":39508,"parent_record_offset":39472,"ancestor_record_offset":37532,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":529,"record_offset":39828,"parent_record_offset":39648,"ancestor_record_offset":37532,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":529,"record_offset":40348,"parent_record_offset":40176,"ancestor_record_offset":37532,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::as_secs_f32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":529,"record_offset":40120,"parent_record_offset":39236,"ancestor_record_offset":37532,"inlinee":11237,"source_line":853,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":529,"record_offset":43488,"parent_record_offset":43456,"ancestor_record_offset":37532,"inlinee":26676,"source_line":485,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":529,"record_offset":42132,"parent_record_offset":42100,"ancestor_record_offset":37532,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":529,"record_offset":41100,"parent_record_offset":40996,"ancestor_record_offset":37532,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":529,"record_offset":41028,"parent_record_offset":40996,"ancestor_record_offset":37532,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":529,"record_offset":40520,"parent_record_offset":40488,"ancestor_record_offset":37532,"inlinee":25985,"source_line":106,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":529,"record_offset":40568,"parent_record_offset":40520,"ancestor_record_offset":37532,"inlinee":25987,"source_line":138,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color_depth
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":529,"record_offset":39192,"parent_record_offset":39168,"ancestor_record_offset":37532,"inlinee":25979,"source_line":132,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$4::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":529,"record_offset":39168,"parent_record_offset":39144,"ancestor_record_offset":37532,"inlinee":25978,"source_line":171,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":529,"record_offset":39268,"parent_record_offset":39236,"ancestor_record_offset":37532,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":529,"record_offset":41488,"parent_record_offset":41464,"ancestor_record_offset":37532,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::impl$8::clone
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":529,"record_offset":38228,"parent_record_offset":38188,"ancestor_record_offset":37532,"inlinee":29807,"source_line":66,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":529,"record_offset":41464,"parent_record_offset":41440,"ancestor_record_offset":37532,"inlinee":8290,"source_line":637,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":529,"record_offset":37848,"parent_record_offset":37808,"ancestor_record_offset":37532,"inlinee":9664,"source_line":67,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::util::impl$0::same
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/util.rs
//     evidence={"module":529,"record_offset":37900,"parent_record_offset":37808,"ancestor_record_offset":37532,"inlinee":9666,"source_line":8,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":529,"record_offset":40600,"parent_record_offset":40568,"ancestor_record_offset":37532,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":529,"record_offset":40664,"parent_record_offset":40632,"ancestor_record_offset":37532,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":529,"record_offset":40792,"parent_record_offset":40732,"ancestor_record_offset":37532,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":529,"record_offset":40632,"parent_record_offset":40600,"ancestor_record_offset":37532,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":529,"record_offset":40732,"parent_record_offset":40696,"ancestor_record_offset":37532,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":529,"record_offset":40696,"parent_record_offset":40664,"ancestor_record_offset":37532,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::transparency_pass::pre_taa_transparency_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 448
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":529,"record_offset":44780,"rva":10238592,"va":5378947712}
// 编译实例 RVAs: 0x9c3a80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9347396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378947712,"raw_hex":"e8bc5e71ff","size":5,"soft_fail":false,"target_va":5369600800,"va":5378948191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=19 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9331820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378947712,"raw_hex":"e8949b71ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5378947927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408923c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1252335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378947712,"raw_hex":"e811e4ecff","size":5,"soft_fail":false,"target_va":5377696704,"va":5378949034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1247791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378947712,"raw_hex":"e8d1f5ecff","size":5,"soft_fail":false,"target_va":5377701312,"va":5378949098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408ee800
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-873894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378947712,"raw_hex":"e85aaaf2ff","size":5,"soft_fail":false,"target_va":5378074624,"va":5378948513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14090b460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-755968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378947712,"raw_hex":"e80077f4ff","size":5,"soft_fail":false,"target_va":5378192480,"va":5378948443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be6750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2239228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378947712,"raw_hex":"e8fc2a2200","size":5,"soft_fail":false,"target_va":5381187408,"va":5378948175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32898633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378947712,"raw_hex":"e849fef501","size":5,"soft_fail":false,"target_va":5411847168,"va":5378948530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32592802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378947712,"raw_hex":"e8a253f101","size":5,"soft_fail":false,"target_va":5411541776,"va":5378948969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-778588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378947712,"raw_hex":"e8a41ef4ff","size":5,"soft_fail":false,"target_va":5378169824,"va":5378948407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1425857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378947712,"raw_hex":"e83f3eeaff","size":5,"soft_fail":false,"target_va":5377522032,"va":5378947884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1424321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378947712,"raw_hex":"e83f44eaff","size":5,"soft_fail":false,"target_va":5377524592,"va":5378948908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2170069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378947712,"raw_hex":"e8d51c2100","size":5,"soft_fail":false,"target_va":5381117840,"va":5378947766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32588151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378947712,"raw_hex":"e87741f101","size":5,"soft_fail":false,"target_va":5411537104,"va":5378948948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%r9","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5378947712,"raw_hex":"41ffd1","size":3,"soft_fail":false,"va":5378948853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":46024,"parent_record_offset":44780,"ancestor_record_offset":44780,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":529,"record_offset":46284,"parent_record_offset":44780,"ancestor_record_offset":44780,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":529,"record_offset":46868,"parent_record_offset":44780,"ancestor_record_offset":44780,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_list::DrawListItem::to_draw_packet
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_list.rs
//     evidence={"module":529,"record_offset":46360,"parent_record_offset":44780,"ancestor_record_offset":44780,"inlinee":25983,"source_line":32,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":529,"record_offset":44988,"parent_record_offset":44780,"ancestor_record_offset":44780,"inlinee":8275,"source_line":337,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::common_draw_params::CommonDrawParams::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs
//     evidence={"module":529,"record_offset":45108,"parent_record_offset":44780,"ancestor_record_offset":44780,"inlinee":25981,"source_line":26,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":529,"record_offset":47084,"parent_record_offset":44780,"ancestor_record_offset":44780,"inlinee":8289,"source_line":637,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_depth_write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":529,"record_offset":47000,"parent_record_offset":44780,"ancestor_record_offset":44780,"inlinee":30288,"source_line":627,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":529,"record_offset":45608,"parent_record_offset":45584,"ancestor_record_offset":44780,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":529,"record_offset":45656,"parent_record_offset":45632,"ancestor_record_offset":44780,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":529,"record_offset":45632,"parent_record_offset":45608,"ancestor_record_offset":44780,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":45584,"parent_record_offset":45560,"ancestor_record_offset":44780,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":45560,"parent_record_offset":45520,"ancestor_record_offset":44780,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":45520,"parent_record_offset":45496,"ancestor_record_offset":44780,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":529,"record_offset":45908,"parent_record_offset":45884,"ancestor_record_offset":44780,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":529,"record_offset":45884,"parent_record_offset":45860,"ancestor_record_offset":44780,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":529,"record_offset":45832,"parent_record_offset":45800,"ancestor_record_offset":44780,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":529,"record_offset":45776,"parent_record_offset":45240,"ancestor_record_offset":44780,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":529,"record_offset":45800,"parent_record_offset":45776,"ancestor_record_offset":44780,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":529,"record_offset":45268,"parent_record_offset":45240,"ancestor_record_offset":44780,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":529,"record_offset":45860,"parent_record_offset":45800,"ancestor_record_offset":44780,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":529,"record_offset":45316,"parent_record_offset":45292,"ancestor_record_offset":44780,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":529,"record_offset":45292,"parent_record_offset":45268,"ancestor_record_offset":44780,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":529,"record_offset":45496,"parent_record_offset":45472,"ancestor_record_offset":44780,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":529,"record_offset":45200,"parent_record_offset":45172,"ancestor_record_offset":44780,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":529,"record_offset":45172,"parent_record_offset":45140,"ancestor_record_offset":44780,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":529,"record_offset":45240,"parent_record_offset":45172,"ancestor_record_offset":44780,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":529,"record_offset":46692,"parent_record_offset":46664,"ancestor_record_offset":44780,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":529,"record_offset":46744,"parent_record_offset":46720,"ancestor_record_offset":44780,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":529,"record_offset":46720,"parent_record_offset":46664,"ancestor_record_offset":44780,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":529,"record_offset":45020,"parent_record_offset":44988,"ancestor_record_offset":44780,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":529,"record_offset":46768,"parent_record_offset":46744,"ancestor_record_offset":44780,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":529,"record_offset":45408,"parent_record_offset":45344,"ancestor_record_offset":44780,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":529,"record_offset":45432,"parent_record_offset":45408,"ancestor_record_offset":44780,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":529,"record_offset":45472,"parent_record_offset":45292,"ancestor_record_offset":44780,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":529,"record_offset":45960,"parent_record_offset":45140,"ancestor_record_offset":44780,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":529,"record_offset":46636,"parent_record_offset":46604,"ancestor_record_offset":44780,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":529,"record_offset":46244,"parent_record_offset":46220,"ancestor_record_offset":44780,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":529,"record_offset":46324,"parent_record_offset":46284,"ancestor_record_offset":44780,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":529,"record_offset":46792,"parent_record_offset":46768,"ancestor_record_offset":44780,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":529,"record_offset":45344,"parent_record_offset":45316,"ancestor_record_offset":44780,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":529,"record_offset":45724,"parent_record_offset":45700,"ancestor_record_offset":44780,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":529,"record_offset":45380,"parent_record_offset":45344,"ancestor_record_offset":44780,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":529,"record_offset":45700,"parent_record_offset":45520,"ancestor_record_offset":44780,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":529,"record_offset":46220,"parent_record_offset":46048,"ancestor_record_offset":44780,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::as_secs_f32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":529,"record_offset":45992,"parent_record_offset":45108,"ancestor_record_offset":44780,"inlinee":11237,"source_line":853,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":529,"record_offset":46392,"parent_record_offset":46360,"ancestor_record_offset":44780,"inlinee":25985,"source_line":106,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":529,"record_offset":46440,"parent_record_offset":46392,"ancestor_record_offset":44780,"inlinee":25987,"source_line":138,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color_depth
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":529,"record_offset":45068,"parent_record_offset":45044,"ancestor_record_offset":44780,"inlinee":25979,"source_line":132,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$4::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":529,"record_offset":45044,"parent_record_offset":45020,"ancestor_record_offset":44780,"inlinee":25978,"source_line":171,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":529,"record_offset":45140,"parent_record_offset":45108,"ancestor_record_offset":44780,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":529,"record_offset":47048,"parent_record_offset":47024,"ancestor_record_offset":44780,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":529,"record_offset":47108,"parent_record_offset":47084,"ancestor_record_offset":44780,"inlinee":8290,"source_line":637,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_write_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":529,"record_offset":47024,"parent_record_offset":47000,"ancestor_record_offset":44780,"inlinee":30289,"source_line":627,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":529,"record_offset":46472,"parent_record_offset":46440,"ancestor_record_offset":44780,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":529,"record_offset":46536,"parent_record_offset":46504,"ancestor_record_offset":44780,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":529,"record_offset":46664,"parent_record_offset":46604,"ancestor_record_offset":44780,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":529,"record_offset":46504,"parent_record_offset":46472,"ancestor_record_offset":44780,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":529,"record_offset":46604,"parent_record_offset":46568,"ancestor_record_offset":44780,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":529,"record_offset":46568,"parent_record_offset":46536,"ancestor_record_offset":44780,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::transparency_pass::post_taa_transparency_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 494
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":529,"record_offset":47176,"rva":10240176,"va":5378949296}
// 编译实例 RVAs: 0x9c40b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9348949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378949296,"raw_hex":"e8ab5871ff","size":5,"soft_fail":false,"target_va":5369600800,"va":5378949744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=19 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9333397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378949296,"raw_hex":"e86b9571ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5378949504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408923c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1253535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378949296,"raw_hex":"e861dfecff","size":5,"soft_fail":false,"target_va":5377696704,"va":5378950234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14090b460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-757506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378949296,"raw_hex":"e8fe70f4ff","size":5,"soft_fail":false,"target_va":5378192480,"va":5378949981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be6750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2237676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378949296,"raw_hex":"e8ec242200","size":5,"soft_fail":false,"target_va":5381187408,"va":5378949727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: transparency_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32897137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378949296,"raw_hex":"e871f8f501","size":5,"soft_fail":false,"target_va":5411847168,"va":5378950026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32591609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378949296,"raw_hex":"e8f94ef101","size":5,"soft_fail":false,"target_va":5411541776,"va":5378950162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-780134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378949296,"raw_hex":"e89a18f4ff","size":5,"soft_fail":false,"target_va":5378169824,"va":5378949953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1427434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378949296,"raw_hex":"e81638eaff","size":5,"soft_fail":false,"target_va":5377522032,"va":5378949461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1425514","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378949296,"raw_hex":"e8963feaff","size":5,"soft_fail":false,"target_va":5377524592,"va":5378950101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2168466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378949296,"raw_hex":"e892162100","size":5,"soft_fail":false,"target_va":5381117840,"va":5378949369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32586958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378949296,"raw_hex":"e8ce3cf101","size":5,"soft_fail":false,"target_va":5411537104,"va":5378950141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rsi","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5378949296,"raw_hex":"ffd6","size":2,"soft_fail":false,"va":5378950051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":47384,"parent_record_offset":47176,"ancestor_record_offset":47176,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":48504,"parent_record_offset":47176,"ancestor_record_offset":47176,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":529,"record_offset":48764,"parent_record_offset":47176,"ancestor_record_offset":47176,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_list::DrawListItem::to_draw_packet
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_list.rs
//     evidence={"module":529,"record_offset":48840,"parent_record_offset":47176,"ancestor_record_offset":47176,"inlinee":25983,"source_line":32,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":529,"record_offset":47412,"parent_record_offset":47176,"ancestor_record_offset":47176,"inlinee":8275,"source_line":337,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::common_draw_params::CommonDrawParams::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs
//     evidence={"module":529,"record_offset":47592,"parent_record_offset":47176,"ancestor_record_offset":47176,"inlinee":25981,"source_line":26,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":529,"record_offset":49432,"parent_record_offset":47176,"ancestor_record_offset":47176,"inlinee":8289,"source_line":637,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_depth_write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":529,"record_offset":49348,"parent_record_offset":47176,"ancestor_record_offset":47176,"inlinee":30288,"source_line":627,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":529,"record_offset":48088,"parent_record_offset":48064,"ancestor_record_offset":47176,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":529,"record_offset":48136,"parent_record_offset":48112,"ancestor_record_offset":47176,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":529,"record_offset":48112,"parent_record_offset":48088,"ancestor_record_offset":47176,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":48064,"parent_record_offset":48040,"ancestor_record_offset":47176,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":48040,"parent_record_offset":48000,"ancestor_record_offset":47176,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":48000,"parent_record_offset":47976,"ancestor_record_offset":47176,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":529,"record_offset":48388,"parent_record_offset":48364,"ancestor_record_offset":47176,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":529,"record_offset":48364,"parent_record_offset":48340,"ancestor_record_offset":47176,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":529,"record_offset":48312,"parent_record_offset":48280,"ancestor_record_offset":47176,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":529,"record_offset":48256,"parent_record_offset":47720,"ancestor_record_offset":47176,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":529,"record_offset":48280,"parent_record_offset":48256,"ancestor_record_offset":47176,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":529,"record_offset":47748,"parent_record_offset":47720,"ancestor_record_offset":47176,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":529,"record_offset":48340,"parent_record_offset":48280,"ancestor_record_offset":47176,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":529,"record_offset":47796,"parent_record_offset":47772,"ancestor_record_offset":47176,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":529,"record_offset":47772,"parent_record_offset":47748,"ancestor_record_offset":47176,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":529,"record_offset":47976,"parent_record_offset":47952,"ancestor_record_offset":47176,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":529,"record_offset":47680,"parent_record_offset":47652,"ancestor_record_offset":47176,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":529,"record_offset":47652,"parent_record_offset":47624,"ancestor_record_offset":47176,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":529,"record_offset":47720,"parent_record_offset":47652,"ancestor_record_offset":47176,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":529,"record_offset":49172,"parent_record_offset":49144,"ancestor_record_offset":47176,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":529,"record_offset":49224,"parent_record_offset":49200,"ancestor_record_offset":47176,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":529,"record_offset":49200,"parent_record_offset":49144,"ancestor_record_offset":47176,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":529,"record_offset":47444,"parent_record_offset":47412,"ancestor_record_offset":47176,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":529,"record_offset":49248,"parent_record_offset":49224,"ancestor_record_offset":47176,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":529,"record_offset":47888,"parent_record_offset":47824,"ancestor_record_offset":47176,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":529,"record_offset":47912,"parent_record_offset":47888,"ancestor_record_offset":47176,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":529,"record_offset":47952,"parent_record_offset":47772,"ancestor_record_offset":47176,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":529,"record_offset":48440,"parent_record_offset":47624,"ancestor_record_offset":47176,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":529,"record_offset":49116,"parent_record_offset":49084,"ancestor_record_offset":47176,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":529,"record_offset":48724,"parent_record_offset":48700,"ancestor_record_offset":47176,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":529,"record_offset":48804,"parent_record_offset":48764,"ancestor_record_offset":47176,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":529,"record_offset":49272,"parent_record_offset":49248,"ancestor_record_offset":47176,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":529,"record_offset":47824,"parent_record_offset":47796,"ancestor_record_offset":47176,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":529,"record_offset":48204,"parent_record_offset":48180,"ancestor_record_offset":47176,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":529,"record_offset":47860,"parent_record_offset":47824,"ancestor_record_offset":47176,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":529,"record_offset":48180,"parent_record_offset":48000,"ancestor_record_offset":47176,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":529,"record_offset":48700,"parent_record_offset":48528,"ancestor_record_offset":47176,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::as_secs_f32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":529,"record_offset":48472,"parent_record_offset":47592,"ancestor_record_offset":47176,"inlinee":11237,"source_line":853,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":529,"record_offset":48872,"parent_record_offset":48840,"ancestor_record_offset":47176,"inlinee":25985,"source_line":106,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":529,"record_offset":48920,"parent_record_offset":48872,"ancestor_record_offset":47176,"inlinee":25987,"source_line":138,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::ColorAttachment::desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":529,"record_offset":47520,"parent_record_offset":47492,"ancestor_record_offset":47176,"inlinee":25998,"source_line":89,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":529,"record_offset":47492,"parent_record_offset":47468,"ancestor_record_offset":47176,"inlinee":8279,"source_line":145,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$5::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":529,"record_offset":47468,"parent_record_offset":47444,"ancestor_record_offset":47176,"inlinee":8277,"source_line":179,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":529,"record_offset":47624,"parent_record_offset":47592,"ancestor_record_offset":47176,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":529,"record_offset":49396,"parent_record_offset":49372,"ancestor_record_offset":47176,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":529,"record_offset":49456,"parent_record_offset":49432,"ancestor_record_offset":47176,"inlinee":8290,"source_line":637,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_write_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":529,"record_offset":49372,"parent_record_offset":49348,"ancestor_record_offset":47176,"inlinee":30289,"source_line":627,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":529,"record_offset":47544,"parent_record_offset":47520,"ancestor_record_offset":47176,"inlinee":8273,"source_line":315,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":529,"record_offset":48952,"parent_record_offset":48920,"ancestor_record_offset":47176,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":529,"record_offset":49016,"parent_record_offset":48984,"ancestor_record_offset":47176,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":529,"record_offset":49144,"parent_record_offset":49084,"ancestor_record_offset":47176,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":529,"record_offset":48984,"parent_record_offset":48952,"ancestor_record_offset":47176,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":529,"record_offset":49084,"parent_record_offset":49048,"ancestor_record_offset":47176,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":529,"record_offset":49048,"parent_record_offset":49016,"ancestor_record_offset":47176,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00529.jsonl.gz"}
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
// 完整结构及原始类型记录: transparency_pass.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: transparency_pass.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/transparency_pass.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: transparency_pass.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":529,"record_offset":33440,"parent_record_offset":31364,"ancestor_record_offset":31364,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00529.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":529,"record_offset":36276,"parent_record_offset":33572,"ancestor_record_offset":33572,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00529.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::uniform","relationship":"pdb_inline_direct","evidence":{"module":529,"record_offset":36364,"parent_record_offset":33572,"ancestor_record_offset":33572,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00529.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":529,"record_offset":36948,"parent_record_offset":33572,"ancestor_record_offset":33572,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00529.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::uniform","relationship":"pdb_inline_direct","evidence":{"module":529,"record_offset":37044,"parent_record_offset":33572,"ancestor_record_offset":33572,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00529.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":529,"record_offset":37216,"parent_record_offset":33572,"ancestor_record_offset":33572,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00529.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":529,"record_offset":37248,"parent_record_offset":37216,"ancestor_record_offset":33572,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00529.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_lighting_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$1::bind_terrain_lighting","relationship":"pdb_inline_direct","evidence":{"module":529,"record_offset":37348,"parent_record_offset":33572,"ancestor_record_offset":33572,"inlinee":30287,"source_line":30,"archive":"_pdb/B/modules/00529.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":529,"record_offset":37480,"parent_record_offset":33572,"ancestor_record_offset":33572,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00529.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass
// [资源接口动作；资源实例 unknown] {"role":"named_shadowmap_binding_api","symbol":"country_core::render::pass_builder::bind_resources::impl$0::bind_shadowmap","relationship":"pdb_inline_direct","evidence":{"module":529,"record_offset":40996,"parent_record_offset":37532,"ancestor_record_offset":37532,"inlinee":27050,"source_line":50,"archive":"_pdb/B/modules/00529.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::uniform","relationship":"pdb_inline_transitive","evidence":{"module":529,"record_offset":41028,"parent_record_offset":40996,"ancestor_record_offset":37532,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00529.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":529,"record_offset":41100,"parent_record_offset":40996,"ancestor_record_offset":37532,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00529.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":529,"record_offset":41148,"parent_record_offset":37532,"ancestor_record_offset":37532,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00529.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::uniform","relationship":"pdb_inline_direct","evidence":{"module":529,"record_offset":41280,"parent_record_offset":37532,"ancestor_record_offset":37532,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00529.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":529,"record_offset":41784,"parent_record_offset":37532,"ancestor_record_offset":37532,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00529.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::uniform","relationship":"pdb_inline_direct","evidence":{"module":529,"record_offset":41880,"parent_record_offset":37532,"ancestor_record_offset":37532,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00529.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":529,"record_offset":42100,"parent_record_offset":37532,"ancestor_record_offset":37532,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00529.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":529,"record_offset":42132,"parent_record_offset":42100,"ancestor_record_offset":37532,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00529.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_lighting_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$1::bind_terrain_lighting","relationship":"pdb_inline_direct","evidence":{"module":529,"record_offset":42232,"parent_record_offset":37532,"ancestor_record_offset":37532,"inlinee":30287,"source_line":30,"archive":"_pdb/B/modules/00529.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":529,"record_offset":42364,"parent_record_offset":37532,"ancestor_record_offset":37532,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00529.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::transparency_pass::pre_taa_transparency_pass
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":529,"record_offset":46868,"parent_record_offset":44780,"ancestor_record_offset":44780,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00529.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [Shader 映射] {"shader_id":"af995489a66eb9588d8b1e15","relative_path":"_water_calc_deferred_rt_args.cs.hlsl_cs_main_0.bin","sha256":"dc5d1ea515e0ebc44b0842fd7f5c15c5a51ef380fa5991eca5c022a794a378e9","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"transparency_pass","prefix_candidates":["transparency_pass","water_","clouds","smoke","window_glass"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"a5e5fe9c13202a4e23a3fce9","relative_path":"_water_deferred_rt.cs.hlsl_cs_main_0.bin","sha256":"b49e27df6d1dccd41d284b6eb0cd3887a2998712da8e463e27d10a453b155645","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"transparency_pass","prefix_candidates":["transparency_pass","water_","clouds","smoke","window_glass"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"74444f291589198e6c27eb84","relative_path":"_water_draw_water_mask.cs.hlsl_cs_main_0.bin","sha256":"b1232643d097424ca6171666d0307a0e4c4d892c1367bac6959e44798145b4f0","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"transparency_pass","prefix_candidates":["transparency_pass","water_","clouds","smoke","window_glass"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"8f95918ce155111a5a34afd7","relative_path":"_water_ice_cracks.raster.hlsl_b903e43ffb3da915.bin","sha256":"eb5a947c86232a0cfdd68a464a6815f127aac6862dc9015d800ea7e553731c75","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"transparency_pass","prefix_candidates":["transparency_pass","water_","clouds","smoke","window_glass"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"ac049c147306f41c90eddb77","relative_path":"_water_ice_cracks.raster.hlsl_f0adff76827e73ff.bin","sha256":"399f9828b872e899113f8e1ec82d307cdee6844594beeeef98e279b2a189c7c5","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"transparency_pass","prefix_candidates":["transparency_pass","water_","clouds","smoke","window_glass"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"1abaa6f03dd83f5bedc9ddc4","relative_path":"_water_ice_deferred_rt.cs.hlsl_cs_main_0.bin","sha256":"e89dbbb6639f2f55d53c4f78a5740bcda1235e97aa82a089c38da5b0dc2fd43e","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"transparency_pass","prefix_candidates":["transparency_pass","water_","clouds","smoke","window_glass"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"aee640dfb411b19da289a496","relative_path":"_water_ice_refl_march.cs.hlsl_cs_main_0.bin","sha256":"e358ca240e8539099ac6e51523ab7bd9fb934b53219cd7e7f25dfe8f2abce3f9","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"transparency_pass","prefix_candidates":["transparency_pass","water_","clouds","smoke","window_glass"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"3e89d19089d4ed2a1e1e0767","relative_path":"_water_ice_refl_resolve.cs.hlsl_cs_main_0.bin","sha256":"074b2bfa7f5d6059ec6c00e24f9de113dc37d7aacffe07236706e55958bd8feb","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"transparency_pass","prefix_candidates":["transparency_pass","water_","clouds","smoke","window_glass"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"44c31eebe1e3551907df32ef","relative_path":"_water_ice_refr_resolve.cs.hlsl_cs_main_0.bin","sha256":"4654758b495c8858fbd76d213e46a4ef430ad8350141e9a324e47c9769064cf9","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"transparency_pass","prefix_candidates":["transparency_pass","water_","clouds","smoke","window_glass"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"1271e32f5d8120b845f5e5c5","relative_path":"_water_plane.raster.hlsl_b903e43ffb3da915.bin","sha256":"ee889ef518781bf3f788ed325d40f9d64b698f5d15b48137ea6668645f7058d0","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"transparency_pass","prefix_candidates":["transparency_pass","water_","clouds","smoke","window_glass"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"edfea69a9ecdc64aa8922895","relative_path":"_water_plane.raster.hlsl_f0adff76827e73ff.bin","sha256":"579286a3363779668845a2d01d39f75c64e4d2b0ac14ebc5e100fa017f4b2b3b","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"transparency_pass","prefix_candidates":["transparency_pass","water_","clouds","smoke","window_glass"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"eaaaf4bd57d922c1b85d2c22","relative_path":"_water_plane_refraction_copy.raster.hlsl_b903e43ffb3da915.bin","sha256":"b139f4517656f52782fe98497c204f0bbeab01506d4cd38e27f955d0dd03a9fd","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"transparency_pass","prefix_candidates":["transparency_pass","water_","clouds","smoke","window_glass"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"532ebc4440b0186aa4acd91b","relative_path":"_water_plane_refraction_copy.raster.hlsl_f0adff76827e73ff.bin","sha256":"2053d5db2797c992c8bfc398f9555276f5147c1a2acfd65610014ba0588131a9","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"transparency_pass","prefix_candidates":["transparency_pass","water_","clouds","smoke","window_glass"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"00af758d9aa85672b280a062","relative_path":"_water_read_water_mask.cs.hlsl_cs_main_0.bin","sha256":"688f6ea1c960e7c0a97d040cb3a4fb0e4fec01b84c40f7092baf1472df44de2b","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"transparency_pass","prefix_candidates":["transparency_pass","water_","clouds","smoke","window_glass"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"b7e6472e0f7afa1f03a7b69c","relative_path":"_water_snow_plane.raster.hlsl_b903e43ffb3da915.bin","sha256":"5aa8c4b57ed4a7b131340422ae1a38de17213d76596410631de293a497a2031b","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"transparency_pass","prefix_candidates":["transparency_pass","water_","clouds","smoke","window_glass"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"ebd8bf8f4fec205d1324c639","relative_path":"_water_snow_plane.raster.hlsl_f0adff76827e73ff.bin","sha256":"aa1b9f6f7f3d9424417e7a75a277c964919e07414d40ebc6a208f8776b71ab1a","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"transparency_pass","prefix_candidates":["transparency_pass","water_","clouds","smoke","window_glass"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"bd90be106277ad43748098c7","relative_path":"_water_spawn_boats.cs.hlsl_cs_main_0.bin","sha256":"de1f0c2fd4fb34ceb03463ba264fbe4f2b6c2e4da3626271030dd87f6d8be944","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"transparency_pass","prefix_candidates":["transparency_pass","water_","clouds","smoke","window_glass"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"af428b8ac017994608d0b013","relative_path":"_water_spawn_lilypads.cs.hlsl_cs_main_0.bin","sha256":"1f29a1266813e9705fc9893d76eaf28aaa64d387048ce67fa86b0ef35b47a583","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"transparency_pass","prefix_candidates":["transparency_pass","water_","clouds","smoke","window_glass"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"bba09f9d4d001f513ae9127c","relative_path":"_water_spawn_reeds.cs.hlsl_cs_main_0.bin","sha256":"e5cf1b97c1efbc1b098c65367f91f4ead23688c0871812004954f3925b24fb9f","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"transparency_pass","prefix_candidates":["transparency_pass","water_","clouds","smoke","window_glass"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"5557e65ce37a8b8d2c53164c","relative_path":"_water_spawn_waterwheels.cs.hlsl_cs_main_0.bin","sha256":"59ad163707e92ab4242254d71f3a0672865cb8dd78c3ef0fcfe2257b36c10147","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"transparency_pass","prefix_candidates":["transparency_pass","water_","clouds","smoke","window_glass"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"8f1768e19f16a3d93d34ca79","relative_path":"_water_x_paths.raster.hlsl_b903e43ffb3da915.bin","sha256":"3f3ad9b0259e941310d693ef89028f2d4fe4fca3e62240af9e2663f4c28e6675","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"transparency_pass","prefix_candidates":["transparency_pass","water_","clouds","smoke","window_glass"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"7b5d6cea082f50ec108b5a12","relative_path":"_water_x_paths.raster.hlsl_f0adff76827e73ff.bin","sha256":"e547be6b8c3519014e802e41b2aa098917165fe23c275e8839fb8a1d9b12de28","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"transparency_pass","prefix_candidates":["transparency_pass","water_","clouds","smoke","window_glass"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _water_read_water_mask.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":1,"entry_name":"cs_main","interface_ids":[2]}],"execution_modes":[{"entry_id":1,"mode":"LocalSize","operands":[64,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":17,"name":"water_mask","storage_class":"UniformConstant","type":{"id":34,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,16]},"decorations":[],"pointee":{"id":16,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[23,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":19,"name":"__alias_mask_out","storage_class":"Uniform","type":{"id":35,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,18]},"decorations":[],"pointee":{"id":18,"name":"type.RWStructuredBuffer.uint","record":{"opcode":"OpTypeStruct","operands":[21]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":21,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":20,"name":"__alias_mask_out_clutter","storage_class":"Uniform","type":{"id":35,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,18]},"decorations":[],"pointee":{"id":18,"name":"type.RWStructuredBuffer.uint","record":{"opcode":"OpTypeStruct","operands":[21]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":21,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2,"name":"unknown","storage_class":"Input","type":{"id":37,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,36]},"decorations":[],"pointee":{"id":36,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[24,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _water_plane.raster.hlsl_b903e43ffb3da915.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Vertex","function_id":1,"entry_name":"vs_main","interface_ids":[2,3,4,5]}],"execution_modes":[]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":52,"name":"frame_constants","storage_class":"Uniform","type":{"id":73,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,50]},"decorations":[],"pointee":{"id":50,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[70,70,70,70,70,70,70,70,60,59,59,60,66,60,59,64,59,64,60,64,60,59,60,60,60,70,55]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":70,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":70,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":70,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":70,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":70,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":70,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":70,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":70,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":60,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":59,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":59,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":60,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":66,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":60,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":59,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":64,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":59,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":64,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":60,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":64,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":60,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":59,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":60,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":60,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":60,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":70,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":55,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":54,"name":"$Globals","storage_class":"Uniform","type":{"id":74,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,53]},"decorations":[],"pointee":{"id":53,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[60,72,66,66,60,66,66,66,60,60,66,66,66,66,66,66,66,64]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"current_view_index","type_id":60,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"model_to_world","type_id":72,"decorations":[{"decoration":"Offset","operands":[16]},{"decoration":"MatrixStride","operands":[16]},{"decoration":"RowMajor","operands":[]}]},{"index":2,"name":"terrain_heightmap_dims","type_id":66,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":3,"name":"visual_terrain_heightmap_dims","type_id":66,"decorations":[{"decoration":"Offset","operands":[88]}]},{"index":4,"name":"history_valid","type_id":60,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":5,"name":"terrain_lighting_texture_dims","type_id":66,"decorations":[{"decoration":"Offset","operands":[100]}]},{"index":6,"name":"blue_noise_texture_dims","type_id":66,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":7,"name":"brdf_fg_lut_dims","type_id":66,"decorations":[{"decoration":"Offset","operands":[120]}]},{"index":8,"name":"rt_bvh_node_count","type_id":60,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"rt_triangle_count","type_id":60,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"water_noise_texture_dims","type_id":66,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"framebuffer_dims","type_id":66,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":12,"name":"water_mask_dims","type_id":66,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":13,"name":"half_depth_tex_dims","type_id":66,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":14,"name":"refraction_tex_dims","type_id":66,"decorations":[{"decoration":"Offset","operands":[168]}]},{"index":15,"name":"taa_history_tex_dims","type_id":66,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":16,"name":"reprojection_tex_dims","type_id":66,"decorations":[{"decoration":"Offset","operands":[184]}]},{"index":17,"name":"water_color","type_id":64,"decorations":[{"decoration":"Offset","operands":[192]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2,"name":"in.var.Vertex_Position","storage_class":"Input","type":{"id":75,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,64]},"decorations":[],"pointee":{"id":64,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[59,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Output","type":{"id":76,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,70]},"decorations":[],"pointee":{"id":70,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[59,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":0,"declared_direction":"stage_output","decorations":[{"decoration":"BuiltIn","operands":[0]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"out.var.C0","storage_class":"Output","type":{"id":77,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,64]},"decorations":[],"pointee":{"id":64,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[59,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"out.var.C1","storage_class":"Output","type":{"id":77,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,64]},"decorations":[],"pointee":{"id":64,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[59,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Fragment","function_id":2,"entry_name":"ps_main","interface_ids":[3,4,5]}],"execution_modes":[{"entry_id":2,"mode":"OriginUpperLeft","operands":[],"operand_kind":"literal"},{"entry_id":2,"mode":"EarlyFragmentTests","operands":[],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":52,"name":"sampler_lnc","storage_class":"UniformConstant","type":{"id":202,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,51]},"decorations":[],"pointee":{"id":51,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":53,"name":"sampler_llr","storage_class":"UniformConstant","type":{"id":202,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,51]},"decorations":[],"pointee":{"id":51,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[1]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":54,"name":"sampler_nnc","storage_class":"UniformConstant","type":{"id":202,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,51]},"decorations":[],"pointee":{"id":51,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":55,"name":"sampler_llc","storage_class":"UniformConstant","type":{"id":202,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,51]},"decorations":[],"pointee":{"id":51,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":56,"name":"sampler_shadow_nearest","storage_class":"UniformConstant","type":{"id":202,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,51]},"decorations":[],"pointee":{"id":51,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":57,"name":"sampler_shadow_linear","storage_class":"UniformConstant","type":{"id":202,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,51]},"decorations":[],"pointee":{"id":51,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[12]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":60,"name":"frame_constants","storage_class":"Uniform","type":{"id":204,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,58]},"decorations":[],"pointee":{"id":58,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[150,150,150,150,150,150,150,150,97,95,95,97,114,97,95,105,95,105,97,105,97,95,97,97,97,150,84]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":150,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":150,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":150,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":150,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":150,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":150,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":150,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":150,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":97,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":95,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":95,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":97,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":114,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":97,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":95,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":105,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":95,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":105,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":97,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":105,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":97,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":95,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":97,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":97,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":97,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":150,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":84,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":63,"name":"$Globals","storage_class":"Uniform","type":{"id":206,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,62]},"decorations":[],"pointee":{"id":62,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[97,203,114,114,97,114,114,114,97,97,114,114,114,114,114,114,114,105]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"current_view_index","type_id":97,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"model_to_world","type_id":203,"decorations":[{"decoration":"Offset","operands":[16]},{"decoration":"MatrixStride","operands":[16]},{"decoration":"RowMajor","operands":[]}]},{"index":2,"name":"terrain_heightmap_dims","type_id":114,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":3,"name":"visual_terrain_heightmap_dims","type_id":114,"decorations":[{"decoration":"Offset","operands":[88]}]},{"index":4,"name":"history_valid","type_id":97,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":5,"name":"terrain_lighting_texture_dims","type_id":114,"decorations":[{"decoration":"Offset","operands":[100]}]},{"index":6,"name":"blue_noise_texture_dims","type_id":114,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":7,"name":"brdf_fg_lut_dims","type_id":114,"decorations":[{"decoration":"Offset","operands":[120]}]},{"index":8,"name":"rt_bvh_node_count","type_id":97,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"rt_triangle_count","type_id":97,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"water_noise_texture_dims","type_id":114,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"framebuffer_dims","type_id":114,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":12,"name":"water_mask_dims","type_id":114,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":13,"name":"half_depth_tex_dims","type_id":114,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":14,"name":"refraction_tex_dims","type_id":114,"decorations":[{"decoration":"Offset","operands":[168]}]},{"index":15,"name":"taa_history_tex_dims","type_id":114,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":16,"name":"reprojection_tex_dims","type_id":114,"decorations":[{"decoration":"Offset","operands":[184]}]},{"index":17,"name":"water_color","type_id":105,"decorations":[{"decoration":"Offset","operands":[192]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":65,"name":"shadow_map","storage_class":"UniformConstant","type":{"id":208,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,64]},"decorations":[],"pointee":{"id":64,"name":"type.2d.image.array","record":{"opcode":"OpTypeImage","operands":[95,1,2,1,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":19,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[19]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":68,"name":"ripple_source_buffer","storage_class":"Uniform","type":{"id":210,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,66]},"decorations":[],"pointee":{"id":66,"name":"type.StructuredBuffer.RippleSourceSsbo","record":{"opcode":"OpTypeStruct","operands":[87]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":87,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":25,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[25]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":69,"name":"water_noise_texture","storage_class":"UniformConstant","type":{"id":207,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,50]},"decorations":[],"pointee":{"id":50,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[95,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":26,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[26]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":70,"name":"water_mask","storage_class":"UniformConstant","type":{"id":207,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,50]},"decorations":[],"pointee":{"id":50,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[95,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":27,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[27]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":71,"name":"half_depth_tex","storage_class":"UniformConstant","type":{"id":207,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,50]},"decorations":[],"pointee":{"id":50,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[95,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":28,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[28]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":72,"name":"refraction_tex","storage_class":"UniformConstant","type":{"id":207,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,50]},"decorations":[],"pointee":{"id":50,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[95,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":29,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[29]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":73,"name":"taa_history_tex","storage_class":"UniformConstant","type":{"id":207,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,50]},"decorations":[],"pointee":{"id":50,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[95,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":30,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[30]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":74,"name":"reprojection_tex","storage_class":"UniformConstant","type":{"id":207,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,50]},"decorations":[],"pointee":{"id":50,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[95,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":31,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[31]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":77,"name":"autofocus_buf","storage_class":"Uniform","type":{"id":211,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,75]},"decorations":[],"pointee":{"id":75,"name":"type.StructuredBuffer.DofFocus","record":{"opcode":"OpTypeStruct","operands":[88]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":88,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":32,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[32]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":79,"name":"__alias_water_rt_count_buffer","storage_class":"Uniform","type":{"id":212,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,78]},"decorations":[],"pointee":{"id":78,"name":"type.RWStructuredBuffer.uint","record":{"opcode":"OpTypeStruct","operands":[85]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":85,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":33,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[33]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":81,"name":"__alias_water_rt_ray_buffer","storage_class":"Uniform","type":{"id":213,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,80]},"decorations":[],"pointee":{"id":80,"name":"type.RWStructuredBuffer.v4uint","record":{"opcode":"OpTypeStruct","operands":[89]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":89,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":34,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[34]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":214,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,150]},"decorations":[],"pointee":{"id":150,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[95,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":15,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[15]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"in.var.C1","storage_class":"Input","type":{"id":215,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,105]},"decorations":[],"pointee":{"id":105,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[95,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"out.var.SV_TARGET","storage_class":"Output","type":{"id":216,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,150]},"decorations":[],"pointee":{"id":150,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[95,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _water_ice_deferred_rt.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[64,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":42,"name":"sampler_lnc","storage_class":"UniformConstant","type":{"id":167,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,41]},"decorations":[],"pointee":{"id":41,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":43,"name":"sampler_nnc","storage_class":"UniformConstant","type":{"id":167,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,41]},"decorations":[],"pointee":{"id":41,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":44,"name":"sampler_default","storage_class":"UniformConstant","type":{"id":167,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,41]},"decorations":[],"pointee":{"id":41,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":46,"name":"$Globals","storage_class":"Uniform","type":{"id":168,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,45]},"decorations":[],"pointee":{"id":45,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[100,100,100,100,100,84,84,84,100,100,100,100,100]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"terrain_lighting_texture_dims","type_id":100,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"blue_noise_texture_dims","type_id":100,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"terrain_heightmap_dims","type_id":100,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"visual_terrain_heightmap_dims","type_id":100,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"brdf_fg_lut_dims","type_id":100,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"rt_bvh_node_count","type_id":84,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"rt_triangle_count","type_id":84,"decorations":[{"decoration":"Offset","operands":[44]}]},{"index":7,"name":"history_valid","type_id":84,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":8,"name":"half_depth_tex_dims","type_id":100,"decorations":[{"decoration":"Offset","operands":[52]}]},{"index":9,"name":"taa_history_tex_dims","type_id":100,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":10,"name":"reprojection_tex_dims","type_id":100,"decorations":[{"decoration":"Offset","operands":[72]}]},{"index":11,"name":"refl_hit_tex_dims","type_id":100,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":12,"name":"refl_hit_radiance_tex_dims","type_id":100,"decorations":[{"decoration":"Offset","operands":[88]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":48,"name":"blue_noise_texture","storage_class":"UniformConstant","type":{"id":169,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,47]},"decorations":[],"pointee":{"id":47,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[83,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":51,"name":"frame_constants","storage_class":"Uniform","type":{"id":171,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,49]},"decorations":[],"pointee":{"id":49,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[113,113,113,113,113,113,113,113,84,83,83,84,100,84,83,92,83,92,84,92,84,83,84,84,84,113,71]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":113,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":113,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":113,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":113,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":113,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":113,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":113,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":113,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":84,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":83,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":83,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":84,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":100,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":84,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":83,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":92,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":83,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":92,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":84,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":92,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":84,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":83,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":84,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":84,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":84,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":113,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":71,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":52,"name":"terrain_heightmap","storage_class":"UniformConstant","type":{"id":169,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,47]},"decorations":[],"pointee":{"id":47,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[83,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":18,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[18]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":53,"name":"visual_terrain_heightmap","storage_class":"UniformConstant","type":{"id":169,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,47]},"decorations":[],"pointee":{"id":47,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[83,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":19,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[19]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":56,"name":"rt_bl_bvh","storage_class":"Uniform","type":{"id":173,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,54]},"decorations":[],"pointee":{"id":54,"name":"type.StructuredBuffer.PackedBlBvhNode","record":{"opcode":"OpTypeStruct","operands":[73]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":73,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":23,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[23]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":59,"name":"rt_triangles","storage_class":"Uniform","type":{"id":174,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,57]},"decorations":[],"pointee":{"id":57,"name":"type.StructuredBuffer.PackedTriangle","record":{"opcode":"OpTypeStruct","operands":[76]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":76,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":24,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[24]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":61,"name":"ice_rt_count_buffer","storage_class":"Uniform","type":{"id":175,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,60]},"decorations":[],"pointee":{"id":60,"name":"type.StructuredBuffer.v4uint","record":{"opcode":"OpTypeStruct","operands":[77]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":77,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":25,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[25]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":62,"name":"ice_rt_ray_buffer","storage_class":"Uniform","type":{"id":175,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,60]},"decorations":[],"pointee":{"id":60,"name":"type.StructuredBuffer.v4uint","record":{"opcode":"OpTypeStruct","operands":[77]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":77,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":26,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[26]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":63,"name":"half_depth_tex","storage_class":"UniformConstant","type":{"id":169,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,47]},"decorations":[],"pointee":{"id":47,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[83,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":27,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[27]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":64,"name":"taa_history_tex","storage_class":"UniformConstant","type":{"id":169,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,47]},"decorations":[],"pointee":{"id":47,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[83,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":28,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[28]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":65,"name":"reprojection_tex","storage_class":"UniformConstant","type":{"id":169,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,47]},"decorations":[],"pointee":{"id":47,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[83,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":29,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[29]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":67,"name":"refl_hit_tex","storage_class":"UniformConstant","type":{"id":176,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,66]},"decorations":[],"pointee":{"id":66,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[83,1,2,0,0,2,7]},"decorations":[]}},"descriptor_set":0,"binding":30,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[30]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":69,"name":"refl_hit_radiance_tex","storage_class":"UniformConstant","type":{"id":177,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,68]},"decorations":[],"pointee":{"id":68,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[84,1,2,0,0,2,33]},"decorations":[]}},"descriptor_set":0,"binding":31,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[31]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":179,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,178]},"decorations":[],"pointee":{"id":178,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[84,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _water_ice_refl_resolve.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":24,"name":"sampler_lnc","storage_class":"UniformConstant","type":{"id":113,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,23]},"decorations":[],"pointee":{"id":23,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":27,"name":"frame_constants","storage_class":"Uniform","type":{"id":116,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,25]},"decorations":[],"pointee":{"id":25,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[79,79,79,79,79,79,79,79,52,50,50,52,57,52,50,66,50,66,52,66,52,50,52,52,52,79,45]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":79,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":79,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":79,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":79,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":79,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":79,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":79,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":79,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":52,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":50,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":50,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":52,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":57,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":52,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":50,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":66,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":50,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":66,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":52,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":66,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":52,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":50,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":52,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":52,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":52,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":79,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":45,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":30,"name":"$Globals","storage_class":"Uniform","type":{"id":117,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,29]},"decorations":[],"pointee":{"id":29,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[57,52,57,57,57,57,57,57,57,57,57,57,57]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"brdf_fg_lut_dims","type_id":57,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"history_valid","type_id":52,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"blue_noise_texture_dims","type_id":57,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"depth_tex_dims","type_id":57,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"ice_roughness_tex_dims","type_id":57,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"refl_hit_tex_dims","type_id":57,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"refl_hit_radiance_tex_dims","type_id":57,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":7,"name":"color_tex_dims","type_id":57,"decorations":[{"decoration":"Offset","operands":[56]}]},{"index":8,"name":"refl_refr_radiance_history_tex_dims","type_id":57,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":9,"name":"refl_refr_radiance_out_tex_dims","type_id":57,"decorations":[{"decoration":"Offset","operands":[72]}]},{"index":10,"name":"ray_len_history_tex_dims","type_id":57,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":11,"name":"ray_len_out_tex_dims","type_id":57,"decorations":[{"decoration":"Offset","operands":[88]}]},{"index":12,"name":"reprojection_tex_dims","type_id":57,"decorations":[{"decoration":"Offset","operands":[96]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":31,"name":"blue_noise_texture","storage_class":"UniformConstant","type":{"id":118,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,28]},"decorations":[],"pointee":{"id":28,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[50,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":32,"name":"depth_tex","storage_class":"UniformConstant","type":{"id":118,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,28]},"decorations":[],"pointee":{"id":28,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[50,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":33,"name":"ice_roughness_tex","storage_class":"UniformConstant","type":{"id":118,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,28]},"decorations":[],"pointee":{"id":28,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[50,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":17,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[17]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":34,"name":"refl_hit_tex","storage_class":"UniformConstant","type":{"id":118,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,28]},"decorations":[],"pointee":{"id":28,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[50,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":18,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[18]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":35,"name":"refl_hit_radiance_tex","storage_class":"UniformConstant","type":{"id":118,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,28]},"decorations":[],"pointee":{"id":28,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[50,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":19,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[19]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":37,"name":"color_tex","storage_class":"UniformConstant","type":{"id":119,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,36]},"decorations":[],"pointee":{"id":36,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[50,1,2,0,0,2,8]},"decorations":[]}},"descriptor_set":0,"binding":20,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[20]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":38,"name":"refl_refr_radiance_history_tex","storage_class":"UniformConstant","type":{"id":118,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,28]},"decorations":[],"pointee":{"id":28,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[50,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":21,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[21]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":39,"name":"refl_refr_radiance_out_tex","storage_class":"UniformConstant","type":{"id":119,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,36]},"decorations":[],"pointee":{"id":36,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[50,1,2,0,0,2,8]},"decorations":[]}},"descriptor_set":0,"binding":22,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[22]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":40,"name":"ray_len_history_tex","storage_class":"UniformConstant","type":{"id":118,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,28]},"decorations":[],"pointee":{"id":28,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[50,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":23,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[23]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":42,"name":"ray_len_out_tex","storage_class":"UniformConstant","type":{"id":120,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,41]},"decorations":[],"pointee":{"id":41,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[50,1,2,0,0,2,7]},"decorations":[]}},"descriptor_set":0,"binding":24,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[24]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":122,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,121]},"decorations":[],"pointee":{"id":121,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[52,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _water_ice_refr_resolve.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":21,"name":"frame_constants","storage_class":"Uniform","type":{"id":76,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,19]},"decorations":[],"pointee":{"id":19,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[50,50,50,50,50,50,50,50,37,35,35,37,40,37,35,48,35,48,37,48,37,35,37,37,37,50,31]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":50,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":50,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":50,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":50,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":50,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":50,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":50,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":50,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":37,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":35,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":35,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":37,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":40,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":37,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":35,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":48,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":35,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":48,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":37,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":48,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":37,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":35,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":37,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":37,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":37,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":50,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":31,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":24,"name":"$Globals","storage_class":"Uniform","type":{"id":77,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,23]},"decorations":[],"pointee":{"id":23,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[40,40,40,40]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"depth_tex_dims","type_id":40,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"refr_hit_tex_dims","type_id":40,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"refr_hit_radiance_tex_dims","type_id":40,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"refl_refr_radiance_out_tex_dims","type_id":40,"decorations":[{"decoration":"Offset","operands":[24]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":25,"name":"depth_tex","storage_class":"UniformConstant","type":{"id":78,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,22]},"decorations":[],"pointee":{"id":22,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[35,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":27,"name":"refr_hit_tex","storage_class":"UniformConstant","type":{"id":79,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,26]},"decorations":[],"pointee":{"id":26,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[37,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":28,"name":"refr_hit_radiance_tex","storage_class":"UniformConstant","type":{"id":78,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,22]},"decorations":[],"pointee":{"id":22,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[35,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":30,"name":"refl_refr_radiance_out_tex","storage_class":"UniformConstant","type":{"id":80,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,29]},"decorations":[],"pointee":{"id":29,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[35,1,2,0,0,2,8]},"decorations":[]}},"descriptor_set":0,"binding":17,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[17]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":82,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,81]},"decorations":[],"pointee":{"id":81,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[37,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _water_plane_refraction_copy.raster.hlsl_f0adff76827e73ff.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Vertex","function_id":1,"entry_name":"vs_main","interface_ids":[2,3,4,5,6]}],"execution_modes":[]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":31,"name":"frame_constants","storage_class":"Uniform","type":{"id":55,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,29]},"decorations":[],"pointee":{"id":29,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[51,51,51,51,51,51,51,51,38,37,37,38,52,38,37,42,37,42,38,42,38,37,38,38,38,51,34]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":51,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":51,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":51,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":51,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":51,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":51,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":51,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":51,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":38,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":37,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":37,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":38,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":52,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":38,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":37,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":42,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":37,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":42,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":38,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":42,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":38,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":37,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":38,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":38,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":38,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":51,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":34,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":33,"name":"$Globals","storage_class":"Uniform","type":{"id":56,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,32]},"decorations":[],"pointee":{"id":32,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[38,35,54,52,52,52,52]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"current_view_index","type_id":38,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"bool_render_shadows","type_id":35,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"model_to_world","type_id":54,"decorations":[{"decoration":"Offset","operands":[16]},{"decoration":"MatrixStride","operands":[16]},{"decoration":"RowMajor","operands":[]}]},{"index":3,"name":"terrain_heightmap_dims","type_id":52,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":4,"name":"visual_terrain_heightmap_dims","type_id":52,"decorations":[{"decoration":"Offset","operands":[88]}]},{"index":5,"name":"water_mask_dims","type_id":52,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":6,"name":"refraction_source_tex_dims","type_id":52,"decorations":[{"decoration":"Offset","operands":[104]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2,"name":"in.var.Vertex_Position","storage_class":"Input","type":{"id":57,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,42]},"decorations":[],"pointee":{"id":42,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[37,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Output","type":{"id":58,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,51]},"decorations":[],"pointee":{"id":51,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[37,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":0,"declared_direction":"stage_output","decorations":[{"decoration":"BuiltIn","operands":[0]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"out.var.C0","storage_class":"Output","type":{"id":59,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,42]},"decorations":[],"pointee":{"id":42,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[37,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"out.var.C1","storage_class":"Output","type":{"id":59,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,42]},"decorations":[],"pointee":{"id":42,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[37,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"out.var.C2","storage_class":"Output","type":{"id":59,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,42]},"decorations":[],"pointee":{"id":42,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[37,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Fragment","function_id":1,"entry_name":"ps_main","interface_ids":[]}],"execution_modes":[{"entry_id":1,"mode":"OriginUpperLeft","operands":[],"operand_kind":"literal"}]}
// [Shader 声明接口；Pass 实际绑定见映射状态] _water_spawn_waterwheels.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[64,1,1],"operand_kind":"literal"}]}
// [Shader 声明接口；Pass 实际绑定见映射状态] _water_draw_water_mask.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":13,"name":"$Globals","storage_class":"Uniform","type":{"id":32,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,12]},"decorations":[],"pointee":{"id":12,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[31,16,16,23]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"out_water_mask_dims","type_id":31,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"is_add","type_id":16,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"is_subtract","type_id":16,"decorations":[{"decoration":"Offset","operands":[12]}]},{"index":3,"name":"cursor_position_ws","type_id":23,"decorations":[{"decoration":"Offset","operands":[16]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":15,"name":"out_water_mask","storage_class":"UniformConstant","type":{"id":33,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,14]},"decorations":[],"pointee":{"id":14,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[20,1,2,0,0,2,1]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":35,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,34]},"decorations":[],"pointee":{"id":34,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[16,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _water_x_paths.raster.hlsl_f0adff76827e73ff.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Vertex","function_id":2,"entry_name":"vs_main","interface_ids":[3,4,5,6,7,8,9,10]}],"execution_modes":[]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":42,"name":"sampler_llb","storage_class":"UniformConstant","type":{"id":89,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,41]},"decorations":[],"pointee":{"id":41,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":4,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[4]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":43,"name":"sampler_default","storage_class":"UniformConstant","type":{"id":89,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,41]},"decorations":[],"pointee":{"id":41,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":46,"name":"frame_constants","storage_class":"Uniform","type":{"id":93,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,44]},"decorations":[],"pointee":{"id":44,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[90,90,90,90,90,90,90,90,57,56,56,57,63,57,56,61,56,61,57,61,57,56,57,57,57,90,54]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":90,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":90,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":90,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":90,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":90,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":90,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":90,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":90,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":57,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":56,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":56,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":57,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":63,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":57,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":56,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":61,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":56,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":61,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":57,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":61,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":57,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":56,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":57,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":57,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":57,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":90,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":54,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":49,"name":"$Globals","storage_class":"Uniform","type":{"id":94,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,48]},"decorations":[],"pointee":{"id":48,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[57,63,63,63,63,55]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"current_view_index","type_id":57,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"water_mask_dims","type_id":63,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"path_mask_dims","type_id":63,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"buildings_mask_dims","type_id":63,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"rocky_terrain_texture_dims","type_id":63,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"bool_render_shadows","type_id":55,"decorations":[{"decoration":"Offset","operands":[40]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":50,"name":"water_mask","storage_class":"UniformConstant","type":{"id":95,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,47]},"decorations":[],"pointee":{"id":47,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[56,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":51,"name":"path_mask","storage_class":"UniformConstant","type":{"id":95,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,47]},"decorations":[],"pointee":{"id":47,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[56,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":52,"name":"buildings_mask","storage_class":"UniformConstant","type":{"id":95,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,47]},"decorations":[],"pointee":{"id":47,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[56,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.Vertex_Position","storage_class":"Input","type":{"id":96,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,61]},"decorations":[],"pointee":{"id":61,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[56,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"in.var.Vertex_Normal","storage_class":"Input","type":{"id":96,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,61]},"decorations":[],"pointee":{"id":61,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[56,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"in.var.prim_center","storage_class":"Input","type":{"id":96,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,61]},"decorations":[],"pointee":{"id":61,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[56,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Output","type":{"id":97,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,90]},"decorations":[],"pointee":{"id":90,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[56,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":0,"declared_direction":"stage_output","decorations":[{"decoration":"BuiltIn","operands":[0]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"out.var.C0","storage_class":"Output","type":{"id":98,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,61]},"decorations":[],"pointee":{"id":61,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[56,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"out.var.C1","storage_class":"Output","type":{"id":98,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,61]},"decorations":[],"pointee":{"id":61,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[56,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"out.var.C2","storage_class":"Output","type":{"id":98,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,61]},"decorations":[],"pointee":{"id":61,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[56,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":10,"name":"out.var.C3","storage_class":"Output","type":{"id":98,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,61]},"decorations":[],"pointee":{"id":61,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[56,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":3,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[3]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Fragment","function_id":2,"entry_name":"ps_main","interface_ids":[]}],"execution_modes":[{"entry_id":2,"mode":"OriginUpperLeft","operands":[],"operand_kind":"literal"}]}
// [Shader 声明接口；Pass 实际绑定见映射状态] _water_x_paths.raster.hlsl_b903e43ffb3da915.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Vertex","function_id":2,"entry_name":"vs_main","interface_ids":[3,4,5,6,7,8,9,10]}],"execution_modes":[]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":42,"name":"sampler_llb","storage_class":"UniformConstant","type":{"id":89,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,41]},"decorations":[],"pointee":{"id":41,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":4,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[4]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":43,"name":"sampler_default","storage_class":"UniformConstant","type":{"id":89,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,41]},"decorations":[],"pointee":{"id":41,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":46,"name":"frame_constants","storage_class":"Uniform","type":{"id":93,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,44]},"decorations":[],"pointee":{"id":44,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[90,90,90,90,90,90,90,90,57,56,56,57,63,57,56,61,56,61,57,61,57,56,57,57,57,90,54]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":90,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":90,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":90,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":90,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":90,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":90,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":90,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":90,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":57,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":56,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":56,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":57,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":63,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":57,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":56,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":61,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":56,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":61,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":57,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":61,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":57,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":56,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":57,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":57,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":57,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":90,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":54,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":49,"name":"$Globals","storage_class":"Uniform","type":{"id":94,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,48]},"decorations":[],"pointee":{"id":48,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[57,63,63,63,63,55]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"current_view_index","type_id":57,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"water_mask_dims","type_id":63,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"path_mask_dims","type_id":63,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"buildings_mask_dims","type_id":63,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"rocky_terrain_texture_dims","type_id":63,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"bool_render_shadows","type_id":55,"decorations":[{"decoration":"Offset","operands":[40]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":50,"name":"water_mask","storage_class":"UniformConstant","type":{"id":95,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,47]},"decorations":[],"pointee":{"id":47,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[56,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":51,"name":"path_mask","storage_class":"UniformConstant","type":{"id":95,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,47]},"decorations":[],"pointee":{"id":47,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[56,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":52,"name":"buildings_mask","storage_class":"UniformConstant","type":{"id":95,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,47]},"decorations":[],"pointee":{"id":47,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[56,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.Vertex_Position","storage_class":"Input","type":{"id":96,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,61]},"decorations":[],"pointee":{"id":61,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[56,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"in.var.Vertex_Normal","storage_class":"Input","type":{"id":96,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,61]},"decorations":[],"pointee":{"id":61,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[56,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"in.var.prim_center","storage_class":"Input","type":{"id":96,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,61]},"decorations":[],"pointee":{"id":61,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[56,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Output","type":{"id":97,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,90]},"decorations":[],"pointee":{"id":90,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[56,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":0,"declared_direction":"stage_output","decorations":[{"decoration":"BuiltIn","operands":[0]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"out.var.C0","storage_class":"Output","type":{"id":98,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,61]},"decorations":[],"pointee":{"id":61,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[56,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"out.var.C1","storage_class":"Output","type":{"id":98,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,61]},"decorations":[],"pointee":{"id":61,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[56,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"out.var.C2","storage_class":"Output","type":{"id":98,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,61]},"decorations":[],"pointee":{"id":61,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[56,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":10,"name":"out.var.C3","storage_class":"Output","type":{"id":98,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,61]},"decorations":[],"pointee":{"id":61,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[56,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":3,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[3]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Fragment","function_id":2,"entry_name":"ps_main","interface_ids":[3,4,5,6]}],"execution_modes":[{"entry_id":2,"mode":"OriginUpperLeft","operands":[],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":38,"name":"sampler_llb","storage_class":"UniformConstant","type":{"id":102,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,37]},"decorations":[],"pointee":{"id":37,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":4,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[4]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":39,"name":"sampler_mipmap","storage_class":"UniformConstant","type":{"id":102,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,37]},"decorations":[],"pointee":{"id":37,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":42,"name":"frame_constants","storage_class":"Uniform","type":{"id":105,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,40]},"decorations":[],"pointee":{"id":40,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[103,103,103,103,103,103,103,103,51,50,50,51,60,51,50,57,50,57,51,57,51,50,51,51,51,103,47]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":103,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":103,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":103,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":103,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":103,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":103,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":103,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":103,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":51,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":50,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":50,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":51,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":60,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":51,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":50,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":57,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":50,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":57,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":51,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":57,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":51,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":50,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":51,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":51,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":51,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":103,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":47,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":44,"name":"path_mask","storage_class":"UniformConstant","type":{"id":106,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,43]},"decorations":[],"pointee":{"id":43,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[50,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":45,"name":"rocky_terrain_texture","storage_class":"UniformConstant","type":{"id":106,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,43]},"decorations":[],"pointee":{"id":43,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[50,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":17,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[17]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.C1","storage_class":"Input","type":{"id":107,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,57]},"decorations":[],"pointee":{"id":57,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[50,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"in.var.C2","storage_class":"Input","type":{"id":107,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,57]},"decorations":[],"pointee":{"id":57,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[50,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"in.var.C3","storage_class":"Input","type":{"id":107,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,57]},"decorations":[],"pointee":{"id":57,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[50,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":3,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[3]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"out.var.SV_TARGET0","storage_class":"Output","type":{"id":108,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,60]},"decorations":[],"pointee":{"id":60,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[51,2]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _water_ice_cracks.raster.hlsl_b903e43ffb3da915.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Vertex","function_id":1,"entry_name":"vs_main","interface_ids":[2,3,4,5,6,7,8]}],"execution_modes":[]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":38,"name":"frame_constants","storage_class":"Uniform","type":{"id":56,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,36]},"decorations":[],"pointee":{"id":36,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[53,53,53,53,53,53,53,53,45,44,44,45,48,45,44,47,44,47,45,47,45,44,45,45,45,53,41]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":53,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":53,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":53,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":53,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":53,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":53,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":53,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":53,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":45,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":44,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":44,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":45,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":48,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":45,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":44,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":47,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":44,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":47,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":45,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":47,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":45,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":44,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":45,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":45,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":45,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":53,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":41,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":40,"name":"$Globals","storage_class":"Uniform","type":{"id":57,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,39]},"decorations":[],"pointee":{"id":39,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[45,48,55]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"current_view_index","type_id":45,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"water_mask_dims","type_id":48,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"model_to_world","type_id":55,"decorations":[{"decoration":"Offset","operands":[16]},{"decoration":"MatrixStride","operands":[16]},{"decoration":"RowMajor","operands":[]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2,"name":"in.var.Vertex_Position","storage_class":"Input","type":{"id":58,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,47]},"decorations":[],"pointee":{"id":47,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[44,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.Vertex_Color","storage_class":"Input","type":{"id":58,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,47]},"decorations":[],"pointee":{"id":47,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[44,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"in.var.Vertex_Normal","storage_class":"Input","type":{"id":58,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,47]},"decorations":[],"pointee":{"id":47,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[44,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Output","type":{"id":59,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,53]},"decorations":[],"pointee":{"id":53,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[44,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":0,"declared_direction":"stage_output","decorations":[{"decoration":"BuiltIn","operands":[0]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"out.var.C0","storage_class":"Output","type":{"id":60,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,47]},"decorations":[],"pointee":{"id":47,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[44,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"out.var.C1","storage_class":"Output","type":{"id":60,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,47]},"decorations":[],"pointee":{"id":47,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[44,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"out.var.C2","storage_class":"Output","type":{"id":60,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,47]},"decorations":[],"pointee":{"id":47,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[44,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Fragment","function_id":2,"entry_name":"ps_main","interface_ids":[3,4]}],"execution_modes":[{"entry_id":2,"mode":"OriginUpperLeft","operands":[],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":34,"name":"frame_constants","storage_class":"Uniform","type":{"id":57,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,32]},"decorations":[],"pointee":{"id":32,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[47,47,47,47,47,47,47,47,38,37,37,38,43,38,37,41,37,41,38,41,38,37,38,38,38,47,35]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":47,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":47,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":47,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":47,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":47,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":47,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":47,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":47,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":38,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":37,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":37,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":38,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":43,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":38,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":37,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":41,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":37,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":41,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":38,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":41,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":38,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":37,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":38,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":38,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":38,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":47,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":35,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.C0","storage_class":"Input","type":{"id":58,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,41]},"decorations":[],"pointee":{"id":41,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[37,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"out.var.SV_TARGET0","storage_class":"Output","type":{"id":59,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,43]},"decorations":[],"pointee":{"id":43,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[38,2]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _water_deferred_rt.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[64,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":41,"name":"sampler_lnc","storage_class":"UniformConstant","type":{"id":174,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,40]},"decorations":[],"pointee":{"id":40,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":42,"name":"sampler_nnc","storage_class":"UniformConstant","type":{"id":174,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,40]},"decorations":[],"pointee":{"id":40,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":43,"name":"sampler_default","storage_class":"UniformConstant","type":{"id":174,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,40]},"decorations":[],"pointee":{"id":40,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":45,"name":"$Globals","storage_class":"Uniform","type":{"id":175,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,44]},"decorations":[],"pointee":{"id":44,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[97,97,97,97,97,81,81,81,97,97,97,97]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"terrain_lighting_texture_dims","type_id":97,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"blue_noise_texture_dims","type_id":97,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"terrain_heightmap_dims","type_id":97,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"visual_terrain_heightmap_dims","type_id":97,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"brdf_fg_lut_dims","type_id":97,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"rt_bvh_node_count","type_id":81,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"rt_triangle_count","type_id":81,"decorations":[{"decoration":"Offset","operands":[44]}]},{"index":7,"name":"history_valid","type_id":81,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":8,"name":"half_depth_tex_dims","type_id":97,"decorations":[{"decoration":"Offset","operands":[52]}]},{"index":9,"name":"taa_history_tex_dims","type_id":97,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":10,"name":"reprojection_tex_dims","type_id":97,"decorations":[{"decoration":"Offset","operands":[72]}]},{"index":11,"name":"color_tex_dims","type_id":97,"decorations":[{"decoration":"Offset","operands":[80]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":47,"name":"blue_noise_texture","storage_class":"UniformConstant","type":{"id":176,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,46]},"decorations":[],"pointee":{"id":46,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[80,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":50,"name":"frame_constants","storage_class":"Uniform","type":{"id":178,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,48]},"decorations":[],"pointee":{"id":48,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[114,114,114,114,114,114,114,114,81,80,80,81,97,81,80,89,80,89,81,89,81,80,81,81,81,114,68]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":114,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":114,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":114,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":114,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":114,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":114,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":114,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":114,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":81,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":80,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":80,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":81,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":97,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":81,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":80,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":89,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":80,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":89,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":81,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":89,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":81,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":80,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":81,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":81,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":81,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":114,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":68,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":51,"name":"terrain_heightmap","storage_class":"UniformConstant","type":{"id":176,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,46]},"decorations":[],"pointee":{"id":46,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[80,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":18,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[18]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":52,"name":"visual_terrain_heightmap","storage_class":"UniformConstant","type":{"id":176,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,46]},"decorations":[],"pointee":{"id":46,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[80,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":19,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[19]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":55,"name":"rt_bl_bvh","storage_class":"Uniform","type":{"id":180,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,53]},"decorations":[],"pointee":{"id":53,"name":"type.StructuredBuffer.PackedBlBvhNode","record":{"opcode":"OpTypeStruct","operands":[70]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":70,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":23,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[23]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":58,"name":"rt_triangles","storage_class":"Uniform","type":{"id":181,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,56]},"decorations":[],"pointee":{"id":56,"name":"type.StructuredBuffer.PackedTriangle","record":{"opcode":"OpTypeStruct","operands":[73]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":73,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":24,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[24]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":60,"name":"water_rt_count_buffer","storage_class":"Uniform","type":{"id":182,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,59]},"decorations":[],"pointee":{"id":59,"name":"type.StructuredBuffer.v4uint","record":{"opcode":"OpTypeStruct","operands":[74]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":74,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":25,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[25]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":61,"name":"water_rt_ray_buffer","storage_class":"Uniform","type":{"id":182,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,59]},"decorations":[],"pointee":{"id":59,"name":"type.StructuredBuffer.v4uint","record":{"opcode":"OpTypeStruct","operands":[74]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":74,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":26,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[26]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":62,"name":"half_depth_tex","storage_class":"UniformConstant","type":{"id":176,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,46]},"decorations":[],"pointee":{"id":46,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[80,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":27,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[27]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":63,"name":"taa_history_tex","storage_class":"UniformConstant","type":{"id":176,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,46]},"decorations":[],"pointee":{"id":46,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[80,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":28,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[28]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":64,"name":"reprojection_tex","storage_class":"UniformConstant","type":{"id":176,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,46]},"decorations":[],"pointee":{"id":46,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[80,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":29,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[29]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":66,"name":"color_tex","storage_class":"UniformConstant","type":{"id":183,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,65]},"decorations":[],"pointee":{"id":65,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[80,1,2,0,0,2,8]},"decorations":[]}},"descriptor_set":0,"binding":30,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[30]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":185,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,184]},"decorations":[],"pointee":{"id":184,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[81,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _water_ice_cracks.raster.hlsl_f0adff76827e73ff.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Vertex","function_id":1,"entry_name":"vs_main","interface_ids":[2,3,4,5,6,7,8]}],"execution_modes":[]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":38,"name":"frame_constants","storage_class":"Uniform","type":{"id":56,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,36]},"decorations":[],"pointee":{"id":36,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[53,53,53,53,53,53,53,53,45,44,44,45,48,45,44,47,44,47,45,47,45,44,45,45,45,53,41]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":53,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":53,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":53,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":53,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":53,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":53,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":53,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":53,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":45,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":44,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":44,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":45,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":48,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":45,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":44,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":47,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":44,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":47,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":45,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":47,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":45,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":44,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":45,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":45,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":45,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":53,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":41,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":40,"name":"$Globals","storage_class":"Uniform","type":{"id":57,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,39]},"decorations":[],"pointee":{"id":39,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[45,48,55]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"current_view_index","type_id":45,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"water_mask_dims","type_id":48,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"model_to_world","type_id":55,"decorations":[{"decoration":"Offset","operands":[16]},{"decoration":"MatrixStride","operands":[16]},{"decoration":"RowMajor","operands":[]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2,"name":"in.var.Vertex_Position","storage_class":"Input","type":{"id":58,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,47]},"decorations":[],"pointee":{"id":47,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[44,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.Vertex_Color","storage_class":"Input","type":{"id":58,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,47]},"decorations":[],"pointee":{"id":47,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[44,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"in.var.Vertex_Normal","storage_class":"Input","type":{"id":58,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,47]},"decorations":[],"pointee":{"id":47,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[44,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Output","type":{"id":59,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,53]},"decorations":[],"pointee":{"id":53,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[44,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":0,"declared_direction":"stage_output","decorations":[{"decoration":"BuiltIn","operands":[0]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"out.var.C0","storage_class":"Output","type":{"id":60,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,47]},"decorations":[],"pointee":{"id":47,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[44,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"out.var.C1","storage_class":"Output","type":{"id":60,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,47]},"decorations":[],"pointee":{"id":47,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[44,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"out.var.C2","storage_class":"Output","type":{"id":60,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,47]},"decorations":[],"pointee":{"id":47,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[44,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Fragment","function_id":1,"entry_name":"ps_main","interface_ids":[]}],"execution_modes":[{"entry_id":1,"mode":"OriginUpperLeft","operands":[],"operand_kind":"literal"}]}
// [Shader 声明接口；Pass 实际绑定见映射状态] _water_ice_refl_march.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[64,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":44,"name":"sampler_lnc","storage_class":"UniformConstant","type":{"id":136,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,43]},"decorations":[],"pointee":{"id":43,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":45,"name":"sampler_nnc","storage_class":"UniformConstant","type":{"id":136,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,43]},"decorations":[],"pointee":{"id":43,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":47,"name":"$Globals","storage_class":"Uniform","type":{"id":137,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,46]},"decorations":[],"pointee":{"id":46,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[77,77,77,77,77,75,75,75,77,77,77,77,77,77]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"terrain_lighting_texture_dims","type_id":77,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"blue_noise_texture_dims","type_id":77,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"terrain_heightmap_dims","type_id":77,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"visual_terrain_heightmap_dims","type_id":77,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"brdf_fg_lut_dims","type_id":77,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"rt_bvh_node_count","type_id":75,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"rt_triangle_count","type_id":75,"decorations":[{"decoration":"Offset","operands":[44]}]},{"index":7,"name":"history_valid","type_id":75,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":8,"name":"half_depth_tex_dims","type_id":77,"decorations":[{"decoration":"Offset","operands":[52]}]},{"index":9,"name":"taa_history_tex_dims","type_id":77,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":10,"name":"reprojection_tex_dims","type_id":77,"decorations":[{"decoration":"Offset","operands":[72]}]},{"index":11,"name":"ice_roughness_tex_dims","type_id":77,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":12,"name":"refl_hit_tex_dims","type_id":77,"decorations":[{"decoration":"Offset","operands":[88]}]},{"index":13,"name":"refl_hit_radiance_tex_dims","type_id":77,"decorations":[{"decoration":"Offset","operands":[96]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":50,"name":"frame_constants","storage_class":"Uniform","type":{"id":141,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,48]},"decorations":[],"pointee":{"id":48,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[109,109,109,109,109,109,109,109,75,74,74,75,77,75,74,88,74,88,75,88,75,74,75,75,75,109,68]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":109,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":109,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":109,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":109,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":109,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":109,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":109,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":109,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":75,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":74,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":74,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":75,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":77,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":75,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":74,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":88,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":74,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":88,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":75,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":88,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":75,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":74,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":75,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":75,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":75,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":109,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":68,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":53,"name":"ice_refl_count_buffer","storage_class":"Uniform","type":{"id":144,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,52]},"decorations":[],"pointee":{"id":52,"name":"type.StructuredBuffer.v4uint","record":{"opcode":"OpTypeStruct","operands":[70]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":70,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":25,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[25]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":54,"name":"ice_refl_px_buffer","storage_class":"Uniform","type":{"id":142,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,51]},"decorations":[],"pointee":{"id":51,"name":"type.StructuredBuffer.uint","record":{"opcode":"OpTypeStruct","operands":[69]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":69,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":26,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[26]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":55,"name":"half_depth_tex","storage_class":"UniformConstant","type":{"id":138,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,42]},"decorations":[],"pointee":{"id":42,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[74,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":27,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[27]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":56,"name":"taa_history_tex","storage_class":"UniformConstant","type":{"id":138,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,42]},"decorations":[],"pointee":{"id":42,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[74,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":28,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[28]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":57,"name":"reprojection_tex","storage_class":"UniformConstant","type":{"id":138,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,42]},"decorations":[],"pointee":{"id":42,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[74,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":29,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[29]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":58,"name":"ice_roughness_tex","storage_class":"UniformConstant","type":{"id":138,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,42]},"decorations":[],"pointee":{"id":42,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[74,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":30,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[30]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":60,"name":"refl_hit_tex","storage_class":"UniformConstant","type":{"id":145,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,59]},"decorations":[],"pointee":{"id":59,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[74,1,2,0,0,2,7]},"decorations":[]}},"descriptor_set":0,"binding":31,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[31]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":62,"name":"refl_hit_radiance_tex","storage_class":"UniformConstant","type":{"id":146,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,61]},"decorations":[],"pointee":{"id":61,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[75,1,2,0,0,2,33]},"decorations":[]}},"descriptor_set":0,"binding":32,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[32]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":64,"name":"__alias_ice_rt_count_buffer","storage_class":"Uniform","type":{"id":147,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,63]},"decorations":[],"pointee":{"id":63,"name":"type.RWStructuredBuffer.uint","record":{"opcode":"OpTypeStruct","operands":[69]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":69,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":33,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[33]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":66,"name":"__alias_ice_rt_ray_buffer","storage_class":"Uniform","type":{"id":148,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,65]},"decorations":[],"pointee":{"id":65,"name":"type.RWStructuredBuffer.v4uint","record":{"opcode":"OpTypeStruct","operands":[70]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":70,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":34,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[34]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":150,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,149]},"decorations":[],"pointee":{"id":149,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[75,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _water_spawn_lilypads.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":24,"name":"sampler_llb","storage_class":"UniformConstant","type":{"id":113,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,23]},"decorations":[],"pointee":{"id":23,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":4,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[4]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":25,"name":"sampler_default","storage_class":"UniformConstant","type":{"id":113,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,23]},"decorations":[],"pointee":{"id":23,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":28,"name":"__alias_draw_commands","storage_class":"Uniform","type":{"id":114,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,26]},"decorations":[],"pointee":{"id":26,"name":"type.RWStructuredBuffer.DrawIndexedIndirectCommand","record":{"opcode":"OpTypeStruct","operands":[46]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":46,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":30,"name":"$Globals","storage_class":"Uniform","type":{"id":117,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,29]},"decorations":[],"pointee":{"id":29,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[50,115,115,115,115,116]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"draw_indirect_buffer_offset","type_id":50,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"water_mask_dims","type_id":115,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"path_mask_dims","type_id":115,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"garden_mask_dims","type_id":115,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"animation_dims","type_id":115,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"grid_dims","type_id":116,"decorations":[{"decoration":"Offset","operands":[40]}]}]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":33,"name":"frame_constants","storage_class":"Uniform","type":{"id":121,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,31]},"decorations":[],"pointee":{"id":31,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[118,118,118,118,118,118,118,118,60,56,56,60,115,60,56,62,56,62,60,62,60,56,60,60,60,118,47]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":118,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":118,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":118,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":118,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":118,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":118,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":118,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":118,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":60,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":56,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":56,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":60,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":115,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":60,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":56,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":62,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":56,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":62,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":60,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":62,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":60,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":56,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":60,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":60,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":60,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":118,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":47,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":35,"name":"water_mask","storage_class":"UniformConstant","type":{"id":122,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[56,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":36,"name":"path_mask","storage_class":"UniformConstant","type":{"id":122,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[56,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":37,"name":"garden_mask","storage_class":"UniformConstant","type":{"id":122,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[56,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":17,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[17]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":39,"name":"animation","storage_class":"UniformConstant","type":{"id":123,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,38]},"decorations":[],"pointee":{"id":38,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[56,1,2,0,0,2,9]},"decorations":[]}},"descriptor_set":0,"binding":18,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[18]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":43,"name":"__alias_lilypad_buffer","storage_class":"Uniform","type":{"id":125,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,40]},"decorations":[],"pointee":{"id":40,"name":"type.RWStructuredBuffer.LilypadXform","record":{"opcode":"OpTypeStruct","operands":[49]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":49,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":19,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[19]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":44,"name":"__alias_lilypad_flower_buffer","storage_class":"Uniform","type":{"id":125,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,40]},"decorations":[],"pointee":{"id":40,"name":"type.RWStructuredBuffer.LilypadXform","record":{"opcode":"OpTypeStruct","operands":[49]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":49,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":20,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[20]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":127,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,126]},"decorations":[],"pointee":{"id":126,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[60,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _water_calc_deferred_rt_args.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":1,"entry_name":"cs_main","interface_ids":[]}],"execution_modes":[{"entry_id":1,"mode":"LocalSize","operands":[1,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"water_rt_count_buffer","storage_class":"Uniform","type":{"id":18,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,8]},"decorations":[],"pointee":{"id":8,"name":"type.RWStructuredBuffer.v4uint","record":{"opcode":"OpTypeStruct","operands":[10]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":10,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _water_snow_plane.raster.hlsl_b903e43ffb3da915.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Vertex","function_id":1,"entry_name":"vs_main","interface_ids":[2,3,4]}],"execution_modes":[]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":33,"name":"frame_constants","storage_class":"Uniform","type":{"id":52,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,31]},"decorations":[],"pointee":{"id":31,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[49,49,49,49,49,49,49,49,40,39,39,40,45,40,39,44,39,44,40,44,40,39,40,40,40,49,36]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":49,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":49,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":49,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":49,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":49,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":49,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":49,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":49,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":40,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":39,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":39,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":40,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":45,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":40,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":39,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":44,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":39,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":44,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":40,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":44,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":40,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":39,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":40,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":40,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":40,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":49,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":36,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":35,"name":"$Globals","storage_class":"Uniform","type":{"id":53,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,34]},"decorations":[],"pointee":{"id":34,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[40,51,45,37]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"current_view_index","type_id":40,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"model_to_world","type_id":51,"decorations":[{"decoration":"Offset","operands":[16]},{"decoration":"MatrixStride","operands":[16]},{"decoration":"RowMajor","operands":[]}]},{"index":2,"name":"water_mask_dims","type_id":45,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":3,"name":"bool_render_shadows","type_id":37,"decorations":[{"decoration":"Offset","operands":[88]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2,"name":"in.var.Vertex_Position","storage_class":"Input","type":{"id":54,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,44]},"decorations":[],"pointee":{"id":44,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[39,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Output","type":{"id":55,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,49]},"decorations":[],"pointee":{"id":49,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[39,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":0,"declared_direction":"stage_output","decorations":[{"decoration":"BuiltIn","operands":[0]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"out.var.C1","storage_class":"Output","type":{"id":56,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,44]},"decorations":[],"pointee":{"id":44,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[39,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Fragment","function_id":2,"entry_name":"ps_main","interface_ids":[3,4]}],"execution_modes":[{"entry_id":2,"mode":"OriginUpperLeft","operands":[],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":32,"name":"sampler_llc","storage_class":"UniformConstant","type":{"id":71,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,31]},"decorations":[],"pointee":{"id":31,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":34,"name":"water_mask","storage_class":"UniformConstant","type":{"id":73,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,33]},"decorations":[],"pointee":{"id":33,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[38,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.C1","storage_class":"Input","type":{"id":74,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,42]},"decorations":[],"pointee":{"id":42,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[38,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"out.var.SV_TARGET0","storage_class":"Output","type":{"id":75,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,44]},"decorations":[],"pointee":{"id":44,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[39,2]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _water_spawn_reeds.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":21,"name":"sampler_llb","storage_class":"UniformConstant","type":{"id":90,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,20]},"decorations":[],"pointee":{"id":20,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":4,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[4]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":22,"name":"sampler_default","storage_class":"UniformConstant","type":{"id":90,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,20]},"decorations":[],"pointee":{"id":20,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":25,"name":"__alias_draw_commands","storage_class":"Uniform","type":{"id":91,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,23]},"decorations":[],"pointee":{"id":23,"name":"type.RWStructuredBuffer.DrawIndexedIndirectCommand","record":{"opcode":"OpTypeStruct","operands":[37]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":37,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":27,"name":"$Globals","storage_class":"Uniform","type":{"id":94,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,26]},"decorations":[],"pointee":{"id":26,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[40,92,92,92,93]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"draw_indirect_buffer_offset","type_id":40,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"water_mask_dims","type_id":92,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"path_mask_dims","type_id":92,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"buildings_mask_non_blurred_dims","type_id":92,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"grid_dims","type_id":93,"decorations":[{"decoration":"Offset","operands":[32]}]}]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":29,"name":"water_mask","storage_class":"UniformConstant","type":{"id":95,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,28]},"decorations":[],"pointee":{"id":28,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[44,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":30,"name":"path_mask","storage_class":"UniformConstant","type":{"id":95,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,28]},"decorations":[],"pointee":{"id":28,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[44,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":31,"name":"buildings_mask_non_blurred","storage_class":"UniformConstant","type":{"id":95,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,28]},"decorations":[],"pointee":{"id":28,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[44,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":17,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[17]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":35,"name":"__alias_instance_buffer","storage_class":"Uniform","type":{"id":97,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,32]},"decorations":[],"pointee":{"id":32,"name":"type.RWStructuredBuffer.Reed","record":{"opcode":"OpTypeStruct","operands":[39]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":39,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":18,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[18]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":99,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,98]},"decorations":[],"pointee":{"id":98,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[48,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _water_spawn_boats.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3,4]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":22,"name":"sampler_lnc","storage_class":"UniformConstant","type":{"id":82,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,21]},"decorations":[],"pointee":{"id":21,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":25,"name":"__alias_draw_commands","storage_class":"Uniform","type":{"id":83,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,23]},"decorations":[],"pointee":{"id":23,"name":"type.RWStructuredBuffer.DrawIndexedIndirectCommand","record":{"opcode":"OpTypeStruct","operands":[41]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":41,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":27,"name":"$Globals","storage_class":"Uniform","type":{"id":85,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,26]},"decorations":[],"pointee":{"id":26,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[45,84,84,84,84]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"draw_indirect_buffer_offset","type_id":45,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"path_mask_dims","type_id":84,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"water_mask_dims","type_id":84,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"buildings_mask_dims","type_id":84,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"grid_dims","type_id":84,"decorations":[{"decoration":"Offset","operands":[32]}]}]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":30,"name":"frame_constants","storage_class":"Uniform","type":{"id":89,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,28]},"decorations":[],"pointee":{"id":28,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[86,86,86,86,86,86,86,86,53,49,49,53,84,53,49,54,49,54,53,54,53,49,53,53,53,86,42]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":86,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":86,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":86,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":86,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":86,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":86,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":86,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":86,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":53,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":49,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":49,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":53,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":84,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":53,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":49,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":54,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":49,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":54,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":53,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":54,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":53,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":49,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":53,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":53,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":53,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":86,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":42,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":32,"name":"path_mask","storage_class":"UniformConstant","type":{"id":90,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,31]},"decorations":[],"pointee":{"id":31,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[49,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":33,"name":"water_mask","storage_class":"UniformConstant","type":{"id":90,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,31]},"decorations":[],"pointee":{"id":31,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[49,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":34,"name":"buildings_mask","storage_class":"UniformConstant","type":{"id":90,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,31]},"decorations":[],"pointee":{"id":31,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[49,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":17,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[17]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":38,"name":"__alias_instance_buffer","storage_class":"Uniform","type":{"id":92,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,35]},"decorations":[],"pointee":{"id":35,"name":"type.RWStructuredBuffer.BoatXform","record":{"opcode":"OpTypeStruct","operands":[44]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":44,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":18,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[18]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":95,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,94]},"decorations":[],"pointee":{"id":94,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[53,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":96,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,53]},"decorations":[],"pointee":{"id":53,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _water_plane_refraction_copy.raster.hlsl_b903e43ffb3da915.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Vertex","function_id":1,"entry_name":"vs_main","interface_ids":[2,3,4,5,6]}],"execution_modes":[]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":31,"name":"frame_constants","storage_class":"Uniform","type":{"id":55,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,29]},"decorations":[],"pointee":{"id":29,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[51,51,51,51,51,51,51,51,38,37,37,38,52,38,37,42,37,42,38,42,38,37,38,38,38,51,34]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":51,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":51,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":51,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":51,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":51,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":51,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":51,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":51,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":38,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":37,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":37,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":38,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":52,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":38,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":37,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":42,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":37,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":42,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":38,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":42,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":38,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":37,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":38,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":38,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":38,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":51,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":34,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":33,"name":"$Globals","storage_class":"Uniform","type":{"id":56,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,32]},"decorations":[],"pointee":{"id":32,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[38,35,54,52,52,52,52]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"current_view_index","type_id":38,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"bool_render_shadows","type_id":35,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"model_to_world","type_id":54,"decorations":[{"decoration":"Offset","operands":[16]},{"decoration":"MatrixStride","operands":[16]},{"decoration":"RowMajor","operands":[]}]},{"index":3,"name":"terrain_heightmap_dims","type_id":52,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":4,"name":"visual_terrain_heightmap_dims","type_id":52,"decorations":[{"decoration":"Offset","operands":[88]}]},{"index":5,"name":"water_mask_dims","type_id":52,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":6,"name":"refraction_source_tex_dims","type_id":52,"decorations":[{"decoration":"Offset","operands":[104]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2,"name":"in.var.Vertex_Position","storage_class":"Input","type":{"id":57,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,42]},"decorations":[],"pointee":{"id":42,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[37,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Output","type":{"id":58,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,51]},"decorations":[],"pointee":{"id":51,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[37,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":0,"declared_direction":"stage_output","decorations":[{"decoration":"BuiltIn","operands":[0]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"out.var.C0","storage_class":"Output","type":{"id":59,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,42]},"decorations":[],"pointee":{"id":42,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[37,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"out.var.C1","storage_class":"Output","type":{"id":59,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,42]},"decorations":[],"pointee":{"id":42,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[37,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"out.var.C2","storage_class":"Output","type":{"id":59,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,42]},"decorations":[],"pointee":{"id":42,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[37,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Fragment","function_id":1,"entry_name":"ps_main","interface_ids":[2,3]}],"execution_modes":[{"entry_id":1,"mode":"OriginUpperLeft","operands":[],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":27,"name":"refraction_source_tex","storage_class":"UniformConstant","type":{"id":35,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,26]},"decorations":[],"pointee":{"id":26,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[28,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":18,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[18]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2,"name":"unknown","storage_class":"Input","type":{"id":36,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,33]},"decorations":[],"pointee":{"id":33,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[28,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":15,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[15]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"out.var.SV_TARGET","storage_class":"Output","type":{"id":37,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,33]},"decorations":[],"pointee":{"id":33,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[28,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _water_snow_plane.raster.hlsl_f0adff76827e73ff.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Vertex","function_id":1,"entry_name":"vs_main","interface_ids":[2,3,4]}],"execution_modes":[]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":33,"name":"frame_constants","storage_class":"Uniform","type":{"id":52,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,31]},"decorations":[],"pointee":{"id":31,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[49,49,49,49,49,49,49,49,40,39,39,40,45,40,39,44,39,44,40,44,40,39,40,40,40,49,36]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":49,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":49,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":49,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":49,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":49,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":49,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":49,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":49,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":40,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":39,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":39,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":40,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":45,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":40,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":39,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":44,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":39,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":44,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":40,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":44,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":40,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":39,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":40,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":40,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":40,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":49,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":36,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":35,"name":"$Globals","storage_class":"Uniform","type":{"id":53,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,34]},"decorations":[],"pointee":{"id":34,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[40,51,45,37]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"current_view_index","type_id":40,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"model_to_world","type_id":51,"decorations":[{"decoration":"Offset","operands":[16]},{"decoration":"MatrixStride","operands":[16]},{"decoration":"RowMajor","operands":[]}]},{"index":2,"name":"water_mask_dims","type_id":45,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":3,"name":"bool_render_shadows","type_id":37,"decorations":[{"decoration":"Offset","operands":[88]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2,"name":"in.var.Vertex_Position","storage_class":"Input","type":{"id":54,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,44]},"decorations":[],"pointee":{"id":44,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[39,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Output","type":{"id":55,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,49]},"decorations":[],"pointee":{"id":49,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[39,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":0,"declared_direction":"stage_output","decorations":[{"decoration":"BuiltIn","operands":[0]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"out.var.C1","storage_class":"Output","type":{"id":56,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,44]},"decorations":[],"pointee":{"id":44,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[39,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Fragment","function_id":2,"entry_name":"ps_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"OriginUpperLeft","operands":[],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":31,"name":"sampler_llc","storage_class":"UniformConstant","type":{"id":70,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,30]},"decorations":[],"pointee":{"id":30,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[3]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":33,"name":"water_mask","storage_class":"UniformConstant","type":{"id":72,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,32]},"decorations":[],"pointee":{"id":32,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[37,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.C1","storage_class":"Input","type":{"id":73,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,41]},"decorations":[],"pointee":{"id":41,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[37,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _water_plane.raster.hlsl_f0adff76827e73ff.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Vertex","function_id":1,"entry_name":"vs_main","interface_ids":[2,3,4,5]}],"execution_modes":[]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":52,"name":"frame_constants","storage_class":"Uniform","type":{"id":73,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,50]},"decorations":[],"pointee":{"id":50,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[70,70,70,70,70,70,70,70,60,59,59,60,66,60,59,64,59,64,60,64,60,59,60,60,60,70,55]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":70,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":70,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":70,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":70,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":70,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":70,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":70,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":70,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":60,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":59,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":59,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":60,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":66,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":60,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":59,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":64,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":59,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":64,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":60,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":64,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":60,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":59,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":60,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":60,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":60,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":70,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":55,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":54,"name":"$Globals","storage_class":"Uniform","type":{"id":74,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,53]},"decorations":[],"pointee":{"id":53,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[60,72,66,66,60,66,66,66,60,60,66,66,66,66,66,66,66,64]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"current_view_index","type_id":60,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"model_to_world","type_id":72,"decorations":[{"decoration":"Offset","operands":[16]},{"decoration":"MatrixStride","operands":[16]},{"decoration":"RowMajor","operands":[]}]},{"index":2,"name":"terrain_heightmap_dims","type_id":66,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":3,"name":"visual_terrain_heightmap_dims","type_id":66,"decorations":[{"decoration":"Offset","operands":[88]}]},{"index":4,"name":"history_valid","type_id":60,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":5,"name":"terrain_lighting_texture_dims","type_id":66,"decorations":[{"decoration":"Offset","operands":[100]}]},{"index":6,"name":"blue_noise_texture_dims","type_id":66,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":7,"name":"brdf_fg_lut_dims","type_id":66,"decorations":[{"decoration":"Offset","operands":[120]}]},{"index":8,"name":"rt_bvh_node_count","type_id":60,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"rt_triangle_count","type_id":60,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"water_noise_texture_dims","type_id":66,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"framebuffer_dims","type_id":66,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":12,"name":"water_mask_dims","type_id":66,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":13,"name":"half_depth_tex_dims","type_id":66,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":14,"name":"refraction_tex_dims","type_id":66,"decorations":[{"decoration":"Offset","operands":[168]}]},{"index":15,"name":"taa_history_tex_dims","type_id":66,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":16,"name":"reprojection_tex_dims","type_id":66,"decorations":[{"decoration":"Offset","operands":[184]}]},{"index":17,"name":"water_color","type_id":64,"decorations":[{"decoration":"Offset","operands":[192]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2,"name":"in.var.Vertex_Position","storage_class":"Input","type":{"id":75,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,64]},"decorations":[],"pointee":{"id":64,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[59,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Output","type":{"id":76,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,70]},"decorations":[],"pointee":{"id":70,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[59,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":0,"declared_direction":"stage_output","decorations":[{"decoration":"BuiltIn","operands":[0]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"out.var.C0","storage_class":"Output","type":{"id":77,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,64]},"decorations":[],"pointee":{"id":64,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[59,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"out.var.C1","storage_class":"Output","type":{"id":77,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,64]},"decorations":[],"pointee":{"id":64,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[59,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Fragment","function_id":2,"entry_name":"ps_main","interface_ids":[3,4]}],"execution_modes":[{"entry_id":2,"mode":"OriginUpperLeft","operands":[],"operand_kind":"literal"},{"entry_id":2,"mode":"EarlyFragmentTests","operands":[],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":51,"name":"sampler_llr","storage_class":"UniformConstant","type":{"id":154,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,50]},"decorations":[],"pointee":{"id":50,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[1]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":52,"name":"sampler_nnc","storage_class":"UniformConstant","type":{"id":154,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,50]},"decorations":[],"pointee":{"id":50,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":55,"name":"frame_constants","storage_class":"Uniform","type":{"id":156,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,53]},"decorations":[],"pointee":{"id":53,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[113,113,113,113,113,113,113,113,78,77,77,78,93,78,77,85,77,85,78,85,78,77,78,78,78,113,69]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":113,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":113,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":113,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":113,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":113,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":113,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":113,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":113,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":78,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":77,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":77,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":78,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":93,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":78,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":77,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":85,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":77,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":85,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":78,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":85,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":78,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":77,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":78,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":78,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":78,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":113,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":69,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":58,"name":"$Globals","storage_class":"Uniform","type":{"id":158,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,57]},"decorations":[],"pointee":{"id":57,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[78,155,93,93,78,93,93,93,78,78,93,93,93,93,93,93,93,85]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"current_view_index","type_id":78,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"model_to_world","type_id":155,"decorations":[{"decoration":"Offset","operands":[16]},{"decoration":"MatrixStride","operands":[16]},{"decoration":"RowMajor","operands":[]}]},{"index":2,"name":"terrain_heightmap_dims","type_id":93,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":3,"name":"visual_terrain_heightmap_dims","type_id":93,"decorations":[{"decoration":"Offset","operands":[88]}]},{"index":4,"name":"history_valid","type_id":78,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":5,"name":"terrain_lighting_texture_dims","type_id":93,"decorations":[{"decoration":"Offset","operands":[100]}]},{"index":6,"name":"blue_noise_texture_dims","type_id":93,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":7,"name":"brdf_fg_lut_dims","type_id":93,"decorations":[{"decoration":"Offset","operands":[120]}]},{"index":8,"name":"rt_bvh_node_count","type_id":78,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"rt_triangle_count","type_id":78,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"water_noise_texture_dims","type_id":93,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"framebuffer_dims","type_id":93,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":12,"name":"water_mask_dims","type_id":93,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":13,"name":"half_depth_tex_dims","type_id":93,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":14,"name":"refraction_tex_dims","type_id":93,"decorations":[{"decoration":"Offset","operands":[168]}]},{"index":15,"name":"taa_history_tex_dims","type_id":93,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":16,"name":"reprojection_tex_dims","type_id":93,"decorations":[{"decoration":"Offset","operands":[184]}]},{"index":17,"name":"water_color","type_id":85,"decorations":[{"decoration":"Offset","operands":[192]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":61,"name":"ripple_source_buffer","storage_class":"Uniform","type":{"id":161,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,59]},"decorations":[],"pointee":{"id":59,"name":"type.StructuredBuffer.RippleSourceSsbo","record":{"opcode":"OpTypeStruct","operands":[72]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":72,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":25,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[25]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":62,"name":"water_noise_texture","storage_class":"UniformConstant","type":{"id":159,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,49]},"decorations":[],"pointee":{"id":49,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[77,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":26,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[26]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":63,"name":"half_depth_tex","storage_class":"UniformConstant","type":{"id":159,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,49]},"decorations":[],"pointee":{"id":49,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[77,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":28,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[28]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":65,"name":"__alias_water_rt_count_buffer","storage_class":"Uniform","type":{"id":162,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,64]},"decorations":[],"pointee":{"id":64,"name":"type.RWStructuredBuffer.uint","record":{"opcode":"OpTypeStruct","operands":[70]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":70,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":33,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[33]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":67,"name":"__alias_water_rt_ray_buffer","storage_class":"Uniform","type":{"id":163,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,66]},"decorations":[],"pointee":{"id":66,"name":"type.RWStructuredBuffer.v4uint","record":{"opcode":"OpTypeStruct","operands":[73]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":73,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":34,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[34]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":164,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,113]},"decorations":[],"pointee":{"id":113,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[77,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":15,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[15]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"in.var.C1","storage_class":"Input","type":{"id":165,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,85]},"decorations":[],"pointee":{"id":85,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[77,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":5161232,"caller_aliases":{"alias_group_rva":5161232,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5163834,"position_among_mapped_render_targets":49,"mapped_render_target_count":59,"previous_callsite":5163623,"next_callsite":5163981,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/deferred_raster.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/transparency_pass.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5161232,"caller_aliases":{"alias_group_rva":5161232,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5163981,"position_among_mapped_render_targets":50,"mapped_render_target_count":59,"previous_callsite":5163834,"next_callsite":5163992,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/transparency_pass.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/apply_screen_shadows.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5161232,"caller_aliases":{"alias_group_rva":5161232,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5165067,"position_among_mapped_render_targets":58,"mapped_render_target_count":59,"previous_callsite":5165039,"next_callsite":5165081,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/taa.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/shader_refactoring.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: transparency_pass.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>", "bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>", "bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass
// 信息 ID: 73008a709aad2d8f4d68bd17
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::transparency_pass::post_taa_transparency_pass
// 信息 ID: 7ce6407e77bf35b6c3ed882c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ed00b -> 0x9c40b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::transparency_pass::pre_taa_transparency_pass
// 信息 ID: 0c3cc71cd3968cec00578313
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ecbcd -> 0x9c3a80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass
// 信息 ID: bd0f53aaecb7c955a49a1286
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ecb3a -> 0x9bf320 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass
// 信息 ID: 2eb07626360dd83ec07489b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
