// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: ccf02f45a2a709470af0ae97594290ef6c56217924050efc52d935d31f2eaf18
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: common_draw_params.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 475, 487, 498, 503, 528, 529, 530, 532
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
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::common_draw_params::CommonDrawParams::new
// 记录形式: inline_instance, inlinee_declaration; instances=26
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x657D
// 首条位置: {"module":475,"inlinee":25981}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::time::Duration::as_secs_f32
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":475,"record_offset":20764,"parent_record_offset":19880,"ancestor_record_offset":19880,"inlinee":11237,"source_line":853,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=23
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":475,"record_offset":19912,"parent_record_offset":19880,"ancestor_record_offset":19880,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":475,"record_offset":20380,"parent_record_offset":20356,"ancestor_record_offset":19880,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":475,"record_offset":20428,"parent_record_offset":20404,"ancestor_record_offset":19880,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":475,"record_offset":20404,"parent_record_offset":20380,"ancestor_record_offset":19880,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":475,"record_offset":20356,"parent_record_offset":20332,"ancestor_record_offset":19880,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":475,"record_offset":20332,"parent_record_offset":20292,"ancestor_record_offset":19880,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":475,"record_offset":20292,"parent_record_offset":20268,"ancestor_record_offset":19880,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":475,"record_offset":20680,"parent_record_offset":20656,"ancestor_record_offset":19880,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":475,"record_offset":20656,"parent_record_offset":20632,"ancestor_record_offset":19880,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=23
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":475,"record_offset":20604,"parent_record_offset":20572,"ancestor_record_offset":19880,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=23
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":475,"record_offset":20548,"parent_record_offset":20012,"ancestor_record_offset":19880,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=23
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":475,"record_offset":20572,"parent_record_offset":20548,"ancestor_record_offset":19880,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=23
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":475,"record_offset":20040,"parent_record_offset":20012,"ancestor_record_offset":19880,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":475,"record_offset":20632,"parent_record_offset":20572,"ancestor_record_offset":19880,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=23
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":475,"record_offset":20088,"parent_record_offset":20064,"ancestor_record_offset":19880,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=23
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":475,"record_offset":20064,"parent_record_offset":20040,"ancestor_record_offset":19880,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=23
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":475,"record_offset":20268,"parent_record_offset":20244,"ancestor_record_offset":19880,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":475,"record_offset":19972,"parent_record_offset":19944,"ancestor_record_offset":19880,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=23
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":475,"record_offset":19944,"parent_record_offset":19912,"ancestor_record_offset":19880,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=23
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":475,"record_offset":20012,"parent_record_offset":19944,"ancestor_record_offset":19880,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":475,"record_offset":20180,"parent_record_offset":20116,"ancestor_record_offset":19880,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":475,"record_offset":20204,"parent_record_offset":20180,"ancestor_record_offset":19880,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":475,"record_offset":20244,"parent_record_offset":20064,"ancestor_record_offset":19880,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":475,"record_offset":20732,"parent_record_offset":19912,"ancestor_record_offset":19880,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":475,"record_offset":22124,"parent_record_offset":22096,"ancestor_record_offset":22036,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":475,"record_offset":20116,"parent_record_offset":20088,"ancestor_record_offset":19880,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":475,"record_offset":20496,"parent_record_offset":20472,"ancestor_record_offset":19880,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":475,"record_offset":20152,"parent_record_offset":20116,"ancestor_record_offset":19880,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":475,"record_offset":20472,"parent_record_offset":20292,"ancestor_record_offset":19880,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_builder::DrawBuilder::bind_common
// 记录形式: compiled_procedure; instances=1
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":73988,"rva":9483360,"va":5378192480}
// 编译实例 RVAs: 0x90b460
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-492260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378192480,"raw_hex":"e81c7df8ff","size":5,"soft_fail":false,"target_va":5377701312,"va":5378193567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908c50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378192480,"raw_hex":"e8dbd6ffff","size":5,"soft_fail":false,"target_va":5378182224,"va":5378192752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t392662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378192480,"raw_hex":"e8d6fd0500","size":5,"soft_fail":false,"target_va":5378585968,"va":5378193301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b850
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t394026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378192480,"raw_hex":"e82a030600","size":5,"soft_fail":false,"target_va":5378586704,"va":5378192673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416286f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13750007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378192480,"raw_hex":"e8f7ced100","size":5,"soft_fail":false,"target_va":5391943408,"va":5378193396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141629a20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13754954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378192480,"raw_hex":"e84ae2d100","size":5,"soft_fail":false,"target_va":5391948320,"va":5378193361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141629b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13755428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378192480,"raw_hex":"e824e4d100","size":5,"soft_fail":false,"target_va":5391948688,"va":5378193255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141629d20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13755698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378192480,"raw_hex":"e832e5d100","size":5,"soft_fail":false,"target_va":5391949088,"va":5378193385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33654536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378192480,"raw_hex":"e808870102","size":5,"soft_fail":false,"target_va":5411847168,"va":5378192627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::bind_resources::impl$0::bind_shadowmap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/bind_resources.rs
//     evidence={"module":503,"record_offset":74316,"parent_record_offset":73988,"ancestor_record_offset":73988,"inlinee":27050,"source_line":50,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_dummy_shadowmap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":503,"record_offset":74160,"parent_record_offset":73988,"ancestor_record_offset":73988,"inlinee":28412,"source_line":270,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":503,"record_offset":75208,"parent_record_offset":73988,"ancestor_record_offset":73988,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_matrix
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":503,"record_offset":74476,"parent_record_offset":73988,"ancestor_record_offset":73988,"inlinee":25133,"source_line":24,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":503,"record_offset":74632,"parent_record_offset":74552,"ancestor_record_offset":73988,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":503,"record_offset":74904,"parent_record_offset":74852,"ancestor_record_offset":73988,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":503,"record_offset":74876,"parent_record_offset":74852,"ancestor_record_offset":73988,"inlinee":24651,"source_line":910,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":503,"record_offset":74236,"parent_record_offset":74192,"ancestor_record_offset":73988,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":503,"record_offset":74268,"parent_record_offset":74160,"ancestor_record_offset":73988,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":503,"record_offset":74192,"parent_record_offset":74160,"ancestor_record_offset":73988,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":503,"record_offset":75168,"parent_record_offset":74500,"ancestor_record_offset":73988,"inlinee":25144,"source_line":106,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":503,"record_offset":74500,"parent_record_offset":74476,"ancestor_record_offset":73988,"inlinee":25134,"source_line":218,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::quat_to_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":503,"record_offset":74552,"parent_record_offset":74500,"ancestor_record_offset":73988,"inlinee":25135,"source_line":184,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::impl$20::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":503,"record_offset":74656,"parent_record_offset":74632,"ancestor_record_offset":73988,"inlinee":25137,"source_line":1011,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":503,"record_offset":74744,"parent_record_offset":74552,"ancestor_record_offset":73988,"inlinee":18181,"source_line":91,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$100::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":503,"record_offset":74704,"parent_record_offset":74680,"ancestor_record_offset":73988,"inlinee":25139,"source_line":1784,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$103::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":503,"record_offset":75104,"parent_record_offset":74500,"ancestor_record_offset":73988,"inlinee":25143,"source_line":1811,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$25::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":503,"record_offset":74852,"parent_record_offset":74500,"ancestor_record_offset":73988,"inlinee":25141,"source_line":1167,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_builder::DrawBuilder::bind_wall_color_customization
// 记录形式: compiled_procedure; instances=1
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":75288,"rva":9485168,"va":5378194288}
// 编译实例 RVAs: 0x90bb70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-493620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378194288,"raw_hex":"e8cc77f8ff","size":5,"soft_fail":false,"target_va":5377701312,"va":5378194927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908610
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378194288,"raw_hex":"e820caffff","size":5,"soft_fail":false,"target_va":5378180624,"va":5378194411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162c290
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13763832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378194288,"raw_hex":"e8f804d200","size":5,"soft_fail":false,"target_va":5391958672,"va":5378194835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=15
//     target_aliases=1 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33652819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378194288,"raw_hex":"e853800102","size":5,"soft_fail":false,"target_va":5411847168,"va":5378194344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":503,"record_offset":75464,"parent_record_offset":75288,"ancestor_record_offset":75288,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_builder::DrawBuilder::bind_roof_color_customization
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x69B2
// 首条位置: {"module":487,"inlinee":27058}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":487,"record_offset":40140,"parent_record_offset":40112,"ancestor_record_offset":40112,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_builder::DrawBuilder::bind_window_color_customization_textures
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 89
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6F38
// 首条位置: {"module":528,"inlinee":28472}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":528,"record_offset":24324,"parent_record_offset":24296,"ancestor_record_offset":24296,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00528.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_builder::DrawBuilder::bind_brick_erase_highlight
// 记录形式: compiled_procedure; instances=1
// 源码行: 93
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":75552,"rva":9485840,"va":5378194960}
// 编译实例 RVAs: 0x90be10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=4
//     target_aliases=19 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8578939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378194960,"raw_hex":"e885187dff","size":5,"soft_fail":false,"target_va":5369616112,"va":5378195046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-494638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378194960,"raw_hex":"e8d273f8ff","size":5,"soft_fail":false,"target_va":5377701312,"va":5378195945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b660
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t390504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378194960,"raw_hex":"e868f50500","size":5,"soft_fail":false,"target_va":5378586208,"va":5378195699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t391092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378194960,"raw_hex":"e8b4f70500","size":5,"soft_fail":false,"target_va":5378586704,"va":5378195607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096ba30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t391412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378194960,"raw_hex":"e8f4f80500","size":5,"soft_fail":false,"target_va":5378587184,"va":5378195767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33651608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378194960,"raw_hex":"e8987b0102","size":5,"soft_fail":false,"target_va":5411847168,"va":5378195555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t33345940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378194960,"raw_hex":"e894d1fc01","size":5,"soft_fail":false,"target_va":5411541776,"va":5378195831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":503,"record_offset":79144,"parent_record_offset":75552,"ancestor_record_offset":75552,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/cursor_terrain_raycast.rs
//     evidence={"module":503,"record_offset":77408,"parent_record_offset":75552,"ancestor_record_offset":75552,"inlinee":26671,"source_line":35,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":503,"record_offset":75736,"parent_record_offset":75552,"ancestor_record_offset":75552,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":503,"record_offset":76188,"parent_record_offset":76164,"ancestor_record_offset":75552,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":503,"record_offset":76236,"parent_record_offset":76212,"ancestor_record_offset":75552,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":503,"record_offset":76212,"parent_record_offset":76188,"ancestor_record_offset":75552,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":76164,"parent_record_offset":76140,"ancestor_record_offset":75552,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":76140,"parent_record_offset":76100,"ancestor_record_offset":75552,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":76100,"parent_record_offset":76076,"ancestor_record_offset":75552,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":503,"record_offset":76488,"parent_record_offset":76464,"ancestor_record_offset":75552,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":503,"record_offset":76464,"parent_record_offset":76440,"ancestor_record_offset":75552,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":503,"record_offset":76412,"parent_record_offset":76380,"ancestor_record_offset":75552,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":503,"record_offset":76356,"parent_record_offset":75828,"ancestor_record_offset":75552,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":503,"record_offset":76380,"parent_record_offset":76356,"ancestor_record_offset":75552,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":503,"record_offset":75856,"parent_record_offset":75828,"ancestor_record_offset":75552,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":503,"record_offset":76440,"parent_record_offset":76380,"ancestor_record_offset":75552,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":503,"record_offset":75904,"parent_record_offset":75880,"ancestor_record_offset":75552,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":503,"record_offset":75880,"parent_record_offset":75856,"ancestor_record_offset":75552,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":503,"record_offset":76076,"parent_record_offset":76052,"ancestor_record_offset":75552,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":503,"record_offset":75792,"parent_record_offset":75764,"ancestor_record_offset":75552,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":503,"record_offset":75764,"parent_record_offset":75736,"ancestor_record_offset":75552,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":503,"record_offset":75828,"parent_record_offset":75764,"ancestor_record_offset":75552,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":503,"record_offset":75992,"parent_record_offset":75932,"ancestor_record_offset":75552,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":503,"record_offset":79188,"parent_record_offset":79144,"ancestor_record_offset":75552,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":503,"record_offset":76012,"parent_record_offset":75992,"ancestor_record_offset":75552,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":503,"record_offset":76052,"parent_record_offset":75880,"ancestor_record_offset":75552,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":503,"record_offset":76540,"parent_record_offset":75736,"ancestor_record_offset":75552,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":503,"record_offset":77432,"parent_record_offset":77408,"ancestor_record_offset":75552,"inlinee":4666,"source_line":998,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":503,"record_offset":76628,"parent_record_offset":76600,"ancestor_record_offset":75552,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":503,"record_offset":75932,"parent_record_offset":75904,"ancestor_record_offset":75552,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":503,"record_offset":76304,"parent_record_offset":76280,"ancestor_record_offset":75552,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":503,"record_offset":75964,"parent_record_offset":75932,"ancestor_record_offset":75552,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":503,"record_offset":76280,"parent_record_offset":76100,"ancestor_record_offset":75552,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_builder::DrawBuilder::bind_stone_floor_textures
// 记录形式: compiled_procedure; instances=1
// 源码行: 116
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":79368,"rva":9487104,"va":5378196224}
// 编译实例 RVAs: 0x90c300
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=12
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-496032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378196224,"raw_hex":"e8606ef8ff","size":5,"soft_fail":false,"target_va":5377701312,"va":5378197339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908610
//     owner=unresolved_namespace; count=12
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378196224,"raw_hex":"e8abc2ffff","size":5,"soft_fail":false,"target_va":5378180624,"va":5378196320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=12
//     target_aliases=1 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33650815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378196224,"raw_hex":"e87f780102","size":5,"soft_fail":false,"target_va":5411847168,"va":5378196348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":503,"record_offset":79540,"parent_record_offset":79368,"ancestor_record_offset":79368,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_builder::DrawBuilder::bind_wooden_floor_textures
// 记录形式: compiled_procedure; instances=1
// 源码行: 167
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":80120,"rva":9488768,"va":5378197888}
// 编译实例 RVAs: 0x90c980
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=12
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-497696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378197888,"raw_hex":"e8e067f8ff","size":5,"soft_fail":false,"target_va":5377701312,"va":5378199003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908610
//     owner=unresolved_namespace; count=12
//     target_aliases=2 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378197888,"raw_hex":"e82bbcffff","size":5,"soft_fail":false,"target_va":5378180624,"va":5378197984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=12
//     target_aliases=1 candidates; complete list: common_draw_params.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33649151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378197888,"raw_hex":"e8ff710102","size":5,"soft_fail":false,"target_va":5411847168,"va":5378198012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":503,"record_offset":80296,"parent_record_offset":80120,"ancestor_record_offset":80120,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00503.jsonl.gz"}
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
// 完整结构及原始类型记录: common_draw_params.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::draw_passes::common_draw_params::CommonDrawParams
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: common_draw_params.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: common_draw_params.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_common
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::uniform","relationship":"pdb_inline_transitive","evidence":{"module":503,"record_offset":74192,"parent_record_offset":74160,"ancestor_record_offset":73988,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00503.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":503,"record_offset":74268,"parent_record_offset":74160,"ancestor_record_offset":73988,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00503.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"named_shadowmap_binding_api","symbol":"country_core::render::pass_builder::bind_resources::impl$0::bind_shadowmap","relationship":"pdb_inline_direct","evidence":{"module":503,"record_offset":74316,"parent_record_offset":73988,"ancestor_record_offset":73988,"inlinee":27050,"source_line":50,"archive":"_pdb/B/modules/00503.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::uniform","relationship":"pdb_inline_direct","evidence":{"module":503,"record_offset":75208,"parent_record_offset":73988,"ancestor_record_offset":73988,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00503.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_wall_color_customization
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":503,"record_offset":75464,"parent_record_offset":75288,"ancestor_record_offset":75288,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00503.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_roof_color_customization
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":487,"record_offset":40140,"parent_record_offset":40112,"ancestor_record_offset":40112,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00487.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_window_color_customization_textures
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":528,"record_offset":24324,"parent_record_offset":24296,"ancestor_record_offset":24296,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00528.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_brick_erase_highlight
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::uniform","relationship":"pdb_inline_direct","evidence":{"module":503,"record_offset":79144,"parent_record_offset":75552,"ancestor_record_offset":75552,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00503.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_stone_floor_textures
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":503,"record_offset":79540,"parent_record_offset":79368,"ancestor_record_offset":79368,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00503.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_wooden_floor_textures
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":503,"record_offset":80296,"parent_record_offset":80120,"ancestor_record_offset":80120,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00503.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// 父函数直接调用位置 = unknown/当前扫描未找到；PDB 内联及间接调用不能据此排除。
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: common_draw_params.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 7; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>
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
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::writer::EventWriter<country_core::systems::backend::grow_grass::GrowGrass>,bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::event::writer::EventWriter<country_core::systems::backend::grow_grass::GrowGrass>", "bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::backend::grow_grass::GrowGrass>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [文件级接口候选，具体函数归属 unknown] core::char::EscapeDebug (*)(char)
// 候选参数类型: ["char"]

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_builder::DrawBuilder::bind_brick_erase_highlight
// 信息 ID: 0d8431e4226edcc08c993838
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks<1> @ 0x8a5044 -> 0x90be10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks<0> @ 0x8a5753 -> 0x90be10 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$13,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> > @ 0x9b6799 -> 0x90be10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_builder::DrawBuilder::bind_common
// 信息 ID: fbf825d8b7e695676e1249e5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 24 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x834139 -> 0x90b460 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x834499 -> 0x90b460 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x834810 -> 0x90b460 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x834bc7 -> 0x90b460 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x834f27 -> 0x90b460 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x83528a -> 0x90b460 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x835fb9 -> 0x90b460 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x836319 -> 0x90b460 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x836679 -> 0x90b460 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x8369b9 -> 0x90b460 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x836d0a -> 0x90b460 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass @ 0x8376b6 -> 0x90b460 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass @ 0x837a14 -> 0x90b460 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::ui_pass::ui_pass @ 0x837edd -> 0x90b460 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::ui_pass::loading_screen_pass @ 0x83828d -> 0x90b460 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::read_terrain_data::read_terrain_data @ 0x98cbc7 -> 0x90b460 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_builder::DrawBuilder::bind_roof_color_customization
// 信息 ID: ebb2f7788d00ac1ce6f33ef1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks<1>; module=487, inline_record=40112; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks<0>; module=487, inline_record=43100; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$16,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=528, inline_record=32524; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$17,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=528, inline_record=38092; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$13,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=528, inline_record=52972; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$12,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=528, inline_record=73112; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_shader_nani; module=532, inline_record=55836; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_builder::DrawBuilder::bind_stone_floor_textures
// 信息 ID: a2167b1a08c29d051c6b5413
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$16,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> > @ 0x9b352e -> 0x90c300 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$17,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> > @ 0x9b441c -> 0x90c300 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_shaders::floor_shader::draw_floor_shader @ 0x9cae9c -> 0x90c300 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_builder::DrawBuilder::bind_wall_color_customization
// 信息 ID: ad8ca0a65b6c7da9d92764e7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks<1> @ 0x8a505b -> 0x90bb70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks<0> @ 0x8a576a -> 0x90bb70 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$16,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> > @ 0x9b3412 -> 0x90bb70 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$17,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> > @ 0x9b43a0 -> 0x90bb70 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$13,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> > @ 0x9b6717 -> 0x90bb70 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$34,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> > @ 0x9b78b3 -> 0x90bb70 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$12,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> > @ 0x9b9725 -> 0x90bb70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_shaders::floor_shader::draw_floor_shader @ 0x9caea7 -> 0x90bb70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_builder::DrawBuilder::bind_window_color_customization_textures
// 信息 ID: 61089bfb4c443d07041e3d90
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$32,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=528, inline_record=24296; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$14,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=528, inline_record=47424; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$31,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=528, inline_record=74280; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_builder::DrawBuilder::bind_wooden_floor_textures
// 信息 ID: e299210e2d1c5abd256f1a24
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$16,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> > @ 0x9b3548 -> 0x90c980 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$17,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> > @ 0x9b4427 -> 0x90c980 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::common_draw_params::CommonDrawParams::new
// 信息 ID: 7247bc04f6e51cab2ad7b7e1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 23 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=19880; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=22036; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=24176; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=27180; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=29320; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=31460; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=475, inline_record=37068; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=475, inline_record=39224; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
