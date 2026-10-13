// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/draw_bricks.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 492363e4c963855d9d61abaf9e1fa09e437b1b847e7e0fac3b21793e3f3efac7
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: draw_bricks.rs.pdb.jsonl.gz
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
// use modular_bitfield; // namespace candidate
//   package identities: [{"name":"modular-bitfield","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks<0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":487,"record_offset":40376,"rva":9065424,"va":5377774544}
// 编译实例 RVAs: 0x8a53d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=2
//     target_aliases=19 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8158781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377774544,"raw_hex":"e8c38183ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5377774888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-74683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377774544,"raw_hex":"e845dcfeff","size":5,"soft_fail":false,"target_va":5377701312,"va":5377775990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t405092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377774544,"raw_hex":"e8642e0600","size":5,"soft_fail":false,"target_va":5378180624,"va":5377775527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14090bb70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t418817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377774544,"raw_hex":"e801640600","size":5,"soft_fail":false,"target_va":5378194288,"va":5377775466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14090be10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t419512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377774544,"raw_hex":"e8b8660600","size":5,"soft_fail":false,"target_va":5378194960,"va":5377775443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140969720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t803217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377774544,"raw_hex":"e891410c00","size":5,"soft_fail":false,"target_va":5378578208,"va":5377774986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t810333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377774544,"raw_hex":"e85d5d0c00","size":5,"soft_fail":false,"target_va":5378585968,"va":5377775630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162ad90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14177707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377774544,"raw_hex":"e8ab55d800","size":5,"soft_fail":false,"target_va":5391953296,"va":5377775584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162d190
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14187302","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377774544,"raw_hex":"e8267bd800","size":5,"soft_fail":false,"target_va":5391962512,"va":5377775205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34072085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377774544,"raw_hex":"e815e60702","size":5,"soft_fail":false,"target_va":5411847152,"va":5377775062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34072405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377774544,"raw_hex":"e855e70702","size":5,"soft_fail":false,"target_va":5411847168,"va":5377774758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::unsafe_world_cell::UnsafeEntityCell::get<country_core::geometry::instanced_wall::WallBrickInstancedDrawProxy>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-5165","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377774544,"raw_hex":"e8d3ebffff","size":5,"soft_fail":false,"target_va":5377769552,"va":5377774712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t33765895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377774544,"raw_hex":"e8073a0302","size":5,"soft_fail":false,"target_va":5411541776,"va":5377775876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33706635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377774544,"raw_hex":"e88b520202","size":5,"soft_fail":false,"target_va":5411482433,"va":5377775793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33767020","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377774544,"raw_hex":"e86c3e0302","size":5,"soft_fail":false,"target_va":5411542880,"va":5377775855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_external_storage_buffer
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t396730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377774544,"raw_hex":"e8ba0d0600","size":5,"soft_fail":false,"target_va":5378171904,"va":5377775169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::discard
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t404998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377774544,"raw_hex":"e8062e0600","size":5,"soft_fail":false,"target_va":5378179776,"va":5377774773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::multi_draw_indexed_indirect
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t401697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377774544,"raw_hex":"e821210600","size":5,"soft_fail":false,"target_va":5378177424,"va":5377775722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5377774544,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5377775093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_entity
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":487,"record_offset":40632,"parent_record_offset":40376,"ancestor_record_offset":40376,"inlinee":27054,"source_line":360,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":487,"record_offset":41232,"parent_record_offset":40376,"ancestor_record_offset":40376,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":487,"record_offset":41088,"parent_record_offset":40376,"ancestor_record_offset":40376,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":487,"record_offset":41052,"parent_record_offset":40376,"ancestor_record_offset":40376,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":487,"record_offset":41016,"parent_record_offset":40376,"ancestor_record_offset":40376,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":487,"record_offset":42020,"parent_record_offset":40376,"ancestor_record_offset":40376,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::frame_constants::RenderViewIndex::is_shadow_view
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/frame_constants.rs
//     evidence={"module":487,"record_offset":40604,"parent_record_offset":40376,"ancestor_record_offset":40376,"inlinee":27053,"source_line":73,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_roof_color_customization
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs
//     evidence={"module":487,"record_offset":43100,"parent_record_offset":40376,"ancestor_record_offset":40376,"inlinee":27058,"source_line":84,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_indexed_indirect
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":487,"record_offset":43248,"parent_record_offset":40376,"ancestor_record_offset":40376,"inlinee":26793,"source_line":384,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":487,"record_offset":43176,"parent_record_offset":40376,"ancestor_record_offset":40376,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::buffer
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":487,"record_offset":42296,"parent_record_offset":40376,"ancestor_record_offset":40376,"inlinee":27057,"source_line":587,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":487,"record_offset":41144,"parent_record_offset":40376,"ancestor_record_offset":40376,"inlinee":27055,"source_line":624,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":487,"record_offset":40836,"parent_record_offset":40812,"ancestor_record_offset":40376,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":487,"record_offset":40884,"parent_record_offset":40860,"ancestor_record_offset":40376,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":487,"record_offset":40860,"parent_record_offset":40836,"ancestor_record_offset":40376,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":487,"record_offset":40812,"parent_record_offset":40788,"ancestor_record_offset":40376,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":487,"record_offset":40788,"parent_record_offset":40688,"ancestor_record_offset":40376,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":487,"record_offset":41580,"parent_record_offset":41556,"ancestor_record_offset":40376,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":487,"record_offset":40984,"parent_record_offset":40656,"ancestor_record_offset":40376,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":487,"record_offset":40656,"parent_record_offset":40632,"ancestor_record_offset":40376,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":487,"record_offset":41968,"parent_record_offset":41944,"ancestor_record_offset":40376,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":487,"record_offset":41944,"parent_record_offset":41920,"ancestor_record_offset":40376,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":487,"record_offset":41892,"parent_record_offset":41860,"ancestor_record_offset":40376,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":487,"record_offset":41836,"parent_record_offset":41300,"ancestor_record_offset":40376,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":487,"record_offset":41860,"parent_record_offset":41836,"ancestor_record_offset":40376,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":487,"record_offset":41328,"parent_record_offset":41300,"ancestor_record_offset":40376,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":487,"record_offset":41920,"parent_record_offset":41860,"ancestor_record_offset":40376,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":487,"record_offset":41376,"parent_record_offset":41352,"ancestor_record_offset":40376,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":487,"record_offset":41352,"parent_record_offset":41328,"ancestor_record_offset":40376,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":487,"record_offset":41556,"parent_record_offset":41532,"ancestor_record_offset":40376,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":487,"record_offset":41260,"parent_record_offset":41232,"ancestor_record_offset":40376,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":487,"record_offset":41300,"parent_record_offset":41232,"ancestor_record_offset":40376,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":487,"record_offset":40956,"parent_record_offset":40932,"ancestor_record_offset":40376,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":487,"record_offset":42088,"parent_record_offset":42048,"ancestor_record_offset":40376,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":487,"record_offset":40932,"parent_record_offset":40656,"ancestor_record_offset":40376,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":487,"record_offset":41468,"parent_record_offset":41404,"ancestor_record_offset":40376,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":487,"record_offset":43216,"parent_record_offset":43176,"ancestor_record_offset":40376,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":487,"record_offset":41492,"parent_record_offset":41468,"ancestor_record_offset":40376,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":487,"record_offset":41532,"parent_record_offset":41352,"ancestor_record_offset":40376,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":487,"record_offset":41112,"parent_record_offset":41088,"ancestor_record_offset":40376,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":487,"record_offset":42352,"parent_record_offset":42324,"ancestor_record_offset":40376,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":487,"record_offset":42172,"parent_record_offset":42144,"ancestor_record_offset":40376,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":487,"record_offset":42200,"parent_record_offset":42172,"ancestor_record_offset":40376,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":487,"record_offset":40688,"parent_record_offset":40656,"ancestor_record_offset":40376,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":487,"record_offset":40756,"parent_record_offset":40728,"ancestor_record_offset":40376,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":487,"record_offset":40728,"parent_record_offset":40688,"ancestor_record_offset":40376,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":487,"record_offset":41760,"parent_record_offset":41580,"ancestor_record_offset":40376,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":487,"record_offset":42116,"parent_record_offset":42088,"ancestor_record_offset":40376,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":487,"record_offset":42048,"parent_record_offset":42020,"ancestor_record_offset":40376,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":487,"record_offset":43128,"parent_record_offset":43100,"ancestor_record_offset":40376,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":487,"record_offset":41192,"parent_record_offset":41168,"ancestor_record_offset":40376,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":487,"record_offset":41168,"parent_record_offset":41144,"ancestor_record_offset":40376,"inlinee":27056,"source_line":624,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks<1>
// 记录形式: compiled_procedure; instances=1
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":487,"record_offset":37392,"rva":9063584,"va":5377772704}
// 编译实例 RVAs: 0x8a4ca0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=2
//     target_aliases=19 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8156880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377772704,"raw_hex":"e8308983ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5377772987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-72929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377772704,"raw_hex":"e81fe3feff","size":5,"soft_fail":false,"target_va":5377701312,"va":5377774236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t406899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377772704,"raw_hex":"e873350600","size":5,"soft_fail":false,"target_va":5378180624,"va":5377773720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14090bb70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t420624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377772704,"raw_hex":"e8106b0600","size":5,"soft_fail":false,"target_va":5378194288,"va":5377773659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14090be10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t421319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377772704,"raw_hex":"e8c76d0600","size":5,"soft_fail":false,"target_va":5378194960,"va":5377773636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140969720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t805118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377772704,"raw_hex":"e8fe480c00","size":5,"soft_fail":false,"target_va":5378578208,"va":5377773085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t812140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377772704,"raw_hex":"e86c640c00","size":5,"soft_fail":false,"target_va":5378585968,"va":5377773823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162ad90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14179514","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377772704,"raw_hex":"e8ba5cd800","size":5,"soft_fail":false,"target_va":5391953296,"va":5377773777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162d190
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14189203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377772704,"raw_hex":"e89382d800","size":5,"soft_fail":false,"target_va":5391962512,"va":5377773304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34073986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377772704,"raw_hex":"e882ed0702","size":5,"soft_fail":false,"target_va":5411847152,"va":5377773161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: draw_bricks.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34073934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377772704,"raw_hex":"e84eed0702","size":5,"soft_fail":false,"target_va":5411847168,"va":5377773229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::unsafe_world_cell::UnsafeEntityCell::get<country_core::geometry::instanced_wall::WallBrickInstancedDrawProxy>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-3345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377772704,"raw_hex":"e8eff2ffff","size":5,"soft_fail":false,"target_va":5377769552,"va":5377772892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t33767683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377772704,"raw_hex":"e803410302","size":5,"soft_fail":false,"target_va":5411541776,"va":5377774088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33708423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377772704,"raw_hex":"e887590202","size":5,"soft_fail":false,"target_va":5411482433,"va":5377774005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33767875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377772704,"raw_hex":"e8c3410302","size":5,"soft_fail":false,"target_va":5411542064,"va":5377774184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33768808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377772704,"raw_hex":"e868450302","size":5,"soft_fail":false,"target_va":5411542880,"va":5377774067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_external_storage_buffer
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t398631","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377772704,"raw_hex":"e827150600","size":5,"soft_fail":false,"target_va":5378171904,"va":5377773268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::discard
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t406378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377772704,"raw_hex":"e86a330600","size":5,"soft_fail":false,"target_va":5378179776,"va":5377773393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::multi_draw_indexed_indirect
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t403489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377772704,"raw_hex":"e821280600","size":5,"soft_fail":false,"target_va":5378177424,"va":5377773930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5377772704,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5377773192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_entity
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":487,"record_offset":37648,"parent_record_offset":37392,"ancestor_record_offset":37392,"inlinee":27054,"source_line":360,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":487,"record_offset":38244,"parent_record_offset":37392,"ancestor_record_offset":37392,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":487,"record_offset":38104,"parent_record_offset":37392,"ancestor_record_offset":37392,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":487,"record_offset":38068,"parent_record_offset":37392,"ancestor_record_offset":37392,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":487,"record_offset":38032,"parent_record_offset":37392,"ancestor_record_offset":37392,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":487,"record_offset":39032,"parent_record_offset":37392,"ancestor_record_offset":37392,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::frame_constants::RenderViewIndex::is_shadow_view
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/frame_constants.rs
//     evidence={"module":487,"record_offset":37620,"parent_record_offset":37392,"ancestor_record_offset":37392,"inlinee":27053,"source_line":73,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_roof_color_customization
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs
//     evidence={"module":487,"record_offset":40112,"parent_record_offset":37392,"ancestor_record_offset":37392,"inlinee":27058,"source_line":84,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_indexed_indirect
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":487,"record_offset":40260,"parent_record_offset":37392,"ancestor_record_offset":37392,"inlinee":26793,"source_line":384,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":487,"record_offset":40188,"parent_record_offset":37392,"ancestor_record_offset":37392,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::buffer
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":487,"record_offset":39308,"parent_record_offset":37392,"ancestor_record_offset":37392,"inlinee":27057,"source_line":587,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":487,"record_offset":38160,"parent_record_offset":37392,"ancestor_record_offset":37392,"inlinee":27055,"source_line":624,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":487,"record_offset":37852,"parent_record_offset":37828,"ancestor_record_offset":37392,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":487,"record_offset":37900,"parent_record_offset":37876,"ancestor_record_offset":37392,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":487,"record_offset":37876,"parent_record_offset":37852,"ancestor_record_offset":37392,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":487,"record_offset":37828,"parent_record_offset":37804,"ancestor_record_offset":37392,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":487,"record_offset":37804,"parent_record_offset":37704,"ancestor_record_offset":37392,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":487,"record_offset":38592,"parent_record_offset":38568,"ancestor_record_offset":37392,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":487,"record_offset":38000,"parent_record_offset":37672,"ancestor_record_offset":37392,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":487,"record_offset":37672,"parent_record_offset":37648,"ancestor_record_offset":37392,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":487,"record_offset":38980,"parent_record_offset":38956,"ancestor_record_offset":37392,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":487,"record_offset":38956,"parent_record_offset":38932,"ancestor_record_offset":37392,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":487,"record_offset":38904,"parent_record_offset":38872,"ancestor_record_offset":37392,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":487,"record_offset":38848,"parent_record_offset":38312,"ancestor_record_offset":37392,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":487,"record_offset":38872,"parent_record_offset":38848,"ancestor_record_offset":37392,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":487,"record_offset":38340,"parent_record_offset":38312,"ancestor_record_offset":37392,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":487,"record_offset":38932,"parent_record_offset":38872,"ancestor_record_offset":37392,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":487,"record_offset":38388,"parent_record_offset":38364,"ancestor_record_offset":37392,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":487,"record_offset":38364,"parent_record_offset":38340,"ancestor_record_offset":37392,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":487,"record_offset":38568,"parent_record_offset":38544,"ancestor_record_offset":37392,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":487,"record_offset":38272,"parent_record_offset":38244,"ancestor_record_offset":37392,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":487,"record_offset":38312,"parent_record_offset":38244,"ancestor_record_offset":37392,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":487,"record_offset":37972,"parent_record_offset":37948,"ancestor_record_offset":37392,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":487,"record_offset":39100,"parent_record_offset":39060,"ancestor_record_offset":37392,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":487,"record_offset":37948,"parent_record_offset":37672,"ancestor_record_offset":37392,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":487,"record_offset":38480,"parent_record_offset":38416,"ancestor_record_offset":37392,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":487,"record_offset":40228,"parent_record_offset":40188,"ancestor_record_offset":37392,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":487,"record_offset":38504,"parent_record_offset":38480,"ancestor_record_offset":37392,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":487,"record_offset":38544,"parent_record_offset":38364,"ancestor_record_offset":37392,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":487,"record_offset":38128,"parent_record_offset":38104,"ancestor_record_offset":37392,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":487,"record_offset":39364,"parent_record_offset":39336,"ancestor_record_offset":37392,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":487,"record_offset":39184,"parent_record_offset":39156,"ancestor_record_offset":37392,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":487,"record_offset":39212,"parent_record_offset":39184,"ancestor_record_offset":37392,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":487,"record_offset":37704,"parent_record_offset":37672,"ancestor_record_offset":37392,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":487,"record_offset":37772,"parent_record_offset":37744,"ancestor_record_offset":37392,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":487,"record_offset":37744,"parent_record_offset":37704,"ancestor_record_offset":37392,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":487,"record_offset":38772,"parent_record_offset":38592,"ancestor_record_offset":37392,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":487,"record_offset":39128,"parent_record_offset":39100,"ancestor_record_offset":37392,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":487,"record_offset":39060,"parent_record_offset":39032,"ancestor_record_offset":37392,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":487,"record_offset":40140,"parent_record_offset":40112,"ancestor_record_offset":37392,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":487,"record_offset":38208,"parent_record_offset":38184,"ancestor_record_offset":37392,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":487,"record_offset":38184,"parent_record_offset":38160,"ancestor_record_offset":37392,"inlinee":27056,"source_line":624,"archive":"_pdb/B/modules/00487.jsonl.gz"}
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
// 完整结构及原始类型记录: draw_bricks.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: draw_bricks.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/draw_bricks.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: draw_bricks.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks<0>
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":487,"record_offset":43128,"parent_record_offset":43100,"ancestor_record_offset":40376,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00487.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::uniform","relationship":"pdb_inline_direct","evidence":{"module":487,"record_offset":43176,"parent_record_offset":40376,"ancestor_record_offset":40376,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00487.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks<1>
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":487,"record_offset":40140,"parent_record_offset":40112,"ancestor_record_offset":37392,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00487.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::uniform","relationship":"pdb_inline_direct","evidence":{"module":487,"record_offset":40188,"parent_record_offset":37392,"ancestor_record_offset":37392,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00487.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// 父函数直接调用位置 = unknown/当前扫描未找到；PDB 内联及间接调用不能据此排除。
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: draw_bricks.rs.enriched.json.gz
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
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks<0>
// 信息 ID: c736c1d0c233ac1f14d47151
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks -> ["0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks<1>
// 信息 ID: 80ed21a195d6bd617a973d80
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks -> ["1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
