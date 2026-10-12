// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/render_loop/compute_passes/stairs.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 449888a8b90008d31bcfb179c451e4b2065d8db7ba14b8129c311fedcac1d1d7
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: stairs.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 517
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_platform; // namespace candidate
//   package identities: [{"name":"bevy_platform","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use equivalent; // namespace candidate
//   package identities: [{"name":"equivalent","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::stairs::stairs
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":517,"record_offset":108544,"rva":10196000,"va":5378905120}
// 编译实例 RVAs: 0x9b9420
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14153b2e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12066206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378905120,"raw_hex":"e89e1db800","size":5,"soft_fail":false,"target_va":5390971616,"va":5378905405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14153be70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12069147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378905120,"raw_hex":"e81b29b800","size":5,"soft_fail":false,"target_va":5390974576,"va":5378905424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14153e5c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12079192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378905120,"raw_hex":"e85850b800","size":5,"soft_fail":false,"target_va":5390984640,"va":5378905443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141548f90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12122197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378905120,"raw_hex":"e855f8b800","size":5,"soft_fail":false,"target_va":5391028112,"va":5378905910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415495d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12123808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378905120,"raw_hex":"e8a0feb800","size":5,"soft_fail":false,"target_va":5391029712,"va":5378905899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141549ad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12125066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378905120,"raw_hex":"e88a03b900","size":5,"soft_fail":false,"target_va":5391030992,"va":5378905921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141770e40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stairs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14383184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378905120,"raw_hex":"e85078db00","size":5,"soft_fail":false,"target_va":5393288768,"va":5378905579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14177b1f0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: stairs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14424810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378905120,"raw_hex":"e8ea1adc00","size":5,"soft_fail":false,"target_va":5393330672,"va":5378905857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141797020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stairs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14539225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378905120,"raw_hex":"e8d9d9dd00","size":5,"soft_fail":false,"target_va":5393444896,"va":5378905666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37715642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378905120,"raw_hex":"e8ba7e3f02","size":5,"soft_fail":false,"target_va":5416621600,"va":5378905953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4630945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378905120,"raw_hex":"e8a1a94600","size":5,"soft_fail":false,"target_va":5383536672,"va":5378905722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4633535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378905120,"raw_hex":"e8bfb34600","size":5,"soft_fail":false,"target_va":5383539152,"va":5378905612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::WithRg::run
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":517,"record_offset":110516,"parent_record_offset":108544,"ancestor_record_offset":108544,"inlinee":12358,"source_line":489,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":517,"record_offset":108700,"parent_record_offset":108544,"ancestor_record_offset":108544,"inlinee":12361,"source_line":41,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":517,"record_offset":110048,"parent_record_offset":110024,"ancestor_record_offset":108544,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":517,"record_offset":110096,"parent_record_offset":110072,"ancestor_record_offset":108544,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":517,"record_offset":110072,"parent_record_offset":110048,"ancestor_record_offset":108544,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":110024,"parent_record_offset":110000,"ancestor_record_offset":108544,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":110000,"parent_record_offset":109960,"ancestor_record_offset":108544,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":109960,"parent_record_offset":109936,"ancestor_record_offset":108544,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":517,"record_offset":108756,"parent_record_offset":108728,"ancestor_record_offset":108544,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":517,"record_offset":110320,"parent_record_offset":110296,"ancestor_record_offset":108544,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":517,"record_offset":110296,"parent_record_offset":110272,"ancestor_record_offset":108544,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":517,"record_offset":110244,"parent_record_offset":110212,"ancestor_record_offset":108544,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":517,"record_offset":110188,"parent_record_offset":109676,"ancestor_record_offset":108544,"inlinee":4185,"source_line":125,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":517,"record_offset":110212,"parent_record_offset":110188,"ancestor_record_offset":108544,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":517,"record_offset":109708,"parent_record_offset":109676,"ancestor_record_offset":108544,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":517,"record_offset":110272,"parent_record_offset":110212,"ancestor_record_offset":108544,"inlinee":4197,"source_line":132,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":517,"record_offset":109756,"parent_record_offset":109732,"ancestor_record_offset":108544,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":517,"record_offset":109732,"parent_record_offset":109708,"ancestor_record_offset":108544,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":517,"record_offset":109936,"parent_record_offset":109912,"ancestor_record_offset":108544,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":517,"record_offset":110388,"parent_record_offset":110364,"ancestor_record_offset":108544,"inlinee":4201,"source_line":2868,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":517,"record_offset":110364,"parent_record_offset":109676,"ancestor_record_offset":108544,"inlinee":4200,"source_line":302,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":517,"record_offset":108728,"parent_record_offset":108700,"ancestor_record_offset":108544,"inlinee":9395,"source_line":515,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":517,"record_offset":109676,"parent_record_offset":108728,"ancestor_record_offset":108544,"inlinee":4195,"source_line":540,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":517,"record_offset":108780,"parent_record_offset":108756,"ancestor_record_offset":108544,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":517,"record_offset":109424,"parent_record_offset":109400,"ancestor_record_offset":108544,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":517,"record_offset":109452,"parent_record_offset":109424,"ancestor_record_offset":108544,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":108996,"parent_record_offset":108972,"ancestor_record_offset":108544,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":109080,"parent_record_offset":109056,"ancestor_record_offset":108544,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":517,"record_offset":109228,"parent_record_offset":109200,"ancestor_record_offset":108544,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":517,"record_offset":109200,"parent_record_offset":109172,"ancestor_record_offset":108544,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":517,"record_offset":109848,"parent_record_offset":109784,"ancestor_record_offset":108544,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":517,"record_offset":109872,"parent_record_offset":109848,"ancestor_record_offset":108544,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":517,"record_offset":109912,"parent_record_offset":109732,"ancestor_record_offset":108544,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":517,"record_offset":110484,"parent_record_offset":108700,"ancestor_record_offset":108544,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":109020,"parent_record_offset":108996,"ancestor_record_offset":108544,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":517,"record_offset":109340,"parent_record_offset":109316,"ancestor_record_offset":108544,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":517,"record_offset":109784,"parent_record_offset":109756,"ancestor_record_offset":108544,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":517,"record_offset":109820,"parent_record_offset":109784,"ancestor_record_offset":108544,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":517,"record_offset":110140,"parent_record_offset":109960,"ancestor_record_offset":108544,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":110412,"parent_record_offset":110388,"ancestor_record_offset":108544,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":110436,"parent_record_offset":110412,"ancestor_record_offset":108544,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$11::fetch
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":517,"record_offset":110576,"parent_record_offset":110552,"ancestor_record_offset":108544,"inlinee":30203,"source_line":568,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$11::unfetch
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":517,"record_offset":110924,"parent_record_offset":110900,"ancestor_record_offset":108544,"inlinee":30215,"source_line":575,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$22::fetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":517,"record_offset":110552,"parent_record_offset":110516,"ancestor_record_offset":108544,"inlinee":30273,"source_line":593,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$22::unfetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":517,"record_offset":110900,"parent_record_offset":110516,"ancestor_record_offset":108544,"inlinee":30280,"source_line":596,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::rocky_terrain::stairs::stairs::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":517,"record_offset":110740,"parent_record_offset":110704,"ancestor_record_offset":108544,"inlinee":30277,"source_line":32463,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::rocky_terrain::stairs::stairs_counter::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":517,"record_offset":110836,"parent_record_offset":110800,"ancestor_record_offset":108544,"inlinee":30279,"source_line":32599,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::compute_passes::stairs::stairs::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/stairs.rs
//     evidence={"module":517,"record_offset":110664,"parent_record_offset":110516,"ancestor_record_offset":108544,"inlinee":30275,"source_line":29,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":517,"record_offset":109400,"parent_record_offset":109376,"ancestor_record_offset":108544,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":109140,"parent_record_offset":109112,"ancestor_record_offset":108544,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":109172,"parent_record_offset":109140,"ancestor_record_offset":108544,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":109112,"parent_record_offset":108892,"ancestor_record_offset":108544,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":108972,"parent_record_offset":108892,"ancestor_record_offset":108544,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":109528,"parent_record_offset":108892,"ancestor_record_offset":108544,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":109056,"parent_record_offset":108892,"ancestor_record_offset":108544,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":517,"record_offset":108816,"parent_record_offset":108780,"ancestor_record_offset":108544,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":517,"record_offset":109376,"parent_record_offset":109272,"ancestor_record_offset":108544,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":109316,"parent_record_offset":109292,"ancestor_record_offset":108544,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":109620,"parent_record_offset":108892,"ancestor_record_offset":108544,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":109292,"parent_record_offset":109272,"ancestor_record_offset":108544,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":108868,"parent_record_offset":108844,"ancestor_record_offset":108544,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":108844,"parent_record_offset":108816,"ancestor_record_offset":108544,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":108892,"parent_record_offset":108868,"ancestor_record_offset":108544,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":108944,"parent_record_offset":108892,"ancestor_record_offset":108544,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":109272,"parent_record_offset":108892,"ancestor_record_offset":108544,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":517,"record_offset":109500,"parent_record_offset":108892,"ancestor_record_offset":108544,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":517,"record_offset":110704,"parent_record_offset":110664,"ancestor_record_offset":108544,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":517,"record_offset":110772,"parent_record_offset":110664,"ancestor_record_offset":108544,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::stairs::stairs::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7643
// 首条位置: {"module":517,"inlinee":30275}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":517,"record_offset":110704,"parent_record_offset":110664,"ancestor_record_offset":110664,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":517,"record_offset":110772,"parent_record_offset":110664,"ancestor_record_offset":110664,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::rocky_terrain::stairs::stairs::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":517,"record_offset":110740,"parent_record_offset":110704,"ancestor_record_offset":110664,"inlinee":30277,"source_line":32463,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::rocky_terrain::stairs::stairs_counter::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":517,"record_offset":110836,"parent_record_offset":110800,"ancestor_record_offset":110664,"inlinee":30279,"source_line":32599,"archive":"_pdb/C/modules/00517.jsonl.gz"}
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
// 完整结构及原始类型记录: stairs.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::stairs::StairsTransformSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer<country_core::systems::render_loop::compute_passes::stairs::StairsTransformSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::stairs::StairsTransformSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer<country_core::systems::render_loop::compute_passes::stairs::StairsTransformSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::stairs::StairsTransformSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::stairs::StairsTransformSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::compute_passes::stairs::StairsTransformSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0<country_core::systems::render_loop::compute_passes::stairs::StairsTransformSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0", "arguments": ["country_core::systems::render_loop::compute_passes::stairs::StairsTransformSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::stairs::StairsTransformSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::stairs::StairsTransformSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::systems::render_loop::compute_passes::stairs::StairsTransformSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::systems::render_loop::compute_passes::stairs::StairsTransformSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::systems::render_loop::compute_passes::stairs::StairsTransformSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::systems::render_loop::compute_passes::stairs::StairsTransformSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::stairs::StairsTransformSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::systems::render_loop::compute_passes::stairs::StairsTransformSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::stairs::StairsTransformSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::compute_passes::stairs::StairsTransformSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: stairs.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/stairs.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: stairs.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::stairs::stairs
// [PDB Shader 类型/入口] {"shader_family":"country_core/rocky_terrain/stairs/stairs","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::rocky_terrain::stairs::stairs::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":110740,"parent_record_offset":110704,"ancestor_record_offset":108544,"inlinee":30277,"source_line":32463,"archive":"_pdb/C/modules/00517.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/rocky_terrain/stairs/stairs_counter","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::rocky_terrain::stairs::stairs_counter::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":110836,"parent_record_offset":110800,"ancestor_record_offset":108544,"inlinee":30279,"source_line":32599,"archive":"_pdb/C/modules/00517.jsonl.gz"}}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::stairs::stairs::closure$0
// [PDB Shader 类型/入口] {"shader_family":"country_core/rocky_terrain/stairs/stairs","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::rocky_terrain::stairs::stairs::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":110740,"parent_record_offset":110704,"ancestor_record_offset":110664,"inlinee":30277,"source_line":32463,"archive":"_pdb/C/modules/00517.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/rocky_terrain/stairs/stairs_counter","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::rocky_terrain::stairs::stairs_counter::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":110836,"parent_record_offset":110800,"ancestor_record_offset":110664,"inlinee":30279,"source_line":32599,"archive":"_pdb/C/modules/00517.jsonl.gz"}}
// [Shader 映射] {"shader_id":"aa1810ecd406020c7cc78d7d","relative_path":"country-core/compiled/rocky_terrain/stairs/stairs.precomp","sha256":"0242160bfa1444f8c52c6a3237ba5ce1b58503e76e214fdb03ecb5434eaafcdf","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::stairs::stairs","hint":{"shader_family":"country_core/rocky_terrain/stairs/stairs","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::rocky_terrain::stairs::stairs::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":110740,"parent_record_offset":110704,"ancestor_record_offset":108544,"inlinee":30277,"source_line":32463,"archive":"_pdb/C/modules/00517.jsonl.gz"}},"shader_uri":"country_core/rocky_terrain/stairs/stairs:main"}
// [Shader 映射] {"shader_id":"fb04e59653a7940924fc966d","relative_path":"country-core/compiled/rocky_terrain/stairs/stairs_counter.precomp","sha256":"baedc0ac751fe442bb8fd297c90da4f3d4726f1a5c4b986b94899cb38b5f99ea","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::stairs::stairs","hint":{"shader_family":"country_core/rocky_terrain/stairs/stairs_counter","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::rocky_terrain::stairs::stairs_counter::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":110836,"parent_record_offset":110800,"ancestor_record_offset":108544,"inlinee":30279,"source_line":32599,"archive":"_pdb/C/modules/00517.jsonl.gz"}},"shader_uri":"country_core/rocky_terrain/stairs/stairs_counter:main"}
// [Shader 映射] {"shader_id":"aa1810ecd406020c7cc78d7d","relative_path":"country-core/compiled/rocky_terrain/stairs/stairs.precomp","sha256":"0242160bfa1444f8c52c6a3237ba5ce1b58503e76e214fdb03ecb5434eaafcdf","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::stairs::stairs::closure$0","hint":{"shader_family":"country_core/rocky_terrain/stairs/stairs","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::rocky_terrain::stairs::stairs::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":110740,"parent_record_offset":110704,"ancestor_record_offset":110664,"inlinee":30277,"source_line":32463,"archive":"_pdb/C/modules/00517.jsonl.gz"}},"shader_uri":"country_core/rocky_terrain/stairs/stairs:main"}
// [Shader 映射] {"shader_id":"fb04e59653a7940924fc966d","relative_path":"country-core/compiled/rocky_terrain/stairs/stairs_counter.precomp","sha256":"baedc0ac751fe442bb8fd297c90da4f3d4726f1a5c4b986b94899cb38b5f99ea","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::stairs::stairs::closure$0","hint":{"shader_family":"country_core/rocky_terrain/stairs/stairs_counter","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::rocky_terrain::stairs::stairs_counter::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":517,"record_offset":110836,"parent_record_offset":110800,"ancestor_record_offset":110664,"inlinee":30279,"source_line":32599,"archive":"_pdb/C/modules/00517.jsonl.gz"}},"shader_uri":"country_core/rocky_terrain/stairs/stairs_counter:main"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/rocky_terrain/stairs/stairs.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[89]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":94,"name":"globalParams","storage_class":"Uniform","type":{"id":93,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,92]},"decorations":[],"pointee":{"id":92,"name":"GlobalParams_natural","record":{"opcode":"OpTypeStruct","operands":[11,20,19,19]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"draw_indirect_buffer_offset","type_id":11,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"contouring_grid_dims","type_id":20,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"pebble_indirect_draw_idx","type_id":19,"decorations":[{"decoration":"Offset","operands":[12]}]},{"index":3,"name":"pebble_buffer_offset","type_id":19,"decorations":[{"decoration":"Offset","operands":[16]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":243,"name":"rocky_terrain.tex","storage_class":"UniformConstant","type":{"id":242,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,240]},"decorations":[],"pointee":{"id":240,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[9,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":9,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[9]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":247,"name":"samplers.lnc_iso","storage_class":"UniformConstant","type":{"id":246,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,244]},"decorations":[],"pointee":{"id":244,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":5,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[5]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":258,"name":"path_mask","storage_class":"Uniform","type":{"id":257,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,256]},"decorations":[],"pointee":{"id":256,"name":"PathExclusionMask_natural","record":{"opcode":"OpTypeStruct","operands":[20]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"tex_dims","type_id":20,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":6,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[6]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":271,"name":"samplers.llb","storage_class":"UniformConstant","type":{"id":246,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,244]},"decorations":[],"pointee":{"id":244,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":4,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[4]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":283,"name":"path_mask.tex","storage_class":"UniformConstant","type":{"id":242,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,240]},"decorations":[],"pointee":{"id":240,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[9,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":6,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[6]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":312,"name":"water_mask.tex","storage_class":"UniformConstant","type":{"id":242,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,240]},"decorations":[],"pointee":{"id":240,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[9,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":10,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":358,"name":"terrain_heightmap","storage_class":"Uniform","type":{"id":357,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,355]},"decorations":[],"pointee":{"id":355,"name":"TerrainHeightmap_natural","record":{"opcode":"OpTypeStruct","operands":[20,356,356,10,23]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"terrain_heightmap_dims","type_id":20,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"bg_terrain_heightmap","type_id":356,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"bg_terrain_fog_height","type_id":356,"decorations":[{"decoration":"Offset","operands":[12]}]},{"index":3,"name":"bg_terrain_height_bounds_ws","type_id":10,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":4,"name":"bg_water_footprint_ws","type_id":23,"decorations":[{"decoration":"Offset","operands":[24]}]}]}},"descriptor_set":17,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[17]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":393,"name":"terrain_heightmap.terrain_heightmap","storage_class":"UniformConstant","type":{"id":242,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,240]},"decorations":[],"pointee":{"id":240,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[9,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":17,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[17]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":1016,"name":"draw_commands","storage_class":"Uniform","type":{"id":1015,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,1013]},"decorations":[],"pointee":{"id":1013,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[1014]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":1014,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":1029,"name":"stairs_buffer","storage_class":"Uniform","type":{"id":1028,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,1026]},"decorations":[],"pointee":{"id":1026,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[1027]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":1027,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":89,"name":"unknown","storage_class":"Input","type":{"id":88,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,86]},"decorations":[],"pointee":{"id":86,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[19,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/rocky_terrain/stairs/stairs_counter.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[1,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":17,"name":"stairs_counter","storage_class":"Uniform","type":{"id":16,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,14]},"decorations":[],"pointee":{"id":14,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[15]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":15,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":22,"name":"globalParams","storage_class":"Uniform","type":{"id":21,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,20]},"decorations":[],"pointee":{"id":20,"name":"GlobalParams_natural","record":{"opcode":"OpTypeStruct","operands":[9]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"draw_indirect_buffer_offset","type_id":9,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":31,"name":"draw_commands","storage_class":"Uniform","type":{"id":30,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,28]},"decorations":[],"pointee":{"id":28,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[29]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":29,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":5678848,"caller_aliases":{"alias_group_rva":5678848,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5679761,"position_among_mapped_render_targets":28,"mapped_render_target_count":74,"previous_callsite":5679750,"next_callsite":5680626,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/roof_tiles_gravity.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/gpu_generate_draw_lists.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: stairs.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::backend::icc::IccProfileChanged>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::backend::icc::IccProfileChanged>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::backend::icc::IccProfileChanged>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::stairs::stairs
// 信息 ID: c8ed3217a8605d7b49fbb33a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56aa91 -> 0x9b9420 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::stairs::stairs::closure$0
// 信息 ID: 7e94fd06b0a1fe5901642f05
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::compute_passes::stairs::stairs", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::stairs::stairs; module=517, inline_record=110664; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
