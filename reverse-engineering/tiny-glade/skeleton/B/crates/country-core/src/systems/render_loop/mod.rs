// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/mod.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 35c4de0e48806875cc0ad7952cc5fbf86fc8bc174511263815b210dc86069679
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 503, 521, 533, 1655
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bytemuck; // namespace candidate
//   package identities: [{"name":"bytemuck","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.9"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::RenderContext::finish
// 记录形式: compiled_procedure; instances=1
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":533,"record_offset":25520,"rva":10345040,"va":5379054160}
// 编译实例 RVAs: 0x9dda50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140606ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4026316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379054160,"raw_hex":"e83490c2ff","size":5,"soft_fail":false,"target_va":5375028128,"va":5379054439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3972828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379054160,"raw_hex":"e82461c3ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379054327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2116063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379054160,"raw_hex":"e8df492000","size":5,"soft_fail":false,"target_va":5381170352,"va":5379054284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32792958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379054160,"raw_hex":"e87e61f401","size":5,"soft_fail":false,"target_va":5411847168,"va":5379054205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":25808,"parent_record_offset":25520,"ancestor_record_offset":25520,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::GpuOwned::mark_unused
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":533,"record_offset":25668,"parent_record_offset":25520,"ancestor_record_offset":25520,"inlinee":28406,"source_line":447,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":26220,"parent_record_offset":26196,"ancestor_record_offset":25520,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":26196,"parent_record_offset":26104,"ancestor_record_offset":25520,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":533,"record_offset":26132,"parent_record_offset":26104,"ancestor_record_offset":25520,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":533,"record_offset":26104,"parent_record_offset":26076,"ancestor_record_offset":25520,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":533,"record_offset":26076,"parent_record_offset":26048,"ancestor_record_offset":25520,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":533,"record_offset":25716,"parent_record_offset":25692,"ancestor_record_offset":25520,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":533,"record_offset":25692,"parent_record_offset":25668,"ancestor_record_offset":25520,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":533,"record_offset":25868,"parent_record_offset":25836,"ancestor_record_offset":25520,"inlinee":6218,"source_line":220,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":533,"record_offset":25740,"parent_record_offset":25716,"ancestor_record_offset":25520,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":533,"record_offset":26164,"parent_record_offset":26132,"ancestor_record_offset":25520,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":25924,"parent_record_offset":25836,"ancestor_record_offset":25520,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":25892,"parent_record_offset":25868,"ancestor_record_offset":25520,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":25764,"parent_record_offset":25740,"ancestor_record_offset":25520,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$25::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":533,"record_offset":25836,"parent_record_offset":25808,"ancestor_record_offset":25520,"inlinee":6217,"source_line":550,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::clear_buffer
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6F40
// 首条位置: {"module":521,"inlinee":28480}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":521,"record_offset":59856,"parent_record_offset":59824,"ancestor_record_offset":59824,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::pass_resource_access::ssbo_to_rg_handle
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/pass_resource_access.rs
//     evidence={"module":521,"record_offset":60632,"parent_record_offset":59824,"ancestor_record_offset":59824,"inlinee":28405,"source_line":203,"archive":"_pdb/B/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":521,"record_offset":60280,"parent_record_offset":60256,"ancestor_record_offset":59824,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":521,"record_offset":60328,"parent_record_offset":60304,"ancestor_record_offset":59824,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":521,"record_offset":60304,"parent_record_offset":60280,"ancestor_record_offset":59824,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":521,"record_offset":60256,"parent_record_offset":60232,"ancestor_record_offset":59824,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":521,"record_offset":60232,"parent_record_offset":60192,"ancestor_record_offset":59824,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":521,"record_offset":60192,"parent_record_offset":60168,"ancestor_record_offset":59824,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":521,"record_offset":60580,"parent_record_offset":60556,"ancestor_record_offset":59824,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":521,"record_offset":60556,"parent_record_offset":60532,"ancestor_record_offset":59824,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":521,"record_offset":60504,"parent_record_offset":60472,"ancestor_record_offset":59824,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":521,"record_offset":60448,"parent_record_offset":59912,"ancestor_record_offset":59824,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":521,"record_offset":60472,"parent_record_offset":60448,"ancestor_record_offset":59824,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":521,"record_offset":59940,"parent_record_offset":59912,"ancestor_record_offset":59824,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":521,"record_offset":60532,"parent_record_offset":60472,"ancestor_record_offset":59824,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":521,"record_offset":59988,"parent_record_offset":59964,"ancestor_record_offset":59824,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":521,"record_offset":59964,"parent_record_offset":59940,"ancestor_record_offset":59824,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":521,"record_offset":60168,"parent_record_offset":60144,"ancestor_record_offset":59824,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":521,"record_offset":59912,"parent_record_offset":59856,"ancestor_record_offset":59824,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":521,"record_offset":60080,"parent_record_offset":60016,"ancestor_record_offset":59824,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":521,"record_offset":60104,"parent_record_offset":60080,"ancestor_record_offset":59824,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":521,"record_offset":60144,"parent_record_offset":59964,"ancestor_record_offset":59824,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":521,"record_offset":59884,"parent_record_offset":59856,"ancestor_record_offset":59824,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":521,"record_offset":60016,"parent_record_offset":59988,"ancestor_record_offset":59824,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":521,"record_offset":60396,"parent_record_offset":60372,"ancestor_record_offset":59824,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":521,"record_offset":60052,"parent_record_offset":60016,"ancestor_record_offset":59824,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":521,"record_offset":60372,"parent_record_offset":60192,"ancestor_record_offset":59824,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00521.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::clear_rg_buffer
// 记录形式: compiled_procedure; instances=1
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":533,"record_offset":26612,"rva":10345392,"va":5379054512}
// 编译实例 RVAs: 0x9ddbb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14003bb80
//     owner=unresolved_namespace; count=1
//     target_aliases=532 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10100906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379054512,"raw_hex":"e856df65ff","size":5,"soft_fail":false,"target_va":5368953728,"va":5379054629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ad810
//     owner=unresolved_namespace; count=1
//     target_aliases=34 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9635270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379054512,"raw_hex":"e83afa6cff","size":5,"soft_fail":false,"target_va":5369419792,"va":5379055057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d9030
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4214339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379054512,"raw_hex":"e8bdb1bfff","size":5,"soft_fail":false,"target_va":5374840880,"va":5379055214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3973251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379054512,"raw_hex":"e87d5fc3ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379054734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087d890
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1443197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379054512,"raw_hex":"e883fae9ff","size":5,"soft_fail":false,"target_va":5377611920,"va":5379055112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409dca90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379054512,"raw_hex":"e84beeffff","size":5,"soft_fail":false,"target_va":5379050128,"va":5379054656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd4b40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2060129","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379054512,"raw_hex":"e8616f1f00","size":5,"soft_fail":false,"target_va":5381114688,"va":5379054554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32485974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379054512,"raw_hex":"e856b2ef01","size":5,"soft_fail":false,"target_va":5411540992,"va":5379055013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4699503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379054512,"raw_hex":"e8914ab8ff","size":5,"soft_fail":false,"target_va":5374355376,"va":5379054874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32486775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379054512,"raw_hex":"e877b5ef01","size":5,"soft_fail":false,"target_va":5411541776,"va":5379054996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32487077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379054512,"raw_hex":"e8a5b6ef01","size":5,"soft_fail":false,"target_va":5411542064,"va":5379054982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::rg::pass_builder::PassBuilder::render
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":533,"record_offset":26868,"parent_record_offset":26612,"ancestor_record_offset":26612,"inlinee":15169,"source_line":66,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::pass_builder::impl$2::write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":533,"record_offset":26812,"parent_record_offset":26612,"ancestor_record_offset":26612,"inlinee":9622,"source_line":168,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::add_pass
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":533,"record_offset":26756,"parent_record_offset":26612,"ancestor_record_offset":26612,"inlinee":9697,"source_line":24,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":27076,"parent_record_offset":27052,"ancestor_record_offset":26612,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":27100,"parent_record_offset":27076,"ancestor_record_offset":26612,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":27020,"parent_record_offset":26992,"ancestor_record_offset":26612,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":27052,"parent_record_offset":27020,"ancestor_record_offset":26612,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":533,"record_offset":26992,"parent_record_offset":26868,"ancestor_record_offset":26612,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":533,"record_offset":27628,"parent_record_offset":27596,"ancestor_record_offset":26612,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":533,"record_offset":27676,"parent_record_offset":27652,"ancestor_record_offset":26612,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":533,"record_offset":27652,"parent_record_offset":27628,"ancestor_record_offset":26612,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":533,"record_offset":27596,"parent_record_offset":27548,"ancestor_record_offset":26612,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":533,"record_offset":27548,"parent_record_offset":27520,"ancestor_record_offset":26612,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":533,"record_offset":26780,"parent_record_offset":26756,"ancestor_record_offset":26612,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":533,"record_offset":27200,"parent_record_offset":27176,"ancestor_record_offset":26612,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":26928,"parent_record_offset":26868,"ancestor_record_offset":26612,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":27292,"parent_record_offset":26868,"ancestor_record_offset":26612,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":27316,"parent_record_offset":27292,"ancestor_record_offset":26612,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":27176,"parent_record_offset":26868,"ancestor_record_offset":26612,"inlinee":8786,"source_line":1784,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":27408,"parent_record_offset":27376,"ancestor_record_offset":26612,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":26956,"parent_record_offset":26868,"ancestor_record_offset":26612,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":27348,"parent_record_offset":26868,"ancestor_record_offset":26612,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":27228,"parent_record_offset":27200,"ancestor_record_offset":26612,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":27128,"parent_record_offset":27100,"ancestor_record_offset":26612,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":27256,"parent_record_offset":27200,"ancestor_record_offset":26612,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_builder::PassBuilder::write_impl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":533,"record_offset":26836,"parent_record_offset":26812,"ancestor_record_offset":26612,"inlinee":9624,"source_line":52,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_builder::impl$0::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":533,"record_offset":27376,"parent_record_offset":27348,"ancestor_record_offset":26612,"inlinee":15161,"source_line":25,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::record_pass
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":533,"record_offset":27520,"parent_record_offset":27376,"ancestor_record_offset":26612,"inlinee":9690,"source_line":100,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::clear_rg_buffer::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 80
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6EDA
// 首条位置: {"module":503,"inlinee":28378}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::rg_buffer_copy_from_slice<u32>
// 记录形式: compiled_procedure; instances=1
// 源码行: 99
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":88024,"rva":9492896,"va":5378202016}
// 编译实例 RVAs: 0x90d9a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14003bb80
//     owner=unresolved_namespace; count=1
//     target_aliases=532 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9248570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378202016,"raw_hex":"e8c6e072ff","size":5,"soft_fail":false,"target_va":5368953728,"va":5378202293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=19 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8585999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378202016,"raw_hex":"e8f1fc7cff","size":5,"soft_fail":false,"target_va":5369616112,"va":5378202106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8044852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378202016,"raw_hex":"e8cc3e85ff","size":5,"soft_fail":false,"target_va":5370158128,"va":5378202975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087d890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-591015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378202016,"raw_hex":"e859fbf6ff","size":5,"soft_fail":false,"target_va":5377611920,"va":5378202930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409dca90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t847801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378202016,"raw_hex":"e8b9ef0c00","size":5,"soft_fail":false,"target_va":5379050128,"va":5378202322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409dd240
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t849516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378202016,"raw_hex":"e86cf60c00","size":5,"soft_fail":false,"target_va":5379052096,"va":5378202575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd4b40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2912486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378202016,"raw_hex":"e8e6702c00","size":5,"soft_fail":false,"target_va":5381114688,"va":5378202197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2967599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378202016,"raw_hex":"e82f482d00","size":5,"soft_fail":false,"target_va":5381170352,"va":5378202748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33644745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378202016,"raw_hex":"e8c9600102","size":5,"soft_fail":false,"target_va":5411847168,"va":5378202418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33339078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378202016,"raw_hex":"e8c6b6fc01","size":5,"soft_fail":false,"target_va":5411541776,"va":5378202693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33339443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378202016,"raw_hex":"e833b8fc01","size":5,"soft_fail":false,"target_va":5411542064,"va":5378202616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::_$LT$impl$u20$$u5b$T$u5d$$GT$::copy_from_slice::len_mismatch_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33340602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378202016,"raw_hex":"e8babcfc01","size":5,"soft_fail":false,"target_va":5411543264,"va":5378202657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t33289726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378202016,"raw_hex":"e8fef5fb01","size":5,"soft_fail":false,"target_va":5411492368,"va":5378202637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t33292119","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378202016,"raw_hex":"e857fffb01","size":5,"soft_fail":false,"target_va":5411494800,"va":5378202676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3621243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378202016,"raw_hex":"e87b413700","size":5,"soft_fail":false,"target_va":5381823600,"va":5378202352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3621548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378202016,"raw_hex":"e8ac423700","size":5,"soft_fail":false,"target_va":5381824016,"va":5378202463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::dynamic_constants::DynamicConstants::allocate
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3099685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378202016,"raw_hex":"e8254c2f00","size":5,"soft_fail":false,"target_va":5381302080,"va":5378202390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":503,"record_offset":88320,"parent_record_offset":88024,"ancestor_record_offset":88024,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] bytemuck::cast_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/lib.rs
//     evidence={"module":503,"record_offset":88208,"parent_record_offset":88024,"ancestor_record_offset":88024,"inlinee":4645,"source_line":354,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":503,"record_offset":89960,"parent_record_offset":88024,"ancestor_record_offset":88024,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":89772,"parent_record_offset":88024,"ancestor_record_offset":88024,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":503,"record_offset":89216,"parent_record_offset":88024,"ancestor_record_offset":88024,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":503,"record_offset":89372,"parent_record_offset":88024,"ancestor_record_offset":88024,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::dynamic_constants::DynamicConstants::push_slice
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":503,"record_offset":89428,"parent_record_offset":88024,"ancestor_record_offset":88024,"inlinee":26614,"source_line":150,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::pass_builder::impl$2::write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":503,"record_offset":89156,"parent_record_offset":88024,"ancestor_record_offset":88024,"inlinee":9622,"source_line":168,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::add_pass
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":503,"record_offset":89096,"parent_record_offset":88024,"ancestor_record_offset":88024,"inlinee":9697,"source_line":24,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":503,"record_offset":88744,"parent_record_offset":88720,"ancestor_record_offset":88024,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":503,"record_offset":88792,"parent_record_offset":88768,"ancestor_record_offset":88024,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":503,"record_offset":88768,"parent_record_offset":88744,"ancestor_record_offset":88024,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":88720,"parent_record_offset":88696,"ancestor_record_offset":88024,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":88696,"parent_record_offset":88656,"ancestor_record_offset":88024,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":88656,"parent_record_offset":88632,"ancestor_record_offset":88024,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":503,"record_offset":89044,"parent_record_offset":89020,"ancestor_record_offset":88024,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":503,"record_offset":89020,"parent_record_offset":88996,"ancestor_record_offset":88024,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":503,"record_offset":88968,"parent_record_offset":88936,"ancestor_record_offset":88024,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":503,"record_offset":88912,"parent_record_offset":88384,"ancestor_record_offset":88024,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":503,"record_offset":88936,"parent_record_offset":88912,"ancestor_record_offset":88024,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":503,"record_offset":88412,"parent_record_offset":88384,"ancestor_record_offset":88024,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":503,"record_offset":88996,"parent_record_offset":88936,"ancestor_record_offset":88024,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":503,"record_offset":88460,"parent_record_offset":88436,"ancestor_record_offset":88024,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":503,"record_offset":88436,"parent_record_offset":88412,"ancestor_record_offset":88024,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":503,"record_offset":88632,"parent_record_offset":88608,"ancestor_record_offset":88024,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":503,"record_offset":88348,"parent_record_offset":88320,"ancestor_record_offset":88024,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":503,"record_offset":88384,"parent_record_offset":88320,"ancestor_record_offset":88024,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::cast_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":503,"record_offset":88232,"parent_record_offset":88208,"ancestor_record_offset":88024,"inlinee":4647,"source_line":253,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::try_cast_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":503,"record_offset":88256,"parent_record_offset":88232,"ancestor_record_offset":88024,"inlinee":4648,"source_line":353,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":503,"record_offset":89396,"parent_record_offset":89372,"ancestor_record_offset":88024,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":503,"record_offset":89124,"parent_record_offset":89096,"ancestor_record_offset":88024,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":503,"record_offset":89508,"parent_record_offset":89472,"ancestor_record_offset":88024,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":503,"record_offset":89616,"parent_record_offset":89588,"ancestor_record_offset":88024,"inlinee":6218,"source_line":220,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":503,"record_offset":88280,"parent_record_offset":88256,"ancestor_record_offset":88024,"inlinee":8266,"source_line":338,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":503,"record_offset":88548,"parent_record_offset":88488,"ancestor_record_offset":88024,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":503,"record_offset":88568,"parent_record_offset":88548,"ancestor_record_offset":88024,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":503,"record_offset":88608,"parent_record_offset":88436,"ancestor_record_offset":88024,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":89540,"parent_record_offset":89428,"ancestor_record_offset":88024,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":89640,"parent_record_offset":89616,"ancestor_record_offset":88024,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":503,"record_offset":89472,"parent_record_offset":89428,"ancestor_record_offset":88024,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":503,"record_offset":88488,"parent_record_offset":88460,"ancestor_record_offset":88024,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":503,"record_offset":88860,"parent_record_offset":88836,"ancestor_record_offset":88024,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":503,"record_offset":88520,"parent_record_offset":88488,"ancestor_record_offset":88024,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":503,"record_offset":88836,"parent_record_offset":88656,"ancestor_record_offset":88024,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":89892,"parent_record_offset":89828,"ancestor_record_offset":88024,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":89280,"parent_record_offset":89244,"ancestor_record_offset":88024,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":89916,"parent_record_offset":89892,"ancestor_record_offset":88024,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":89304,"parent_record_offset":89280,"ancestor_record_offset":88024,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":503,"record_offset":89800,"parent_record_offset":89772,"ancestor_record_offset":88024,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":503,"record_offset":89244,"parent_record_offset":89216,"ancestor_record_offset":88024,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":503,"record_offset":89828,"parent_record_offset":89800,"ancestor_record_offset":88024,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":503,"record_offset":89336,"parent_record_offset":89244,"ancestor_record_offset":88024,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":503,"record_offset":89864,"parent_record_offset":89828,"ancestor_record_offset":88024,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$25::drop
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":503,"record_offset":89588,"parent_record_offset":89564,"ancestor_record_offset":88024,"inlinee":6217,"source_line":550,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_builder::PassBuilder::write_impl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":503,"record_offset":89184,"parent_record_offset":89156,"ancestor_record_offset":88024,"inlinee":9624,"source_line":52,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::rg_buffer_copy_from_slice::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 117
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1257E
// 首条位置: {"module":1655,"inlinee":75134}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1655,"record_offset":43984,"parent_record_offset":43600,"ancestor_record_offset":43600,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::GpuOwned::track_unwrap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1655,"record_offset":43688,"parent_record_offset":43600,"ancestor_record_offset":43600,"inlinee":74648,"source_line":424,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Ref::raw
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":1655,"record_offset":43660,"parent_record_offset":43600,"ancestor_record_offset":43600,"inlinee":28197,"source_line":344,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":1655,"record_offset":43832,"parent_record_offset":43804,"ancestor_record_offset":43600,"inlinee":6218,"source_line":220,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1655,"record_offset":43780,"parent_record_offset":43688,"ancestor_record_offset":43600,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1655,"record_offset":43856,"parent_record_offset":43832,"ancestor_record_offset":43600,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$17::track
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1655,"record_offset":43748,"parent_record_offset":43724,"ancestor_record_offset":43600,"inlinee":8709,"source_line":250,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$22::track
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1655,"record_offset":43724,"parent_record_offset":43688,"ancestor_record_offset":43600,"inlinee":74584,"source_line":397,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$25::drop
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1655,"record_offset":43804,"parent_record_offset":43780,"ancestor_record_offset":43600,"inlinee":6217,"source_line":550,"archive":"_pdb/B/modules/01655.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::clear_rg_color_image
// 记录形式: compiled_procedure; instances=1
// 源码行: 137
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":533,"record_offset":27768,"rva":10346128,"va":5379055248}
// 编译实例 RVAs: 0x9dde90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14003bb80
//     owner=unresolved_namespace; count=1
//     target_aliases=532 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10101642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379055248,"raw_hex":"e876dc65ff","size":5,"soft_fail":false,"target_va":5368953728,"va":5379055365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ad810
//     owner=unresolved_namespace; count=1
//     target_aliases=34 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9636006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379055248,"raw_hex":"e85af76cff","size":5,"soft_fail":false,"target_va":5369419792,"va":5379055793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d9030
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4215075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379055248,"raw_hex":"e8ddaebfff","size":5,"soft_fail":false,"target_va":5374840880,"va":5379055950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3973986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379055248,"raw_hex":"e89e5cc3ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379055469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087d890
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1443933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379055248,"raw_hex":"e8a3f7e9ff","size":5,"soft_fail":false,"target_va":5377611920,"va":5379055848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409dca90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379055248,"raw_hex":"e86cebffff","size":5,"soft_fail":false,"target_va":5379050128,"va":5379055391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd4b40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2059393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379055248,"raw_hex":"e8816c1f00","size":5,"soft_fail":false,"target_va":5381114688,"va":5379055290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32485239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379055248,"raw_hex":"e877afef01","size":5,"soft_fail":false,"target_va":5411540992,"va":5379055748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4700238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379055248,"raw_hex":"e8b247b8ff","size":5,"soft_fail":false,"target_va":5374355376,"va":5379055609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32486040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379055248,"raw_hex":"e898b2ef01","size":5,"soft_fail":false,"target_va":5411541776,"va":5379055731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32486342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379055248,"raw_hex":"e8c6b3ef01","size":5,"soft_fail":false,"target_va":5411542064,"va":5379055717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::rg::pass_builder::PassBuilder::render
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":533,"record_offset":28028,"parent_record_offset":27768,"ancestor_record_offset":27768,"inlinee":15169,"source_line":66,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::pass_builder::impl$2::write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":533,"record_offset":27972,"parent_record_offset":27768,"ancestor_record_offset":27768,"inlinee":9622,"source_line":168,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::add_pass
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":533,"record_offset":27916,"parent_record_offset":27768,"ancestor_record_offset":27768,"inlinee":9697,"source_line":24,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":28236,"parent_record_offset":28212,"ancestor_record_offset":27768,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":28260,"parent_record_offset":28236,"ancestor_record_offset":27768,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":28180,"parent_record_offset":28152,"ancestor_record_offset":27768,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":28212,"parent_record_offset":28180,"ancestor_record_offset":27768,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":533,"record_offset":28152,"parent_record_offset":28028,"ancestor_record_offset":27768,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":533,"record_offset":28788,"parent_record_offset":28756,"ancestor_record_offset":27768,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":533,"record_offset":28836,"parent_record_offset":28812,"ancestor_record_offset":27768,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":533,"record_offset":28812,"parent_record_offset":28788,"ancestor_record_offset":27768,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":533,"record_offset":28756,"parent_record_offset":28708,"ancestor_record_offset":27768,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":533,"record_offset":28708,"parent_record_offset":28680,"ancestor_record_offset":27768,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":533,"record_offset":27940,"parent_record_offset":27916,"ancestor_record_offset":27768,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":533,"record_offset":28360,"parent_record_offset":28336,"ancestor_record_offset":27768,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":28088,"parent_record_offset":28028,"ancestor_record_offset":27768,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":28452,"parent_record_offset":28028,"ancestor_record_offset":27768,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":28476,"parent_record_offset":28452,"ancestor_record_offset":27768,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":28336,"parent_record_offset":28028,"ancestor_record_offset":27768,"inlinee":8786,"source_line":1784,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":28568,"parent_record_offset":28536,"ancestor_record_offset":27768,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":533,"record_offset":28116,"parent_record_offset":28028,"ancestor_record_offset":27768,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":28508,"parent_record_offset":28028,"ancestor_record_offset":27768,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":28388,"parent_record_offset":28360,"ancestor_record_offset":27768,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":28288,"parent_record_offset":28260,"ancestor_record_offset":27768,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":28416,"parent_record_offset":28360,"ancestor_record_offset":27768,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_builder::PassBuilder::write_impl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":533,"record_offset":27996,"parent_record_offset":27972,"ancestor_record_offset":27768,"inlinee":9624,"source_line":52,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_builder::impl$0::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":533,"record_offset":28536,"parent_record_offset":28508,"ancestor_record_offset":27768,"inlinee":15161,"source_line":25,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::record_pass
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":533,"record_offset":28680,"parent_record_offset":28536,"ancestor_record_offset":27768,"inlinee":9690,"source_line":100,"archive":"_pdb/B/modules/00533.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::clear_rg_color_image::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 146
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6EDC
// 首条位置: {"module":503,"inlinee":28380}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::resource_registry::ResourceRegistry::image
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_registry.rs
//     evidence={"module":503,"record_offset":5092,"parent_record_offset":5064,"ancestor_record_offset":5064,"inlinee":28382,"source_line":82,"archive":"_pdb/B/modules/00503.jsonl.gz"}
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
// 完整结构及原始类型记录: mod.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::RenderContext
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::render_loop::RenderContext>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::render_loop::RenderContext"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mod.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/mod.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: mod.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::clear_rg_buffer
// [资源接口动作；资源实例 unknown] {"role":"graph_resource_write_api","symbol":"rhapsody::rg::pass_builder::impl$2::write","relationship":"pdb_inline_direct","evidence":{"module":533,"record_offset":26812,"parent_record_offset":26612,"ancestor_record_offset":26612,"inlinee":9622,"source_line":168,"archive":"_pdb/B/modules/00533.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::rg_buffer_copy_from_slice<u32>
// [资源接口动作；资源实例 unknown] {"role":"graph_resource_write_api","symbol":"rhapsody::rg::pass_builder::impl$2::write","relationship":"pdb_inline_direct","evidence":{"module":503,"record_offset":89156,"parent_record_offset":88024,"ancestor_record_offset":88024,"inlinee":9622,"source_line":168,"archive":"_pdb/B/modules/00503.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::clear_rg_color_image
// [资源接口动作；资源实例 unknown] {"role":"graph_resource_write_api","symbol":"rhapsody::rg::pass_builder::impl$2::write","relationship":"pdb_inline_direct","evidence":{"module":533,"record_offset":27972,"parent_record_offset":27768,"ancestor_record_offset":27768,"inlinee":9622,"source_line":168,"archive":"_pdb/B/modules/00533.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":1449168,"caller_aliases":{"alias_group_rva":1449168,"count":1,"display_examples":["rhapsody::rg::state::RenderGraphState::compile"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":1455607,"position_among_mapped_render_targets":25,"mapped_render_target_count":25,"previous_callsite":1455431,"next_callsite":null,"previous_target_files":["crates/tiny-glade/src/render_loop.rs"],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":1460144,"caller_aliases":{"alias_group_rva":1460144,"count":1,"display_examples":["rhapsody::rg::state::RenderGraphState::compile"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":1463276,"position_among_mapped_render_targets":4,"mapped_render_target_count":4,"previous_callsite":1463150,"next_callsite":null,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/ui_pass.rs"],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":1467664,"caller_aliases":{"alias_group_rva":1467664,"count":1,"display_examples":["rhapsody::rg::state::RenderGraphState::compile"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":1469310,"position_among_mapped_render_targets":2,"mapped_render_target_count":2,"previous_callsite":1469191,"next_callsite":null,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/final_post.rs"],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":9993680,"caller_aliases":{"alias_group_rva":9993680,"count":1,"display_examples":["country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":9995128,"position_among_mapped_render_targets":1,"mapped_render_target_count":5,"previous_callsite":null,"next_callsite":9995237,"previous_target_files":[],"next_target_files":["crates/country-core/src/systems/render_loop/mod.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":9993680,"caller_aliases":{"alias_group_rva":9993680,"count":1,"display_examples":["country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":9995237,"position_among_mapped_render_targets":2,"mapped_render_target_count":5,"previous_callsite":9995128,"next_callsite":9995699,"previous_target_files":["crates/country-core/src/systems/render_loop/mod.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/gpu_generate_draw_lists.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":10005232,"caller_aliases":{"alias_group_rva":10005232,"count":1,"display_examples":["country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":10005425,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":10007200,"caller_aliases":{"alias_group_rva":10007200,"count":1,"display_examples":["country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists_pass2"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":10008638,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":10221568,"caller_aliases":{"alias_group_rva":10221568,"count":1,"display_examples":["country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":10221754,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":10225872,"caller_aliases":{"alias_group_rva":10225872,"count":1,"display_examples":["country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":10226683,"position_among_mapped_render_targets":1,"mapped_render_target_count":3,"previous_callsite":null,"next_callsite":10226713,"previous_target_files":[],"next_target_files":["crates/country-core/src/systems/render_loop/mod.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":10225872,"caller_aliases":{"alias_group_rva":10225872,"count":1,"display_examples":["country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":10226713,"position_among_mapped_render_targets":2,"mapped_render_target_count":3,"previous_callsite":10226683,"next_callsite":10229243,"previous_target_files":["crates/country-core/src/systems/render_loop/mod.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/mod.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":10225872,"caller_aliases":{"alias_group_rva":10225872,"count":1,"display_examples":["country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":10229243,"position_among_mapped_render_targets":3,"mapped_render_target_count":3,"previous_callsite":10226713,"next_callsite":null,"previous_target_files":["crates/country-core/src/systems/render_loop/mod.rs"],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":10385264,"caller_aliases":{"alias_group_rva":10385264,"count":1,"display_examples":["country_core::systems::render_loop::compute_passes::bin_lights::bin_lights"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":10385658,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":10879616,"caller_aliases":{"alias_group_rva":10879616,"count":1,"display_examples":["country_core::systems::render_loop::compute_passes::grass::grass_compute"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":10884019,"position_among_mapped_render_targets":2,"mapped_render_target_count":2,"previous_callsite":10883288,"next_callsite":null,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/prefix_scan.rs"],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mod.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 6; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>,bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorMoved> >,bevy_ecs::event::writer::EventWriter<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>", "bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorMoved> >", "bevy_ecs::event::writer::EventWriter<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorMoved> >
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::mouse::MouseButton> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>,bevy_ecs::change_detection::ResMut<country_core::input::ActiveInputModeTracker>,bevy_ecs::event::reader::EventReader<bevy_input::mouse::MouseWheel>,bevy_ecs::system::system_param::Local<country_core::systems::backend::input::translate_input::MouseMoved>,bevy_ecs::event::reader::EventReader<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>,bevy_ecs::event::reader::EventReader<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >", "bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::mouse::MouseButton> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>", "bevy_ecs::change_detection::ResMut<country_core::input::ActiveInputModeTracker>", "bevy_ecs::event::reader::EventReader<bevy_input::mouse::MouseWheel>", "bevy_ecs::system::system_param::Local<country_core::systems::backend::input::translate_input::MouseMoved>", "bevy_ecs::event::reader::EventReader<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>", "bevy_ecs::event::reader::EventReader<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>", "bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::mouse::MouseButton> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::ActiveInputModeTracker>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<bevy_input::mouse::MouseWheel>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::backend::input::translate_input::MouseMoved>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustClosed>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::ResMut<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>", "bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>", "bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustClosed>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::window::WindowSize>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustClosed>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::NextSession>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::ResMut<country_core::systems::mod_key::Modifiers>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::system::system_param::Local<country_core::systems::mod_key::ToggleSnapping>,bevy_ecs::event::reader::EventReader<country_core::resources::walls::decorator::DecoratorOpEvent>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::ResMut<country_core::systems::mod_key::Modifiers>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::system::system_param::Local<country_core::systems::mod_key::ToggleSnapping>", "bevy_ecs::event::reader::EventReader<country_core::resources::walls::decorator::DecoratorOpEvent>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::mod_key::Modifiers>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::mod_key::ToggleSnapping>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::walls::decorator::DecoratorOpEvent>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>,bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::event::writer::EventWriter<country_core::resources::window::CursorMovedScreenSpace>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>", "bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::event::writer::EventWriter<country_core::resources::window::CursorMovedScreenSpace>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::window::CursorMovedScreenSpace>

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::RenderContext::finish
// 信息 ID: 3fe86408d89ca46330ad7313
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x1635f7 -> 0x9dda50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0> @ 0x1653ec -> 0x9dda50 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<tuple$<>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::execute_presentation_render_graph::closure_env$0> @ 0x166b7e -> 0x9dda50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::clear_buffer
// 信息 ID: f0fc4032beae46354bd73569
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists; module=521, inline_record=59824; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists; module=521, inline_record=60664; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::clear_rg_buffer
// 信息 ID: d0ce1571a449c15f40553ac2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists @ 0x988378 -> 0x9ddbb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists @ 0x9883e5 -> 0x9ddbb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists @ 0x98abb1 -> 0x9ddbb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists_pass2 @ 0x98b83e -> 0x9ddbb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass @ 0x9bf8ba -> 0x9ddbb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::bin_lights::bin_lights @ 0x9e78fa -> 0x9ddbb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::grass::grass_compute @ 0xa613b3 -> 0x9ddbb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::clear_rg_buffer::closure$0
// 信息 ID: b2045fec5a668ac27d438da9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::clear_rg_buffer", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::clear_rg_buffer::closure_env$0 <- rhapsody::rg::recorder::impl$1::render<country_core::systems::render_loop::clear_rg_buffer::closure_env$0,country_core::render::pass_builder::pass_resources::PassResources>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] rhapsody::rg::recorder::impl$1::render<country_core::systems::render_loop::clear_rg_buffer::closure_env$0,country_core::render::pass_builder::pass_resources::PassResources>; module=503, inline_record=4560; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::clear_rg_color_image
// 信息 ID: 86f8ff39a4452691e22afc32
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass @ 0x9c15fb -> 0x9dde90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::clear_rg_color_image::closure$0
// 信息 ID: 078885cdc8a5eea33e43c57e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::clear_rg_color_image", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::clear_rg_color_image::closure_env$0 <- rhapsody::rg::recorder::impl$1::render<country_core::systems::render_loop::clear_rg_color_image::closure_env$0,country_core::render::pass_builder::pass_resources::PassResources>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] rhapsody::rg::recorder::impl$1::render<country_core::systems::render_loop::clear_rg_color_image::closure_env$0,country_core::render::pass_builder::pass_resources::PassResources>; module=503, inline_record=5064; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::rg_buffer_copy_from_slice::closure$0
// 信息 ID: 3da30ed5934c3426fbd00d98
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::rg_buffer_copy_from_slice", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0 <- rhapsody::rg::recorder::impl$1::render<country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32>,country_core::render::pass_builder::pass_resources::PassResources>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0 <- core::ptr::drop_in_place<country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0 <- rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0 <- alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32>,alloc::alloc::Global>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32>,alloc::alloc::Global>; module=1655, inline_record=43600; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::rg_buffer_copy_from_slice<u32>
// 信息 ID: bdab84203e63c1156e2f93fc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::rg_buffer_copy_from_slice -> ["u32"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass @ 0x9c0bfb -> 0x90d9a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass @ 0x9c0c19 -> 0x90d9a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
