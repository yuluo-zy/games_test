// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/instanced_roof.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 862b9bc5778243363f69922dc7f0449dd425442f7476852808c88c11cb347c06
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: instanced_roof.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 532
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
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_shader_nani
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":532,"record_offset":54540,"rva":10326128,"va":5379035248}
// 编译实例 RVAs: 0x9d9070
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=19 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9419214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379035248,"raw_hex":"e8324670ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5379035321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=9
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1334998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379035248,"raw_hex":"e82aa1ebff","size":5,"soft_fail":false,"target_va":5377701312,"va":5379036305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140904ae0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-870748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379035248,"raw_hex":"e8a4b6f2ff","size":5,"soft_fail":false,"target_va":5378165472,"va":5379036215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908610
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-855117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379035248,"raw_hex":"e8b3f3f2ff","size":5,"soft_fail":false,"target_va":5378180624,"va":5379035736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908c50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-853292","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379035248,"raw_hex":"e8d4faf2ff","size":5,"soft_fail":false,"target_va":5378182224,"va":5379035511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b570
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-450136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379035248,"raw_hex":"e8a821f9ff","size":5,"soft_fail":false,"target_va":5378585968,"va":5379036099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162ad90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12917505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379035248,"raw_hex":"e8011bc500","size":5,"soft_fail":false,"target_va":5391953296,"va":5379035786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162bc90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12921453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379035248,"raw_hex":"e86d2ac500","size":5,"soft_fail":false,"target_va":5391957136,"va":5379035678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162ec90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12933366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379035248,"raw_hex":"e8f658c500","size":5,"soft_fail":false,"target_va":5391969424,"va":5379036053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141631090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12942996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379035248,"raw_hex":"e8947ec500","size":5,"soft_fail":false,"target_va":5391978640,"va":5379035639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=9
//     target_aliases=1 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32811706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379035248,"raw_hex":"e8baaaf401","size":5,"soft_fail":false,"target_va":5411847168,"va":5379035457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32505522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379035248,"raw_hex":"e8b2feef01","size":5,"soft_fail":false,"target_va":5411541776,"va":5379036249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":532,"record_offset":55596,"parent_record_offset":54540,"ancestor_record_offset":54540,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_roof_color_customization
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs
//     evidence={"module":532,"record_offset":55836,"parent_record_offset":54540,"ancestor_record_offset":54540,"inlinee":27058,"source_line":84,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":532,"record_offset":55912,"parent_record_offset":54540,"ancestor_record_offset":54540,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":532,"record_offset":56044,"parent_record_offset":54540,"ancestor_record_offset":54540,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":532,"record_offset":55712,"parent_record_offset":54540,"ancestor_record_offset":54540,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":532,"record_offset":54760,"parent_record_offset":54540,"ancestor_record_offset":54540,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":532,"record_offset":55624,"parent_record_offset":54540,"ancestor_record_offset":54540,"inlinee":27055,"source_line":624,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":532,"record_offset":55212,"parent_record_offset":55188,"ancestor_record_offset":54540,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":532,"record_offset":55260,"parent_record_offset":55236,"ancestor_record_offset":54540,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":532,"record_offset":55236,"parent_record_offset":55212,"ancestor_record_offset":54540,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":532,"record_offset":55188,"parent_record_offset":55164,"ancestor_record_offset":54540,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":532,"record_offset":55164,"parent_record_offset":55124,"ancestor_record_offset":54540,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":532,"record_offset":55124,"parent_record_offset":55100,"ancestor_record_offset":54540,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":532,"record_offset":55512,"parent_record_offset":55488,"ancestor_record_offset":54540,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":532,"record_offset":55488,"parent_record_offset":55464,"ancestor_record_offset":54540,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":532,"record_offset":55436,"parent_record_offset":55404,"ancestor_record_offset":54540,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":532,"record_offset":55380,"parent_record_offset":54852,"ancestor_record_offset":54540,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":532,"record_offset":55404,"parent_record_offset":55380,"ancestor_record_offset":54540,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":532,"record_offset":54880,"parent_record_offset":54852,"ancestor_record_offset":54540,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":532,"record_offset":55464,"parent_record_offset":55404,"ancestor_record_offset":54540,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":532,"record_offset":54928,"parent_record_offset":54904,"ancestor_record_offset":54540,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":532,"record_offset":54904,"parent_record_offset":54880,"ancestor_record_offset":54540,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":532,"record_offset":55100,"parent_record_offset":55076,"ancestor_record_offset":54540,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":532,"record_offset":54816,"parent_record_offset":54788,"ancestor_record_offset":54540,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":532,"record_offset":54788,"parent_record_offset":54760,"ancestor_record_offset":54540,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":532,"record_offset":54852,"parent_record_offset":54788,"ancestor_record_offset":54540,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":532,"record_offset":55016,"parent_record_offset":54956,"ancestor_record_offset":54540,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":532,"record_offset":56084,"parent_record_offset":56044,"ancestor_record_offset":54540,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":532,"record_offset":55036,"parent_record_offset":55016,"ancestor_record_offset":54540,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":532,"record_offset":55076,"parent_record_offset":54904,"ancestor_record_offset":54540,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":532,"record_offset":55564,"parent_record_offset":54760,"ancestor_record_offset":54540,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":532,"record_offset":54956,"parent_record_offset":54928,"ancestor_record_offset":54540,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":532,"record_offset":55328,"parent_record_offset":55304,"ancestor_record_offset":54540,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":532,"record_offset":54988,"parent_record_offset":54956,"ancestor_record_offset":54540,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":532,"record_offset":55304,"parent_record_offset":55124,"ancestor_record_offset":54540,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":532,"record_offset":55744,"parent_record_offset":55712,"ancestor_record_offset":54540,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":532,"record_offset":55672,"parent_record_offset":55648,"ancestor_record_offset":54540,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":532,"record_offset":55648,"parent_record_offset":55624,"ancestor_record_offset":54540,"inlinee":27056,"source_line":624,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_spire_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":532,"record_offset":56192,"rva":10327712,"va":5379036832}
// 编译实例 RVAs: 0x9d96a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=19 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9420795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379036832,"raw_hex":"e8054070ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5379036902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1336016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379036832,"raw_hex":"e8309debff","size":5,"soft_fail":false,"target_va":5377701312,"va":5379037323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140969720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-458797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379036832,"raw_hex":"e8d3fff8ff","size":5,"soft_fail":false,"target_va":5378578208,"va":5379037000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141630d90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12940702","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379036832,"raw_hex":"e89e75c500","size":5,"soft_fail":false,"target_va":5391977872,"va":5379037165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32810075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379036832,"raw_hex":"e85ba4f401","size":5,"soft_fail":false,"target_va":5411847152,"va":5379037072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32810030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379036832,"raw_hex":"e82ea4f401","size":5,"soft_fail":false,"target_va":5411847168,"va":5379037133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32504503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379036832,"raw_hex":"e8b7faef01","size":5,"soft_fail":false,"target_va":5411541776,"va":5379037268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32445181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379036832,"raw_hex":"e8fd12ef01","size":5,"soft_fail":false,"target_va":5411482433,"va":5379037247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_instanced_w_gl_draw_flag
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-861048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379036832,"raw_hex":"e888dcf2ff","size":5,"soft_fail":false,"target_va":5378176144,"va":5379037187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5379036832,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5379037099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":532,"record_offset":57476,"parent_record_offset":56192,"ancestor_record_offset":56192,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":532,"record_offset":57236,"parent_record_offset":56192,"ancestor_record_offset":56192,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_instanced
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":532,"record_offset":57620,"parent_record_offset":56192,"ancestor_record_offset":56192,"inlinee":27909,"source_line":298,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":532,"record_offset":56400,"parent_record_offset":56192,"ancestor_record_offset":56192,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":532,"record_offset":56852,"parent_record_offset":56828,"ancestor_record_offset":56192,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":532,"record_offset":56900,"parent_record_offset":56876,"ancestor_record_offset":56192,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":532,"record_offset":56876,"parent_record_offset":56852,"ancestor_record_offset":56192,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":532,"record_offset":56828,"parent_record_offset":56804,"ancestor_record_offset":56192,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":532,"record_offset":56804,"parent_record_offset":56764,"ancestor_record_offset":56192,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":532,"record_offset":56764,"parent_record_offset":56740,"ancestor_record_offset":56192,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":532,"record_offset":57152,"parent_record_offset":57128,"ancestor_record_offset":56192,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":532,"record_offset":57128,"parent_record_offset":57104,"ancestor_record_offset":56192,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":532,"record_offset":57076,"parent_record_offset":57044,"ancestor_record_offset":56192,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":532,"record_offset":57020,"parent_record_offset":56492,"ancestor_record_offset":56192,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":532,"record_offset":57044,"parent_record_offset":57020,"ancestor_record_offset":56192,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":532,"record_offset":56520,"parent_record_offset":56492,"ancestor_record_offset":56192,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":532,"record_offset":57104,"parent_record_offset":57044,"ancestor_record_offset":56192,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":532,"record_offset":56568,"parent_record_offset":56544,"ancestor_record_offset":56192,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":532,"record_offset":56544,"parent_record_offset":56520,"ancestor_record_offset":56192,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":532,"record_offset":56740,"parent_record_offset":56716,"ancestor_record_offset":56192,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":532,"record_offset":56456,"parent_record_offset":56428,"ancestor_record_offset":56192,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":532,"record_offset":56428,"parent_record_offset":56400,"ancestor_record_offset":56192,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":532,"record_offset":56492,"parent_record_offset":56428,"ancestor_record_offset":56192,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":532,"record_offset":57304,"parent_record_offset":57264,"ancestor_record_offset":56192,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":532,"record_offset":56656,"parent_record_offset":56596,"ancestor_record_offset":56192,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":532,"record_offset":56676,"parent_record_offset":56656,"ancestor_record_offset":56192,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":532,"record_offset":56716,"parent_record_offset":56544,"ancestor_record_offset":56192,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":532,"record_offset":57204,"parent_record_offset":56400,"ancestor_record_offset":56192,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":532,"record_offset":57388,"parent_record_offset":57360,"ancestor_record_offset":56192,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":532,"record_offset":57416,"parent_record_offset":57388,"ancestor_record_offset":56192,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":532,"record_offset":56596,"parent_record_offset":56568,"ancestor_record_offset":56192,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":532,"record_offset":56968,"parent_record_offset":56944,"ancestor_record_offset":56192,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":532,"record_offset":56628,"parent_record_offset":56596,"ancestor_record_offset":56192,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":532,"record_offset":56944,"parent_record_offset":56764,"ancestor_record_offset":56192,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":532,"record_offset":57332,"parent_record_offset":57304,"ancestor_record_offset":56192,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":532,"record_offset":57552,"parent_record_offset":57528,"ancestor_record_offset":56192,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":532,"record_offset":57576,"parent_record_offset":57552,"ancestor_record_offset":56192,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":532,"record_offset":57264,"parent_record_offset":57236,"ancestor_record_offset":56192,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::InstanceBuffer::instance_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":532,"record_offset":57528,"parent_record_offset":57504,"ancestor_record_offset":56192,"inlinee":30361,"source_line":392,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_spire_shader::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/instanced_roof.rs
//     evidence={"module":532,"record_offset":57504,"parent_record_offset":57476,"ancestor_record_offset":56192,"inlinee":30384,"source_line":66,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_spire_shader::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x76B0
// 首条位置: {"module":532,"inlinee":30384}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::ssbo::InstanceBuffer::instance_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":532,"record_offset":57528,"parent_record_offset":57504,"ancestor_record_offset":57504,"inlinee":30361,"source_line":392,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":532,"record_offset":57552,"parent_record_offset":57528,"ancestor_record_offset":57504,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":532,"record_offset":57576,"parent_record_offset":57552,"ancestor_record_offset":57504,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_support_beam_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 73
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":532,"record_offset":57652,"rva":10328224,"va":5379037344}
// 编译实例 RVAs: 0x9d98a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=19 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9421308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379037344,"raw_hex":"e8043e70ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5379037415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1336679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379037344,"raw_hex":"e8999aebff","size":5,"soft_fail":false,"target_va":5377701312,"va":5379037986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908c50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-855488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379037344,"raw_hex":"e840f2f2ff","size":5,"soft_fail":false,"target_va":5378182224,"va":5379037707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140969720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-459310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379037344,"raw_hex":"e8d2fdf8ff","size":5,"soft_fail":false,"target_va":5378578208,"va":5379037513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162ef90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12932352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379037344,"raw_hex":"e80055c500","size":5,"soft_fail":false,"target_va":5391970192,"va":5379037835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32809562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379037344,"raw_hex":"e85aa2f401","size":5,"soft_fail":false,"target_va":5411847152,"va":5379037585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: instanced_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32809510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379037344,"raw_hex":"e826a2f401","size":5,"soft_fail":false,"target_va":5411847168,"va":5379037653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32503826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379037344,"raw_hex":"e812f8ef01","size":5,"soft_fail":false,"target_va":5411541776,"va":5379037945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32444504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379037344,"raw_hex":"e85810ef01","size":5,"soft_fail":false,"target_va":5411482433,"va":5379037924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_instanced_w_gl_draw_flag
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-861718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379037344,"raw_hex":"e8ead9f2ff","size":5,"soft_fail":false,"target_va":5378176144,"va":5379037857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5379037344,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5379037616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":532,"record_offset":58948,"parent_record_offset":57652,"ancestor_record_offset":57652,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":532,"record_offset":58708,"parent_record_offset":57652,"ancestor_record_offset":57652,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_instanced
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":532,"record_offset":59216,"parent_record_offset":57652,"ancestor_record_offset":57652,"inlinee":27909,"source_line":298,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":532,"record_offset":59092,"parent_record_offset":57652,"ancestor_record_offset":57652,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":532,"record_offset":57872,"parent_record_offset":57652,"ancestor_record_offset":57652,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":532,"record_offset":58324,"parent_record_offset":58300,"ancestor_record_offset":57652,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":532,"record_offset":58372,"parent_record_offset":58348,"ancestor_record_offset":57652,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":532,"record_offset":58348,"parent_record_offset":58324,"ancestor_record_offset":57652,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":532,"record_offset":58300,"parent_record_offset":58276,"ancestor_record_offset":57652,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":532,"record_offset":58276,"parent_record_offset":58236,"ancestor_record_offset":57652,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":532,"record_offset":58236,"parent_record_offset":58212,"ancestor_record_offset":57652,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":532,"record_offset":58624,"parent_record_offset":58600,"ancestor_record_offset":57652,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":532,"record_offset":58600,"parent_record_offset":58576,"ancestor_record_offset":57652,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":532,"record_offset":58548,"parent_record_offset":58516,"ancestor_record_offset":57652,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":532,"record_offset":58492,"parent_record_offset":57964,"ancestor_record_offset":57652,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":532,"record_offset":58516,"parent_record_offset":58492,"ancestor_record_offset":57652,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":532,"record_offset":57992,"parent_record_offset":57964,"ancestor_record_offset":57652,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":532,"record_offset":58576,"parent_record_offset":58516,"ancestor_record_offset":57652,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":532,"record_offset":58040,"parent_record_offset":58016,"ancestor_record_offset":57652,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":532,"record_offset":58016,"parent_record_offset":57992,"ancestor_record_offset":57652,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":532,"record_offset":58212,"parent_record_offset":58188,"ancestor_record_offset":57652,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":532,"record_offset":57928,"parent_record_offset":57900,"ancestor_record_offset":57652,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":532,"record_offset":57900,"parent_record_offset":57872,"ancestor_record_offset":57652,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":532,"record_offset":57964,"parent_record_offset":57900,"ancestor_record_offset":57652,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":532,"record_offset":58776,"parent_record_offset":58736,"ancestor_record_offset":57652,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":532,"record_offset":58128,"parent_record_offset":58068,"ancestor_record_offset":57652,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":532,"record_offset":58148,"parent_record_offset":58128,"ancestor_record_offset":57652,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":532,"record_offset":58188,"parent_record_offset":58016,"ancestor_record_offset":57652,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":532,"record_offset":58676,"parent_record_offset":57872,"ancestor_record_offset":57652,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":532,"record_offset":58860,"parent_record_offset":58832,"ancestor_record_offset":57652,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":532,"record_offset":58888,"parent_record_offset":58860,"ancestor_record_offset":57652,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":532,"record_offset":58068,"parent_record_offset":58040,"ancestor_record_offset":57652,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":532,"record_offset":58440,"parent_record_offset":58416,"ancestor_record_offset":57652,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":532,"record_offset":58100,"parent_record_offset":58068,"ancestor_record_offset":57652,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":532,"record_offset":58416,"parent_record_offset":58236,"ancestor_record_offset":57652,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":532,"record_offset":58804,"parent_record_offset":58776,"ancestor_record_offset":57652,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":532,"record_offset":59024,"parent_record_offset":59000,"ancestor_record_offset":57652,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":532,"record_offset":59048,"parent_record_offset":59024,"ancestor_record_offset":57652,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":532,"record_offset":58736,"parent_record_offset":58708,"ancestor_record_offset":57652,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":532,"record_offset":59124,"parent_record_offset":59092,"ancestor_record_offset":57652,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::InstanceBuffer::instance_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":532,"record_offset":59000,"parent_record_offset":58976,"ancestor_record_offset":57652,"inlinee":30361,"source_line":392,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_support_beam_shader::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/instanced_roof.rs
//     evidence={"module":532,"record_offset":58976,"parent_record_offset":58948,"ancestor_record_offset":57652,"inlinee":30386,"source_line":81,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_support_beam_shader::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x76B2
// 首条位置: {"module":532,"inlinee":30386}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::ssbo::InstanceBuffer::instance_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":532,"record_offset":59000,"parent_record_offset":58976,"ancestor_record_offset":58976,"inlinee":30361,"source_line":392,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":532,"record_offset":59024,"parent_record_offset":59000,"ancestor_record_offset":58976,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00532.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":532,"record_offset":59048,"parent_record_offset":59024,"ancestor_record_offset":58976,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00532.jsonl.gz"}
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
// 完整结构及原始类型记录: instanced_roof.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: instanced_roof.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/instanced_roof.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: instanced_roof.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_shader_nani
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":532,"record_offset":55712,"parent_record_offset":54540,"ancestor_record_offset":54540,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00532.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":532,"record_offset":55744,"parent_record_offset":55712,"ancestor_record_offset":54540,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00532.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":532,"record_offset":55912,"parent_record_offset":54540,"ancestor_record_offset":54540,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00532.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::uniform","relationship":"pdb_inline_direct","evidence":{"module":532,"record_offset":56044,"parent_record_offset":54540,"ancestor_record_offset":54540,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00532.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_support_beam_shader
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":532,"record_offset":59092,"parent_record_offset":57652,"ancestor_record_offset":57652,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/00532.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":532,"record_offset":59124,"parent_record_offset":59092,"ancestor_record_offset":57652,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00532.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [Shader 映射] {"shader_id":"80a5008f64cdaec31725d612","relative_path":"_instanced_roof_beams.raster.hlsl_b903e43ffb3da915.bin","sha256":"4d3366198bef320422e2d63f34615f5ca5de07d57c71a1727a029884d8b874e9","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"instanced_roof","prefix_candidates":["instanced_roof","roof_tile","roof"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"5e5a906dd22b0dec6e58ef6c","relative_path":"_instanced_roof_beams.raster.hlsl_f0adff76827e73ff.bin","sha256":"9f0b53d8b104400de8c299a2ebaab7c91c323b019ddc67f538c57e4264bd62f9","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"instanced_roof","prefix_candidates":["instanced_roof","roof_tile","roof"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"cb41d9cf28f3179adf919fda","relative_path":"_instanced_roof_spire.raster.hlsl_b903e43ffb3da915.bin","sha256":"1dbfa7f30843a384b7eb4c6d9fbe49e5eff381ab669db0190b6224d53285b01c","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"instanced_roof","prefix_candidates":["instanced_roof","roof_tile","roof"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"a77de6ae8439cb29b38ef793","relative_path":"_instanced_roof_spire.raster.hlsl_f0adff76827e73ff.bin","sha256":"3bea6def1bf85de3fd0123e750cf9c60fac01228bd0ba02d211d75d06c6a1799","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"instanced_roof","prefix_candidates":["instanced_roof","roof_tile","roof"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"48554c4231fdcb6b58f4ed50","relative_path":"_roof_snow.raster.hlsl_b903e43ffb3da915.bin","sha256":"bb7d41ae4ae7e2ad93a71b277c63e40335b7eefab993bd9863670f884dd69988","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"instanced_roof","prefix_candidates":["instanced_roof","roof_tile","roof"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"3a8cd9300b69f4bc2b99b051","relative_path":"_roof_snow.raster.hlsl_f0adff76827e73ff.bin","sha256":"3d737a07918e82f795bd50feef677361c166c6a39bfeda844eb73a8e886c22e2","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"instanced_roof","prefix_candidates":["instanced_roof","roof_tile","roof"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"8ddcce52d5683171fcc7d918","relative_path":"_roof_tiles_gravity.cs.hlsl_cs_main_0.bin","sha256":"706c3651a87f2c3fdcaa3950e866711e958346ba19482c14430addc7b741c5c0","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"instanced_roof","prefix_candidates":["instanced_roof","roof_tile","roof"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _roof_snow.raster.hlsl_f0adff76827e73ff.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Vertex","function_id":2,"entry_name":"vs_main","interface_ids":[3,4,5,6,7,8,9,10]}],"execution_modes":[]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":40,"name":"frame_constants","storage_class":"Uniform","type":{"id":87,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,38]},"decorations":[],"pointee":{"id":38,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[84,84,84,84,84,84,84,84,48,47,47,48,55,48,47,52,47,52,48,52,48,47,48,48,48,84,43]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":84,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":84,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":84,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":84,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":84,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":84,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":84,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":84,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":48,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":47,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":47,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":48,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":55,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":48,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":47,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":52,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":47,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":52,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":48,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":52,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":48,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":47,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":48,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":48,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":48,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":84,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":43,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":42,"name":"$Globals","storage_class":"Uniform","type":{"id":88,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,41]},"decorations":[],"pointee":{"id":41,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[48,86,52,44,44]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"current_view_index","type_id":48,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"model_to_world","type_id":86,"decorations":[{"decoration":"Offset","operands":[16]},{"decoration":"MatrixStride","operands":[16]},{"decoration":"RowMajor","operands":[]}]},{"index":2,"name":"roof_pivot","type_id":52,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":3,"name":"is_circle","type_id":44,"decorations":[{"decoration":"Offset","operands":[92]}]},{"index":4,"name":"bool_render_shadows","type_id":44,"decorations":[{"decoration":"Offset","operands":[96]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.Vertex_Position","storage_class":"Input","type":{"id":89,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,52]},"decorations":[],"pointee":{"id":52,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[47,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"in.var.Vertex_UV","storage_class":"Input","type":{"id":90,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,50]},"decorations":[],"pointee":{"id":50,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[47,2]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"in.var.Vertex_Normal","storage_class":"Input","type":{"id":89,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,52]},"decorations":[],"pointee":{"id":52,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[47,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Output","type":{"id":91,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,84]},"decorations":[],"pointee":{"id":84,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[47,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":0,"declared_direction":"stage_output","decorations":[{"decoration":"BuiltIn","operands":[0]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"out.var.C0","storage_class":"Output","type":{"id":92,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,52]},"decorations":[],"pointee":{"id":52,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[47,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"out.var.C1","storage_class":"Output","type":{"id":92,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,52]},"decorations":[],"pointee":{"id":52,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[47,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"out.var.C2","storage_class":"Output","type":{"id":92,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,52]},"decorations":[],"pointee":{"id":52,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[47,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":10,"name":"out.var.C3","storage_class":"Output","type":{"id":92,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,52]},"decorations":[],"pointee":{"id":52,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[47,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":3,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[3]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Fragment","function_id":2,"entry_name":"ps_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"OriginUpperLeft","operands":[],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.C3","storage_class":"Input","type":{"id":35,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,32]},"decorations":[],"pointee":{"id":32,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[31,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":3,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[3]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _roof_snow.raster.hlsl_b903e43ffb3da915.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Vertex","function_id":2,"entry_name":"vs_main","interface_ids":[3,4,5,6,7,8,9,10]}],"execution_modes":[]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":40,"name":"frame_constants","storage_class":"Uniform","type":{"id":87,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,38]},"decorations":[],"pointee":{"id":38,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[84,84,84,84,84,84,84,84,48,47,47,48,55,48,47,52,47,52,48,52,48,47,48,48,48,84,43]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":84,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":84,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":84,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":84,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":84,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":84,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":84,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":84,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":48,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":47,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":47,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":48,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":55,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":48,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":47,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":52,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":47,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":52,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":48,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":52,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":48,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":47,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":48,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":48,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":48,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":84,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":43,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":42,"name":"$Globals","storage_class":"Uniform","type":{"id":88,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,41]},"decorations":[],"pointee":{"id":41,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[48,86,52,44,44]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"current_view_index","type_id":48,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"model_to_world","type_id":86,"decorations":[{"decoration":"Offset","operands":[16]},{"decoration":"MatrixStride","operands":[16]},{"decoration":"RowMajor","operands":[]}]},{"index":2,"name":"roof_pivot","type_id":52,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":3,"name":"is_circle","type_id":44,"decorations":[{"decoration":"Offset","operands":[92]}]},{"index":4,"name":"bool_render_shadows","type_id":44,"decorations":[{"decoration":"Offset","operands":[96]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.Vertex_Position","storage_class":"Input","type":{"id":89,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,52]},"decorations":[],"pointee":{"id":52,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[47,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"in.var.Vertex_UV","storage_class":"Input","type":{"id":90,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,50]},"decorations":[],"pointee":{"id":50,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[47,2]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"in.var.Vertex_Normal","storage_class":"Input","type":{"id":89,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,52]},"decorations":[],"pointee":{"id":52,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[47,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Output","type":{"id":91,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,84]},"decorations":[],"pointee":{"id":84,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[47,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":0,"declared_direction":"stage_output","decorations":[{"decoration":"BuiltIn","operands":[0]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"out.var.C0","storage_class":"Output","type":{"id":92,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,52]},"decorations":[],"pointee":{"id":52,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[47,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"out.var.C1","storage_class":"Output","type":{"id":92,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,52]},"decorations":[],"pointee":{"id":52,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[47,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"out.var.C2","storage_class":"Output","type":{"id":92,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,52]},"decorations":[],"pointee":{"id":52,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[47,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":10,"name":"out.var.C3","storage_class":"Output","type":{"id":92,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,52]},"decorations":[],"pointee":{"id":52,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[47,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":3,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[3]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Fragment","function_id":2,"entry_name":"ps_main","interface_ids":[3,4,5,6]}],"execution_modes":[{"entry_id":2,"mode":"OriginUpperLeft","operands":[],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.C0","storage_class":"Input","type":{"id":47,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,36]},"decorations":[],"pointee":{"id":36,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[34,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"in.var.C1","storage_class":"Input","type":{"id":47,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,36]},"decorations":[],"pointee":{"id":36,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[34,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"in.var.C3","storage_class":"Input","type":{"id":47,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,36]},"decorations":[],"pointee":{"id":36,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[34,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":3,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[3]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"out.var.SV_TARGET0","storage_class":"Output","type":{"id":48,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,39]},"decorations":[],"pointee":{"id":39,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[35,2]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _instanced_roof_beams.raster.hlsl_f0adff76827e73ff.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Vertex","function_id":2,"entry_name":"vs_main","interface_ids":[3,4,5,6,7,8,9]}],"execution_modes":[]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":38,"name":"frame_constants","storage_class":"Uniform","type":{"id":91,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,36]},"decorations":[],"pointee":{"id":36,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[88,88,88,88,88,88,88,88,54,53,53,54,59,54,53,57,53,57,54,57,54,53,54,54,54,88,47]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":88,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":88,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":88,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":88,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":88,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":88,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":88,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":88,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":54,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":53,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":53,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":54,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":59,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":54,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":53,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":57,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":53,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":57,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":54,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":57,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":54,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":53,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":54,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":54,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":54,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":88,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":47,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":40,"name":"$Globals","storage_class":"Uniform","type":{"id":92,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,39]},"decorations":[],"pointee":{"id":39,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[54]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"current_view_index","type_id":54,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":42,"name":"color_library","storage_class":"Uniform","type":{"id":93,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,41]},"decorations":[],"pointee":{"id":41,"name":"type.ConstantBuffer.ColorLibrary","record":{"opcode":"OpTypeStruct","operands":[88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"temp_color_1","type_id":88,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"temp_color_2","type_id":88,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"temp_color_3","type_id":88,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"roof_color_1","type_id":88,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"roof_color_2","type_id":88,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"sky_low_day","type_id":88,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"sky_high_day","type_id":88,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"sun_tint_day","type_id":88,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"sky_low_dusk","type_id":88,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"sky_high_dusk","type_id":88,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":10,"name":"sun_tint_dusk","type_id":88,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":11,"name":"indirect_light_tint","type_id":88,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":12,"name":"fence_color_1","type_id":88,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":13,"name":"fence_color_2","type_id":88,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":14,"name":"flower_color_1","type_id":88,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":15,"name":"flower_color_2","type_id":88,"decorations":[{"decoration":"Offset","operands":[240]}]},{"index":16,"name":"canopy_color_1","type_id":88,"decorations":[{"decoration":"Offset","operands":[256]}]},{"index":17,"name":"canopy_color_2","type_id":88,"decorations":[{"decoration":"Offset","operands":[272]}]},{"index":18,"name":"grass_1","type_id":88,"decorations":[{"decoration":"Offset","operands":[288]}]},{"index":19,"name":"grass_2","type_id":88,"decorations":[{"decoration":"Offset","operands":[304]}]},{"index":20,"name":"path","type_id":88,"decorations":[{"decoration":"Offset","operands":[320]}]},{"index":21,"name":"sky_base","type_id":88,"decorations":[{"decoration":"Offset","operands":[336]}]},{"index":22,"name":"sky_horizon","type_id":88,"decorations":[{"decoration":"Offset","operands":[352]}]},{"index":23,"name":"sky_clouds_1","type_id":88,"decorations":[{"decoration":"Offset","operands":[368]}]},{"index":24,"name":"sky_clouds_2","type_id":88,"decorations":[{"decoration":"Offset","operands":[384]}]},{"index":25,"name":"terrain_1","type_id":88,"decorations":[{"decoration":"Offset","operands":[400]}]},{"index":26,"name":"terrain_2","type_id":88,"decorations":[{"decoration":"Offset","operands":[416]}]},{"index":27,"name":"terrain_flattened","type_id":88,"decorations":[{"decoration":"Offset","operands":[432]}]},{"index":28,"name":"wall_1","type_id":88,"decorations":[{"decoration":"Offset","operands":[448]}]},{"index":29,"name":"wall_2","type_id":88,"decorations":[{"decoration":"Offset","operands":[464]}]},{"index":30,"name":"wall_moss","type_id":88,"decorations":[{"decoration":"Offset","operands":[480]}]},{"index":31,"name":"wall_bricks","type_id":88,"decorations":[{"decoration":"Offset","operands":[496]}]},{"index":32,"name":"ivy_branch","type_id":88,"decorations":[{"decoration":"Offset","operands":[512]}]},{"index":33,"name":"ivy_leaf_1","type_id":88,"decorations":[{"decoration":"Offset","operands":[528]}]},{"index":34,"name":"ivy_leaf_2","type_id":88,"decorations":[{"decoration":"Offset","operands":[544]}]},{"index":35,"name":"terrain_highlight","type_id":88,"decorations":[{"decoration":"Offset","operands":[560]}]},{"index":36,"name":"night_tint","type_id":88,"decorations":[{"decoration":"Offset","operands":[576]}]}]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":46,"name":"instanced_data","storage_class":"Uniform","type":{"id":95,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,43]},"decorations":[],"pointee":{"id":43,"name":"type.StructuredBuffer.RoofSupportBeam","record":{"opcode":"OpTypeStruct","operands":[49]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":49,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.Vertex_Position","storage_class":"Input","type":{"id":96,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,57]},"decorations":[],"pointee":{"id":57,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[53,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"in.var.Vertex_Normal","storage_class":"Input","type":{"id":96,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,57]},"decorations":[],"pointee":{"id":57,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[53,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Input","type":{"id":97,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,54]},"decorations":[],"pointee":{"id":54,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":43,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[43]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Output","type":{"id":98,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,88]},"decorations":[],"pointee":{"id":88,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[53,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":0,"declared_direction":"stage_output","decorations":[{"decoration":"BuiltIn","operands":[0]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"out.var.C0","storage_class":"Output","type":{"id":99,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,57]},"decorations":[],"pointee":{"id":57,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[53,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"out.var.C1","storage_class":"Output","type":{"id":99,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,57]},"decorations":[],"pointee":{"id":57,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[53,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"out.var.C2","storage_class":"Output","type":{"id":99,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,57]},"decorations":[],"pointee":{"id":57,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[53,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Fragment","function_id":2,"entry_name":"ps_main","interface_ids":[]}],"execution_modes":[{"entry_id":2,"mode":"OriginUpperLeft","operands":[],"operand_kind":"literal"}]}
// [Shader 声明接口；Pass 实际绑定见映射状态] _instanced_roof_beams.raster.hlsl_b903e43ffb3da915.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Vertex","function_id":2,"entry_name":"vs_main","interface_ids":[3,4,5,6,7,8,9]}],"execution_modes":[]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":38,"name":"frame_constants","storage_class":"Uniform","type":{"id":91,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,36]},"decorations":[],"pointee":{"id":36,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[88,88,88,88,88,88,88,88,54,53,53,54,59,54,53,57,53,57,54,57,54,53,54,54,54,88,47]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":88,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":88,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":88,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":88,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":88,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":88,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":88,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":88,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":54,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":53,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":53,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":54,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":59,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":54,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":53,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":57,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":53,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":57,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":54,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":57,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":54,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":53,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":54,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":54,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":54,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":88,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":47,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":40,"name":"$Globals","storage_class":"Uniform","type":{"id":92,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,39]},"decorations":[],"pointee":{"id":39,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[54]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"current_view_index","type_id":54,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":42,"name":"color_library","storage_class":"Uniform","type":{"id":93,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,41]},"decorations":[],"pointee":{"id":41,"name":"type.ConstantBuffer.ColorLibrary","record":{"opcode":"OpTypeStruct","operands":[88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88,88]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"temp_color_1","type_id":88,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"temp_color_2","type_id":88,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"temp_color_3","type_id":88,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"roof_color_1","type_id":88,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"roof_color_2","type_id":88,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"sky_low_day","type_id":88,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"sky_high_day","type_id":88,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"sun_tint_day","type_id":88,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"sky_low_dusk","type_id":88,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"sky_high_dusk","type_id":88,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":10,"name":"sun_tint_dusk","type_id":88,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":11,"name":"indirect_light_tint","type_id":88,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":12,"name":"fence_color_1","type_id":88,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":13,"name":"fence_color_2","type_id":88,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":14,"name":"flower_color_1","type_id":88,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":15,"name":"flower_color_2","type_id":88,"decorations":[{"decoration":"Offset","operands":[240]}]},{"index":16,"name":"canopy_color_1","type_id":88,"decorations":[{"decoration":"Offset","operands":[256]}]},{"index":17,"name":"canopy_color_2","type_id":88,"decorations":[{"decoration":"Offset","operands":[272]}]},{"index":18,"name":"grass_1","type_id":88,"decorations":[{"decoration":"Offset","operands":[288]}]},{"index":19,"name":"grass_2","type_id":88,"decorations":[{"decoration":"Offset","operands":[304]}]},{"index":20,"name":"path","type_id":88,"decorations":[{"decoration":"Offset","operands":[320]}]},{"index":21,"name":"sky_base","type_id":88,"decorations":[{"decoration":"Offset","operands":[336]}]},{"index":22,"name":"sky_horizon","type_id":88,"decorations":[{"decoration":"Offset","operands":[352]}]},{"index":23,"name":"sky_clouds_1","type_id":88,"decorations":[{"decoration":"Offset","operands":[368]}]},{"index":24,"name":"sky_clouds_2","type_id":88,"decorations":[{"decoration":"Offset","operands":[384]}]},{"index":25,"name":"terrain_1","type_id":88,"decorations":[{"decoration":"Offset","operands":[400]}]},{"index":26,"name":"terrain_2","type_id":88,"decorations":[{"decoration":"Offset","operands":[416]}]},{"index":27,"name":"terrain_flattened","type_id":88,"decorations":[{"decoration":"Offset","operands":[432]}]},{"index":28,"name":"wall_1","type_id":88,"decorations":[{"decoration":"Offset","operands":[448]}]},{"index":29,"name":"wall_2","type_id":88,"decorations":[{"decoration":"Offset","operands":[464]}]},{"index":30,"name":"wall_moss","type_id":88,"decorations":[{"decoration":"Offset","operands":[480]}]},{"index":31,"name":"wall_bricks","type_id":88,"decorations":[{"decoration":"Offset","operands":[496]}]},{"index":32,"name":"ivy_branch","type_id":88,"decorations":[{"decoration":"Offset","operands":[512]}]},{"index":33,"name":"ivy_leaf_1","type_id":88,"decorations":[{"decoration":"Offset","operands":[528]}]},{"index":34,"name":"ivy_leaf_2","type_id":88,"decorations":[{"decoration":"Offset","operands":[544]}]},{"index":35,"name":"terrain_highlight","type_id":88,"decorations":[{"decoration":"Offset","operands":[560]}]},{"index":36,"name":"night_tint","type_id":88,"decorations":[{"decoration":"Offset","operands":[576]}]}]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":46,"name":"instanced_data","storage_class":"Uniform","type":{"id":95,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,43]},"decorations":[],"pointee":{"id":43,"name":"type.StructuredBuffer.RoofSupportBeam","record":{"opcode":"OpTypeStruct","operands":[49]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":49,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.Vertex_Position","storage_class":"Input","type":{"id":96,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,57]},"decorations":[],"pointee":{"id":57,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[53,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"in.var.Vertex_Normal","storage_class":"Input","type":{"id":96,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,57]},"decorations":[],"pointee":{"id":57,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[53,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Input","type":{"id":97,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,54]},"decorations":[],"pointee":{"id":54,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":43,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[43]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Output","type":{"id":98,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,88]},"decorations":[],"pointee":{"id":88,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[53,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":0,"declared_direction":"stage_output","decorations":[{"decoration":"BuiltIn","operands":[0]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"out.var.C0","storage_class":"Output","type":{"id":99,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,57]},"decorations":[],"pointee":{"id":57,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[53,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"out.var.C1","storage_class":"Output","type":{"id":99,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,57]},"decorations":[],"pointee":{"id":57,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[53,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"out.var.C2","storage_class":"Output","type":{"id":99,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,57]},"decorations":[],"pointee":{"id":57,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[53,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Fragment","function_id":2,"entry_name":"ps_main","interface_ids":[3,4,5]}],"execution_modes":[{"entry_id":2,"mode":"OriginUpperLeft","operands":[],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.C0","storage_class":"Input","type":{"id":44,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,34]},"decorations":[],"pointee":{"id":34,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[32,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"in.var.C1","storage_class":"Input","type":{"id":44,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,34]},"decorations":[],"pointee":{"id":34,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[32,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"out.var.SV_TARGET0","storage_class":"Output","type":{"id":45,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,36]},"decorations":[],"pointee":{"id":36,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[33,2]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _roof_tiles_gravity.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[64,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":22,"name":"frame_constants_storage_buffer","storage_class":"Uniform","type":{"id":71,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,19]},"decorations":[],"pointee":{"id":19,"name":"type.StructuredBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[29]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":29,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":1,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[1]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":24,"name":"push_constants","storage_class":"PushConstant","type":{"id":72,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[9,23]},"decorations":[],"pointee":{"id":23,"name":"type.ConstantBuffer.PushConstants","record":{"opcode":"OpTypeStruct","operands":[38,38,36,36,36]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"first_instance","type_id":38,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"instance_count","type_id":38,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"wall_height_delta","type_id":36,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":3,"name":"roof_pivot_y","type_id":36,"decorations":[{"decoration":"Offset","operands":[12]}]},{"index":4,"name":"animation_t","type_id":36,"decorations":[{"decoration":"Offset","operands":[16]}]}]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":"unknown","declared_direction":"push_constant_input","decorations":[],"access_qualifiers":["Block"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":27,"name":"__alias_instanced_data","storage_class":"Uniform","type":{"id":73,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,25]},"decorations":[],"pointee":{"id":25,"name":"type.RWStructuredBuffer.RoofShingle","record":{"opcode":"OpTypeStruct","operands":[30]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":30,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":75,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,74]},"decorations":[],"pointee":{"id":74,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[38,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _instanced_roof_spire.raster.hlsl_f0adff76827e73ff.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Vertex","function_id":2,"entry_name":"vs_main","interface_ids":[3,4,5,6,7,8,9,10,11]}],"execution_modes":[]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":41,"name":"frame_constants","storage_class":"Uniform","type":{"id":98,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,39]},"decorations":[],"pointee":{"id":39,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[95,95,95,95,95,95,95,95,54,53,53,54,60,54,53,58,53,58,54,58,54,53,54,54,54,95,47]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":95,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":95,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":95,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":95,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":95,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":95,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":95,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":95,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":54,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":53,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":53,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":54,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":60,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":54,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":53,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":58,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":53,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":58,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":54,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":58,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":54,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":53,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":54,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":54,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":54,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":95,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":47,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":43,"name":"$Globals","storage_class":"Uniform","type":{"id":99,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,42]},"decorations":[],"pointee":{"id":42,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[54]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"current_view_index","type_id":54,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":46,"name":"instanced_data","storage_class":"Uniform","type":{"id":100,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,44]},"decorations":[],"pointee":{"id":44,"name":"type.StructuredBuffer.RoofSpire","record":{"opcode":"OpTypeStruct","operands":[48]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":48,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.Vertex_Position","storage_class":"Input","type":{"id":101,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,58]},"decorations":[],"pointee":{"id":58,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[53,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"in.var.Vertex_Normal","storage_class":"Input","type":{"id":101,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,58]},"decorations":[],"pointee":{"id":58,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[53,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"in.var.roof_profile_mult","storage_class":"Input","type":{"id":102,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,53]},"decorations":[],"pointee":{"id":53,"name":"unknown","record":{"opcode":"OpTypeFloat","operands":[32]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Input","type":{"id":103,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,54]},"decorations":[],"pointee":{"id":54,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":43,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[43]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"unknown","storage_class":"Output","type":{"id":104,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,95]},"decorations":[],"pointee":{"id":95,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[53,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":0,"declared_direction":"stage_output","decorations":[{"decoration":"BuiltIn","operands":[0]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"out.var.C0","storage_class":"Output","type":{"id":105,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,58]},"decorations":[],"pointee":{"id":58,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[53,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"out.var.C1","storage_class":"Output","type":{"id":105,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,58]},"decorations":[],"pointee":{"id":58,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[53,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":10,"name":"out.var.C2","storage_class":"Output","type":{"id":105,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,58]},"decorations":[],"pointee":{"id":58,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[53,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":11,"name":"out.var.C3","storage_class":"Output","type":{"id":106,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,53]},"decorations":[],"pointee":{"id":53,"name":"unknown","record":{"opcode":"OpTypeFloat","operands":[32]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":3,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[3]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Fragment","function_id":2,"entry_name":"ps_main","interface_ids":[]}],"execution_modes":[{"entry_id":2,"mode":"OriginUpperLeft","operands":[],"operand_kind":"literal"}]}
// [Shader 声明接口；Pass 实际绑定见映射状态] _instanced_roof_spire.raster.hlsl_b903e43ffb3da915.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Vertex","function_id":2,"entry_name":"vs_main","interface_ids":[3,4,5,6,7,8,9,10,11]}],"execution_modes":[]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":41,"name":"frame_constants","storage_class":"Uniform","type":{"id":98,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,39]},"decorations":[],"pointee":{"id":39,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[95,95,95,95,95,95,95,95,54,53,53,54,60,54,53,58,53,58,54,58,54,53,54,54,54,95,47]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":95,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":95,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":95,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":95,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":95,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":95,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":95,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":95,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":54,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":53,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":53,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":54,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":60,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":54,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":53,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":58,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":53,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":58,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":54,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":58,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":54,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":53,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":54,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":54,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":54,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":95,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":47,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":43,"name":"$Globals","storage_class":"Uniform","type":{"id":99,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,42]},"decorations":[],"pointee":{"id":42,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[54]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"current_view_index","type_id":54,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":46,"name":"instanced_data","storage_class":"Uniform","type":{"id":100,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,44]},"decorations":[],"pointee":{"id":44,"name":"type.StructuredBuffer.RoofSpire","record":{"opcode":"OpTypeStruct","operands":[48]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":48,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.Vertex_Position","storage_class":"Input","type":{"id":101,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,58]},"decorations":[],"pointee":{"id":58,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[53,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"in.var.Vertex_Normal","storage_class":"Input","type":{"id":101,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,58]},"decorations":[],"pointee":{"id":58,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[53,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"in.var.roof_profile_mult","storage_class":"Input","type":{"id":102,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,53]},"decorations":[],"pointee":{"id":53,"name":"unknown","record":{"opcode":"OpTypeFloat","operands":[32]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"unknown","storage_class":"Input","type":{"id":103,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,54]},"decorations":[],"pointee":{"id":54,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":43,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[43]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"unknown","storage_class":"Output","type":{"id":104,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,95]},"decorations":[],"pointee":{"id":95,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[53,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":0,"declared_direction":"stage_output","decorations":[{"decoration":"BuiltIn","operands":[0]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"out.var.C0","storage_class":"Output","type":{"id":105,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,58]},"decorations":[],"pointee":{"id":58,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[53,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"out.var.C1","storage_class":"Output","type":{"id":105,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,58]},"decorations":[],"pointee":{"id":58,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[53,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":10,"name":"out.var.C2","storage_class":"Output","type":{"id":105,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,58]},"decorations":[],"pointee":{"id":58,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[53,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":11,"name":"out.var.C3","storage_class":"Output","type":{"id":106,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,53]},"decorations":[],"pointee":{"id":53,"name":"unknown","record":{"opcode":"OpTypeFloat","operands":[32]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":3,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[3]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Fragment","function_id":2,"entry_name":"ps_main","interface_ids":[3,4,5,6,7]}],"execution_modes":[{"entry_id":2,"mode":"OriginUpperLeft","operands":[],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.C0","storage_class":"Input","type":{"id":47,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,37]},"decorations":[],"pointee":{"id":37,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[35,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"in.var.C1","storage_class":"Input","type":{"id":47,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,37]},"decorations":[],"pointee":{"id":37,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[35,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[1]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"in.var.C2","storage_class":"Input","type":{"id":47,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,37]},"decorations":[],"pointee":{"id":37,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[35,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"in.var.C3","storage_class":"Input","type":{"id":48,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,35]},"decorations":[],"pointee":{"id":35,"name":"unknown","record":{"opcode":"OpTypeFloat","operands":[32]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":3,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[3]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"out.var.SV_TARGET0","storage_class":"Output","type":{"id":49,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,39]},"decorations":[],"pointee":{"id":39,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[36,2]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// 父函数直接调用位置 = unknown/当前扫描未找到；PDB 内联及间接调用不能据此排除。
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: instanced_roof.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::writer::EventWriter<country_core::systems::backend::grow_grass::GrowGrass>,bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::event::writer::EventWriter<country_core::systems::backend::grow_grass::GrowGrass>", "bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::backend::grow_grass::GrowGrass>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [文件级接口候选，具体函数归属 unknown] core::char::EscapeDebug (*)(char)
// 候选参数类型: ["char"]

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_shader_nani
// 信息 ID: 2ed5853c157563a3c420a95b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_spire_shader
// 信息 ID: cf6e0d4204cd00d033e42716
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_spire_shader::closure$0
// 信息 ID: 579718bdc289db9667dde84f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_spire_shader", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_spire_shader; module=532, inline_record=57504; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_support_beam_shader
// 信息 ID: e05a527d936117eaf4982b81
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_support_beam_shader::closure$0
// 信息 ID: 0a16a919123fbd5d4fdd0ed1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_support_beam_shader", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_support_beam_shader; module=532, inline_record=58976; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
