// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/instanced_critters.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 5732409f3407174df9d2466f0ebf01dfa40f907a646aae2bd608be3393e64fc6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: instanced_critters.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1615
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
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use modular_bitfield; // namespace candidate
//   package identities: [{"name":"modular-bitfield","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_critters::draw_instanced_butterfly_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1615,"record_offset":50172,"rva":22366176,"va":5391075296}
// 编译实例 RVAs: 0x15547e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13374335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391075296,"raw_hex":"e881ec33ff","size":5,"soft_fail":false,"target_va":5377701312,"va":5391075642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12894936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391075296,"raw_hex":"e8283d3bff","size":5,"soft_fail":false,"target_va":5378180624,"va":5391075555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908c50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12893162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391075296,"raw_hex":"e816443bff","size":5,"soft_fail":false,"target_va":5378182224,"va":5391075381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162e390
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t891617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391075296,"raw_hex":"e8e19a0d00","size":5,"soft_fail":false,"target_va":5391967120,"va":5391075498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20771754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391075296,"raw_hex":"e8aaf33c01","size":5,"soft_fail":false,"target_va":5411847168,"va":5391075409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_instanced_w_gl_draw_flag
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12899450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391075296,"raw_hex":"e8862b3bff","size":5,"soft_fail":false,"target_va":5378176144,"va":5391075589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_instanced
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":1615,"record_offset":50556,"parent_record_offset":50172,"ancestor_record_offset":50172,"inlinee":27909,"source_line":298,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":1615,"record_offset":50508,"parent_record_offset":50172,"ancestor_record_offset":50172,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":1615,"record_offset":50380,"parent_record_offset":50172,"ancestor_record_offset":50172,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":1615,"record_offset":50408,"parent_record_offset":50380,"ancestor_record_offset":50172,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_critters::draw_instanced_firefly_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1615,"record_offset":50588,"rva":22366640,"va":5391075760}
// 编译实例 RVAs: 0x15549b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=19 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21459730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391075760,"raw_hex":"e8ee8cb8fe","size":5,"soft_fail":false,"target_va":5369616112,"va":5391075837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13375067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391075760,"raw_hex":"e8a5e933ff","size":5,"soft_fail":false,"target_va":5377701312,"va":5391076374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908c50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12893844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391075760,"raw_hex":"e86c413bff","size":5,"soft_fail":false,"target_va":5378182224,"va":5391076063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096ba30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12489030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391075760,"raw_hex":"e8ba6e41ff","size":5,"soft_fail":false,"target_va":5378587184,"va":5391076209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162e090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t890081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391075760,"raw_hex":"e8e1940d00","size":5,"soft_fail":false,"target_va":5391966352,"va":5391076266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20771205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391075760,"raw_hex":"e885f13c01","size":5,"soft_fail":false,"target_va":5411847168,"va":5391075958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20465447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391075760,"raw_hex":"e827473801","size":5,"soft_fail":false,"target_va":5411541776,"va":5391076324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::discard
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12896202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391075760,"raw_hex":"e836383bff","size":5,"soft_fail":false,"target_va":5378179776,"va":5391075973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_instanced_w_gl_draw_flag
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12900148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391075760,"raw_hex":"e8cc283bff","size":5,"soft_fail":false,"target_va":5378176144,"va":5391076287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_instanced
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":1615,"record_offset":51832,"parent_record_offset":50588,"ancestor_record_offset":50588,"inlinee":27909,"source_line":298,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":1615,"record_offset":51760,"parent_record_offset":50588,"ancestor_record_offset":50588,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":1615,"record_offset":51632,"parent_record_offset":50588,"ancestor_record_offset":50588,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":1615,"record_offset":50796,"parent_record_offset":50588,"ancestor_record_offset":50588,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1615,"record_offset":51248,"parent_record_offset":51224,"ancestor_record_offset":50588,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1615,"record_offset":51296,"parent_record_offset":51272,"ancestor_record_offset":50588,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1615,"record_offset":51272,"parent_record_offset":51248,"ancestor_record_offset":50588,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1615,"record_offset":51224,"parent_record_offset":51200,"ancestor_record_offset":50588,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1615,"record_offset":51200,"parent_record_offset":51160,"ancestor_record_offset":50588,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1615,"record_offset":51160,"parent_record_offset":51136,"ancestor_record_offset":50588,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":1615,"record_offset":51548,"parent_record_offset":51524,"ancestor_record_offset":50588,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":1615,"record_offset":51524,"parent_record_offset":51500,"ancestor_record_offset":50588,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":1615,"record_offset":51472,"parent_record_offset":51440,"ancestor_record_offset":50588,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":1615,"record_offset":51416,"parent_record_offset":50888,"ancestor_record_offset":50588,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":1615,"record_offset":51440,"parent_record_offset":51416,"ancestor_record_offset":50588,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":1615,"record_offset":50916,"parent_record_offset":50888,"ancestor_record_offset":50588,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":1615,"record_offset":51500,"parent_record_offset":51440,"ancestor_record_offset":50588,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1615,"record_offset":50964,"parent_record_offset":50940,"ancestor_record_offset":50588,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1615,"record_offset":50940,"parent_record_offset":50916,"ancestor_record_offset":50588,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1615,"record_offset":51136,"parent_record_offset":51112,"ancestor_record_offset":50588,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":1615,"record_offset":50852,"parent_record_offset":50824,"ancestor_record_offset":50588,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":1615,"record_offset":50824,"parent_record_offset":50796,"ancestor_record_offset":50588,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":1615,"record_offset":50888,"parent_record_offset":50824,"ancestor_record_offset":50588,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1615,"record_offset":51052,"parent_record_offset":50992,"ancestor_record_offset":50588,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1615,"record_offset":51800,"parent_record_offset":51760,"ancestor_record_offset":50588,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1615,"record_offset":51072,"parent_record_offset":51052,"ancestor_record_offset":50588,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1615,"record_offset":51112,"parent_record_offset":50940,"ancestor_record_offset":50588,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1615,"record_offset":51600,"parent_record_offset":50796,"ancestor_record_offset":50588,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1615,"record_offset":50992,"parent_record_offset":50964,"ancestor_record_offset":50588,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1615,"record_offset":51364,"parent_record_offset":51340,"ancestor_record_offset":50588,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1615,"record_offset":51024,"parent_record_offset":50992,"ancestor_record_offset":50588,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1615,"record_offset":51340,"parent_record_offset":51160,"ancestor_record_offset":50588,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":1615,"record_offset":51668,"parent_record_offset":51632,"ancestor_record_offset":50588,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_critters::draw_instanced_firefly_glow_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1615,"record_offset":51864,"rva":22367472,"va":5391076592}
// 编译实例 RVAs: 0x1554cf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=19 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21460562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391076592,"raw_hex":"e8ae89b8fe","size":5,"soft_fail":false,"target_va":5369616112,"va":5391076669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13375979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391076592,"raw_hex":"e815e633ff","size":5,"soft_fail":false,"target_va":5377701312,"va":5391077286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908c50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12894683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391076592,"raw_hex":"e8253e3bff","size":5,"soft_fail":false,"target_va":5378182224,"va":5391076902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096ba30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12489869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391076592,"raw_hex":"e8736b41ff","size":5,"soft_fail":false,"target_va":5378587184,"va":5391077048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162d790
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t886872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391076592,"raw_hex":"e858880d00","size":5,"soft_fail":false,"target_va":5391964048,"va":5391077171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162e090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t889242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391076592,"raw_hex":"e89a910d00","size":5,"soft_fail":false,"target_va":5391966352,"va":5391077105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162fb90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t896121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391076592,"raw_hex":"e879ac0d00","size":5,"soft_fail":false,"target_va":5391973264,"va":5391077138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20770373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391076592,"raw_hex":"e845ee3c01","size":5,"soft_fail":false,"target_va":5411847168,"va":5391076790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20464542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391076592,"raw_hex":"e89e433801","size":5,"soft_fail":false,"target_va":5411541776,"va":5391077229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::discard
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12897034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391076592,"raw_hex":"e8f6343bff","size":5,"soft_fail":false,"target_va":5378179776,"va":5391076805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_instanced_w_gl_draw_flag
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12901053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391076592,"raw_hex":"e843253bff","size":5,"soft_fail":false,"target_va":5378176144,"va":5391077192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_instanced
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":1615,"record_offset":53204,"parent_record_offset":51864,"ancestor_record_offset":51864,"inlinee":27909,"source_line":298,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":1615,"record_offset":53132,"parent_record_offset":51864,"ancestor_record_offset":51864,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":1615,"record_offset":53004,"parent_record_offset":51864,"ancestor_record_offset":51864,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":1615,"record_offset":52084,"parent_record_offset":51864,"ancestor_record_offset":51864,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1615,"record_offset":52920,"parent_record_offset":51864,"ancestor_record_offset":51864,"inlinee":8285,"source_line":628,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1615,"record_offset":52536,"parent_record_offset":52512,"ancestor_record_offset":51864,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1615,"record_offset":52584,"parent_record_offset":52560,"ancestor_record_offset":51864,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1615,"record_offset":52560,"parent_record_offset":52536,"ancestor_record_offset":51864,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1615,"record_offset":52512,"parent_record_offset":52488,"ancestor_record_offset":51864,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1615,"record_offset":52488,"parent_record_offset":52448,"ancestor_record_offset":51864,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1615,"record_offset":52448,"parent_record_offset":52424,"ancestor_record_offset":51864,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":1615,"record_offset":52836,"parent_record_offset":52812,"ancestor_record_offset":51864,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":1615,"record_offset":52812,"parent_record_offset":52788,"ancestor_record_offset":51864,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":1615,"record_offset":52760,"parent_record_offset":52728,"ancestor_record_offset":51864,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":1615,"record_offset":52704,"parent_record_offset":52176,"ancestor_record_offset":51864,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":1615,"record_offset":52728,"parent_record_offset":52704,"ancestor_record_offset":51864,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":1615,"record_offset":52204,"parent_record_offset":52176,"ancestor_record_offset":51864,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":1615,"record_offset":52788,"parent_record_offset":52728,"ancestor_record_offset":51864,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1615,"record_offset":52252,"parent_record_offset":52228,"ancestor_record_offset":51864,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1615,"record_offset":52228,"parent_record_offset":52204,"ancestor_record_offset":51864,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1615,"record_offset":52424,"parent_record_offset":52400,"ancestor_record_offset":51864,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":1615,"record_offset":52140,"parent_record_offset":52112,"ancestor_record_offset":51864,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":1615,"record_offset":52112,"parent_record_offset":52084,"ancestor_record_offset":51864,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":1615,"record_offset":52176,"parent_record_offset":52112,"ancestor_record_offset":51864,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1615,"record_offset":52340,"parent_record_offset":52280,"ancestor_record_offset":51864,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1615,"record_offset":53172,"parent_record_offset":53132,"ancestor_record_offset":51864,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1615,"record_offset":52360,"parent_record_offset":52340,"ancestor_record_offset":51864,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1615,"record_offset":52400,"parent_record_offset":52228,"ancestor_record_offset":51864,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1615,"record_offset":52888,"parent_record_offset":52084,"ancestor_record_offset":51864,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1615,"record_offset":52280,"parent_record_offset":52252,"ancestor_record_offset":51864,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1615,"record_offset":52652,"parent_record_offset":52628,"ancestor_record_offset":51864,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1615,"record_offset":52312,"parent_record_offset":52280,"ancestor_record_offset":51864,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1615,"record_offset":52628,"parent_record_offset":52448,"ancestor_record_offset":51864,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":1615,"record_offset":53040,"parent_record_offset":53004,"ancestor_record_offset":51864,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":1615,"record_offset":52968,"parent_record_offset":52944,"ancestor_record_offset":51864,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1615,"record_offset":52944,"parent_record_offset":52920,"ancestor_record_offset":51864,"inlinee":8286,"source_line":628,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_critters::draw_duck_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 80
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1615,"record_offset":53236,"rva":22368384,"va":5391077504}
// 编译实例 RVAs: 0x1555080
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=10
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13378022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391077504,"raw_hex":"e81ade33ff","size":5,"soft_fail":false,"target_va":5377701312,"va":5391079329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908c50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12895836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391077504,"raw_hex":"e8a4393bff","size":5,"soft_fail":false,"target_va":5378182224,"va":5391078055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b570
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12492935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391077504,"raw_hex":"e8795f41ff","size":5,"soft_fail":false,"target_va":5378585968,"va":5391078898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12492042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391077504,"raw_hex":"e8f66241ff","size":5,"soft_fail":false,"target_va":5378586704,"va":5391078741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096ba30
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12491410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391077504,"raw_hex":"e86e6541ff","size":5,"soft_fail":false,"target_va":5378587184,"va":5391078589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096bc10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12490857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391077504,"raw_hex":"e8976741ff","size":5,"soft_fail":false,"target_va":5378587664,"va":5391078516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20769162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391077504,"raw_hex":"e88ae93c01","size":5,"soft_fail":false,"target_va":5411847168,"va":5391078001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::component::Components::get_id
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12378036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391077504,"raw_hex":"e84c2043ff","size":5,"soft_fail":false,"target_va":5378699680,"va":5391077711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::storage::table::Table::get_component
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-8734274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391077504,"raw_hex":"e8beb97aff","size":5,"soft_fail":false,"target_va":5382343504,"va":5391077773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t20462589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391077504,"raw_hex":"e8fd3b3801","size":5,"soft_fail":false,"target_va":5411541776,"va":5391079182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20463714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391077504,"raw_hex":"e862403801","size":5,"soft_fail":false,"target_va":5411542880,"va":5391079161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12904243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391077504,"raw_hex":"e8cd183bff","size":5,"soft_fail":false,"target_va":5378174784,"va":5391079022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeEntityCell::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":1615,"record_offset":53836,"parent_record_offset":53236,"ancestor_record_offset":53236,"inlinee":26783,"source_line":785,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_entity
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":1615,"record_offset":53444,"parent_record_offset":53236,"ancestor_record_offset":53236,"inlinee":27054,"source_line":360,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1615,"record_offset":54080,"parent_record_offset":53236,"ancestor_record_offset":53236,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1615,"record_offset":56020,"parent_record_offset":53236,"ancestor_record_offset":53236,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":1615,"record_offset":54780,"parent_record_offset":53236,"ancestor_record_offset":53236,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":1615,"record_offset":54652,"parent_record_offset":53236,"ancestor_record_offset":53236,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_matrix
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":1615,"record_offset":54864,"parent_record_offset":53236,"ancestor_record_offset":53236,"inlinee":25133,"source_line":24,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1615,"record_offset":53648,"parent_record_offset":53624,"ancestor_record_offset":53236,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1615,"record_offset":53696,"parent_record_offset":53672,"ancestor_record_offset":53236,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1615,"record_offset":53672,"parent_record_offset":53648,"ancestor_record_offset":53236,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1615,"record_offset":53624,"parent_record_offset":53600,"ancestor_record_offset":53236,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1615,"record_offset":53600,"parent_record_offset":53500,"ancestor_record_offset":53236,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":1615,"record_offset":53800,"parent_record_offset":53468,"ancestor_record_offset":53236,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":1615,"record_offset":53468,"parent_record_offset":53444,"ancestor_record_offset":53236,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1615,"record_offset":53956,"parent_record_offset":53932,"ancestor_record_offset":53236,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":1615,"record_offset":53864,"parent_record_offset":53836,"ancestor_record_offset":53236,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::fetch_table
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":1615,"record_offset":53932,"parent_record_offset":53904,"ancestor_record_offset":53236,"inlinee":26791,"source_line":1161,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::get_component
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":1615,"record_offset":53904,"parent_record_offset":53836,"ancestor_record_offset":53236,"inlinee":26790,"source_line":1189,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1615,"record_offset":53768,"parent_record_offset":53744,"ancestor_record_offset":53236,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1615,"record_offset":55104,"parent_record_offset":55024,"ancestor_record_offset":53236,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1615,"record_offset":55376,"parent_record_offset":55324,"ancestor_record_offset":53236,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1615,"record_offset":55348,"parent_record_offset":55324,"ancestor_record_offset":53236,"inlinee":24651,"source_line":910,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1615,"record_offset":53744,"parent_record_offset":53468,"ancestor_record_offset":53236,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1615,"record_offset":54832,"parent_record_offset":54780,"ancestor_record_offset":53236,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1615,"record_offset":54144,"parent_record_offset":54116,"ancestor_record_offset":53236,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1615,"record_offset":53500,"parent_record_offset":53468,"ancestor_record_offset":53236,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1615,"record_offset":53568,"parent_record_offset":53540,"ancestor_record_offset":53236,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1615,"record_offset":53540,"parent_record_offset":53500,"ancestor_record_offset":53236,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":1615,"record_offset":54688,"parent_record_offset":54652,"ancestor_record_offset":53236,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":1615,"record_offset":54892,"parent_record_offset":54864,"ancestor_record_offset":53236,"inlinee":25134,"source_line":218,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::quat_to_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":1615,"record_offset":55024,"parent_record_offset":54892,"ancestor_record_offset":53236,"inlinee":25135,"source_line":184,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::impl$20::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1615,"record_offset":55128,"parent_record_offset":55104,"ancestor_record_offset":53236,"inlinee":25137,"source_line":1011,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":1615,"record_offset":54984,"parent_record_offset":54952,"ancestor_record_offset":53236,"inlinee":18181,"source_line":91,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$100::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":1615,"record_offset":55176,"parent_record_offset":55152,"ancestor_record_offset":53236,"inlinee":25139,"source_line":1784,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$103::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":1615,"record_offset":54952,"parent_record_offset":54892,"ancestor_record_offset":53236,"inlinee":25143,"source_line":1811,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$25::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":1615,"record_offset":55324,"parent_record_offset":54892,"ancestor_record_offset":53236,"inlinee":25141,"source_line":1167,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_critters::draw_wren_shader
// 记录形式: compiled_procedure; instances=1
// 源码行: 124
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1615,"record_offset":56052,"rva":22372032,"va":5391081152}
// 编译实例 RVAs: 0x1555ec0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13380175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391081152,"raw_hex":"e8b1d533ff","size":5,"soft_fail":false,"target_va":5377701312,"va":5391081482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140904ae0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12915957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391081152,"raw_hex":"e80beb3aff","size":5,"soft_fail":false,"target_va":5378165472,"va":5391081424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12900770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391081152,"raw_hex":"e85e263bff","size":5,"soft_fail":false,"target_va":5378180624,"va":5391081389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908c50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12899018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391081152,"raw_hex":"e8362d3bff","size":5,"soft_fail":false,"target_va":5378182224,"va":5391081237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: instanced_critters.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20765898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391081152,"raw_hex":"e8cadc3c01","size":5,"soft_fail":false,"target_va":5411847168,"va":5391081265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":1615,"record_offset":56368,"parent_record_offset":56052,"ancestor_record_offset":56052,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/common_includes.rs
//     evidence={"module":1615,"record_offset":56240,"parent_record_offset":56052,"ancestor_record_offset":56052,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/01615.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":1615,"record_offset":56268,"parent_record_offset":56240,"ancestor_record_offset":56052,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/01615.jsonl.gz"}
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
// 完整结构及原始类型记录: instanced_critters.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: instanced_critters.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/instanced_critters.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: instanced_critters.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_critters::draw_instanced_butterfly_shader
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":1615,"record_offset":50380,"parent_record_offset":50172,"ancestor_record_offset":50172,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/01615.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":1615,"record_offset":50408,"parent_record_offset":50380,"ancestor_record_offset":50172,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/01615.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":1615,"record_offset":50508,"parent_record_offset":50172,"ancestor_record_offset":50172,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/01615.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_critters::draw_instanced_firefly_shader
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":1615,"record_offset":51632,"parent_record_offset":50588,"ancestor_record_offset":50588,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/01615.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":1615,"record_offset":51668,"parent_record_offset":51632,"ancestor_record_offset":50588,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/01615.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::uniform","relationship":"pdb_inline_direct","evidence":{"module":1615,"record_offset":51760,"parent_record_offset":50588,"ancestor_record_offset":50588,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/01615.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_critters::draw_instanced_firefly_glow_shader
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":1615,"record_offset":53004,"parent_record_offset":51864,"ancestor_record_offset":51864,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/01615.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":1615,"record_offset":53040,"parent_record_offset":53004,"ancestor_record_offset":51864,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/01615.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::uniform","relationship":"pdb_inline_direct","evidence":{"module":1615,"record_offset":53132,"parent_record_offset":51864,"ancestor_record_offset":51864,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/01615.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_critters::draw_duck_shader
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":1615,"record_offset":54652,"parent_record_offset":53236,"ancestor_record_offset":53236,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/01615.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":1615,"record_offset":54688,"parent_record_offset":54652,"ancestor_record_offset":53236,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/01615.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::uniform","relationship":"pdb_inline_direct","evidence":{"module":1615,"record_offset":54780,"parent_record_offset":53236,"ancestor_record_offset":53236,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/01615.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_critters::draw_wren_shader
// [资源接口动作；资源实例 unknown] {"role":"named_terrain_heightmap_binding_api","symbol":"country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$0::bind_terrain_heightmap","relationship":"pdb_inline_direct","evidence":{"module":1615,"record_offset":56240,"parent_record_offset":56052,"ancestor_record_offset":56052,"inlinee":27475,"source_line":13,"archive":"_pdb/B/modules/01615.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":1615,"record_offset":56268,"parent_record_offset":56240,"ancestor_record_offset":56052,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/01615.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":1615,"record_offset":56368,"parent_record_offset":56052,"ancestor_record_offset":56052,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/01615.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// 父函数直接调用位置 = unknown/当前扫描未找到；PDB 内联及间接调用不能据此排除。
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: instanced_critters.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_critters::draw_duck_shader
// 信息 ID: 53ea855086697807f0c62040
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_critters::draw_instanced_butterfly_shader
// 信息 ID: e0767f0a3c90cf7e9addcaa4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_critters::draw_instanced_firefly_glow_shader
// 信息 ID: a19ee2d702c451facdc13ed6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_critters::draw_instanced_firefly_shader
// 信息 ID: d1459673201a31a1e8b6f35b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::instanced_critters::draw_wren_shader
// 信息 ID: 52400f683d38acb6703a176c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
