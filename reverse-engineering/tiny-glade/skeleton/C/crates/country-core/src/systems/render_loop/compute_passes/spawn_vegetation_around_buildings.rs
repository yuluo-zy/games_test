// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/render_loop/compute_passes/spawn_vegetation_around_buildings.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 453f48cfa5631945eec08c9b2d59aee93135484c36be74d4210e41a05f13f75a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: spawn_vegetation_around_buildings.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 567
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
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::spawn_vegetation_around_buildings::spawn_vegetation_around_buildings
// 记录形式: compiled_procedure; instances=1
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":567,"record_offset":2524,"rva":11783360,"va":5380492480}
// 编译实例 RVAs: 0xb3ccc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14153e5c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spawn_vegetation_around_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10491612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380492480,"raw_hex":"e8dc16a000","size":5,"soft_fail":false,"target_va":5390984640,"va":5380493023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14153ecb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spawn_vegetation_around_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10493426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380492480,"raw_hex":"e8f21da000","size":5,"soft_fail":false,"target_va":5390986416,"va":5380492985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14153f3a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spawn_vegetation_around_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10495183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380492480,"raw_hex":"e8cf24a000","size":5,"soft_fail":false,"target_va":5390988192,"va":5380493004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141549ad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spawn_vegetation_around_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10537456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380492480,"raw_hex":"e8f0c9a000","size":5,"soft_fail":false,"target_va":5391030992,"va":5380493531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14154a610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spawn_vegetation_around_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10540347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380492480,"raw_hex":"e83bd5a000","size":5,"soft_fail":false,"target_va":5391033872,"va":5380493520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14154bb50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spawn_vegetation_around_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10545798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380492480,"raw_hex":"e886eaa000","size":5,"soft_fail":false,"target_va":5391039312,"va":5380493509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141770e40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: spawn_vegetation_around_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12795602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380492480,"raw_hex":"e8d23ec300","size":5,"soft_fail":false,"target_va":5393288768,"va":5380493161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141779660
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: spawn_vegetation_around_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12830159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380492480,"raw_hex":"e8cfc5c300","size":5,"soft_fail":false,"target_va":5393323616,"va":5380493452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417835d0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: spawn_vegetation_around_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12871171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380492480,"raw_hex":"e80366c400","size":5,"soft_fail":false,"target_va":5393364432,"va":5380493256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36128694","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380492480,"raw_hex":"e8b6472702","size":5,"soft_fail":false,"target_va":5416621600,"va":5380492901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3043365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380492480,"raw_hex":"e825702e00","size":5,"soft_fail":false,"target_va":5383536672,"va":5380493302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3045953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380492480,"raw_hex":"e8417a2e00","size":5,"soft_fail":false,"target_va":5383539152,"va":5380493194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::WithRg::run
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":567,"record_offset":6060,"parent_record_offset":2524,"ancestor_record_offset":2524,"inlinee":12358,"source_line":489,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":567,"record_offset":2736,"parent_record_offset":2524,"ancestor_record_offset":2524,"inlinee":12326,"source_line":34,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":567,"record_offset":4424,"parent_record_offset":2524,"ancestor_record_offset":2524,"inlinee":12361,"source_line":41,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":4068,"parent_record_offset":4044,"ancestor_record_offset":2524,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":4116,"parent_record_offset":4092,"ancestor_record_offset":2524,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":4092,"parent_record_offset":4068,"ancestor_record_offset":2524,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":4044,"parent_record_offset":4020,"ancestor_record_offset":2524,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":4020,"parent_record_offset":3980,"ancestor_record_offset":2524,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":3980,"parent_record_offset":3956,"ancestor_record_offset":2524,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":567,"record_offset":2792,"parent_record_offset":2764,"ancestor_record_offset":2524,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":567,"record_offset":4340,"parent_record_offset":4316,"ancestor_record_offset":2524,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":567,"record_offset":4316,"parent_record_offset":4292,"ancestor_record_offset":2524,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":567,"record_offset":4264,"parent_record_offset":4232,"ancestor_record_offset":2524,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":567,"record_offset":4208,"parent_record_offset":3704,"ancestor_record_offset":2524,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":567,"record_offset":5732,"parent_record_offset":5376,"ancestor_record_offset":2524,"inlinee":4185,"source_line":125,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":567,"record_offset":4232,"parent_record_offset":4208,"ancestor_record_offset":2524,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":567,"record_offset":3732,"parent_record_offset":3704,"ancestor_record_offset":2524,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":567,"record_offset":4292,"parent_record_offset":4232,"ancestor_record_offset":2524,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":567,"record_offset":5816,"parent_record_offset":5756,"ancestor_record_offset":2524,"inlinee":4197,"source_line":132,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":567,"record_offset":3780,"parent_record_offset":3756,"ancestor_record_offset":2524,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":567,"record_offset":3756,"parent_record_offset":3732,"ancestor_record_offset":2524,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":567,"record_offset":3956,"parent_record_offset":3932,"ancestor_record_offset":2524,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":567,"record_offset":5932,"parent_record_offset":5908,"ancestor_record_offset":2524,"inlinee":4201,"source_line":2868,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":567,"record_offset":5908,"parent_record_offset":5376,"ancestor_record_offset":2524,"inlinee":4200,"source_line":302,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":567,"record_offset":2764,"parent_record_offset":2736,"ancestor_record_offset":2524,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":567,"record_offset":3704,"parent_record_offset":2764,"ancestor_record_offset":2524,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":567,"record_offset":4456,"parent_record_offset":4424,"ancestor_record_offset":2524,"inlinee":9395,"source_line":515,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":567,"record_offset":5376,"parent_record_offset":4456,"ancestor_record_offset":2524,"inlinee":4195,"source_line":540,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":567,"record_offset":2816,"parent_record_offset":2792,"ancestor_record_offset":2524,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":567,"record_offset":3452,"parent_record_offset":3428,"ancestor_record_offset":2524,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":567,"record_offset":3480,"parent_record_offset":3452,"ancestor_record_offset":2524,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":567,"record_offset":3028,"parent_record_offset":3004,"ancestor_record_offset":2524,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":567,"record_offset":3112,"parent_record_offset":3088,"ancestor_record_offset":2524,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":567,"record_offset":3260,"parent_record_offset":3232,"ancestor_record_offset":2524,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":567,"record_offset":3232,"parent_record_offset":3204,"ancestor_record_offset":2524,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":567,"record_offset":3868,"parent_record_offset":3808,"ancestor_record_offset":2524,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":567,"record_offset":3892,"parent_record_offset":3868,"ancestor_record_offset":2524,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":567,"record_offset":3932,"parent_record_offset":3756,"ancestor_record_offset":2524,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":567,"record_offset":4392,"parent_record_offset":2736,"ancestor_record_offset":2524,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":3052,"parent_record_offset":3028,"ancestor_record_offset":2524,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":567,"record_offset":3372,"parent_record_offset":3348,"ancestor_record_offset":2524,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":567,"record_offset":3808,"parent_record_offset":3780,"ancestor_record_offset":2524,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":567,"record_offset":3840,"parent_record_offset":3808,"ancestor_record_offset":2524,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":567,"record_offset":4160,"parent_record_offset":3980,"ancestor_record_offset":2524,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":567,"record_offset":5956,"parent_record_offset":5932,"ancestor_record_offset":2524,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":567,"record_offset":5980,"parent_record_offset":5956,"ancestor_record_offset":2524,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$11::fetch
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":567,"record_offset":6120,"parent_record_offset":6096,"ancestor_record_offset":2524,"inlinee":30203,"source_line":568,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$11::unfetch
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":567,"record_offset":6468,"parent_record_offset":6444,"ancestor_record_offset":2524,"inlinee":30215,"source_line":575,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$22::fetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":567,"record_offset":6096,"parent_record_offset":6060,"ancestor_record_offset":2524,"inlinee":30273,"source_line":593,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$22::unfetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":567,"record_offset":6444,"parent_record_offset":6060,"ancestor_record_offset":2524,"inlinee":30280,"source_line":596,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::garden::spawn_bushes_buildings::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":567,"record_offset":6284,"parent_record_offset":6248,"ancestor_record_offset":2524,"inlinee":35733,"source_line":8162,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::garden::spawn_flowers_buildings::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":567,"record_offset":6380,"parent_record_offset":6344,"ancestor_record_offset":2524,"inlinee":35735,"source_line":8268,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::compute_passes::spawn_vegetation_around_buildings::spawn_vegetation_around_buildings::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/spawn_vegetation_around_buildings.rs
//     evidence={"module":567,"record_offset":6208,"parent_record_offset":6060,"ancestor_record_offset":2524,"inlinee":35731,"source_line":27,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":567,"record_offset":3428,"parent_record_offset":3408,"ancestor_record_offset":2524,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":567,"record_offset":3172,"parent_record_offset":3144,"ancestor_record_offset":2524,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":567,"record_offset":3204,"parent_record_offset":3172,"ancestor_record_offset":2524,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":567,"record_offset":3144,"parent_record_offset":2924,"ancestor_record_offset":2524,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":567,"record_offset":3004,"parent_record_offset":2924,"ancestor_record_offset":2524,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":567,"record_offset":3556,"parent_record_offset":2924,"ancestor_record_offset":2524,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":567,"record_offset":3088,"parent_record_offset":2924,"ancestor_record_offset":2524,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":567,"record_offset":2848,"parent_record_offset":2816,"ancestor_record_offset":2524,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":567,"record_offset":3408,"parent_record_offset":3304,"ancestor_record_offset":2524,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":567,"record_offset":3348,"parent_record_offset":3324,"ancestor_record_offset":2524,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":567,"record_offset":3648,"parent_record_offset":2924,"ancestor_record_offset":2524,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":567,"record_offset":3324,"parent_record_offset":3304,"ancestor_record_offset":2524,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":567,"record_offset":2900,"parent_record_offset":2876,"ancestor_record_offset":2524,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":567,"record_offset":2876,"parent_record_offset":2848,"ancestor_record_offset":2524,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":567,"record_offset":2924,"parent_record_offset":2900,"ancestor_record_offset":2524,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":567,"record_offset":2976,"parent_record_offset":2924,"ancestor_record_offset":2524,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":567,"record_offset":3304,"parent_record_offset":2924,"ancestor_record_offset":2524,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":567,"record_offset":3528,"parent_record_offset":2924,"ancestor_record_offset":2524,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":567,"record_offset":5652,"parent_record_offset":5628,"ancestor_record_offset":2524,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":567,"record_offset":6248,"parent_record_offset":6208,"ancestor_record_offset":2524,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":567,"record_offset":6316,"parent_record_offset":6208,"ancestor_record_offset":2524,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::spawn_vegetation_around_buildings::spawn_vegetation_around_buildings::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8B93
// 首条位置: {"module":567,"inlinee":35731}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":567,"record_offset":6248,"parent_record_offset":6208,"ancestor_record_offset":6208,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":567,"record_offset":6316,"parent_record_offset":6208,"ancestor_record_offset":6208,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::garden::spawn_bushes_buildings::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":567,"record_offset":6284,"parent_record_offset":6248,"ancestor_record_offset":6208,"inlinee":35733,"source_line":8162,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::garden::spawn_flowers_buildings::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":567,"record_offset":6380,"parent_record_offset":6344,"ancestor_record_offset":6208,"inlinee":35735,"source_line":8268,"archive":"_pdb/C/modules/00567.jsonl.gz"}
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
// 完整结构及原始类型记录: spawn_vegetation_around_buildings.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: spawn_vegetation_around_buildings.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/spawn_vegetation_around_buildings.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: spawn_vegetation_around_buildings.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::spawn_vegetation_around_buildings::spawn_vegetation_around_buildings
// [PDB Shader 类型/入口] {"shader_family":"country_core/garden/spawn_bushes_buildings","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::spawn_bushes_buildings::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":567,"record_offset":6284,"parent_record_offset":6248,"ancestor_record_offset":2524,"inlinee":35733,"source_line":8162,"archive":"_pdb/C/modules/00567.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/garden/spawn_flowers_buildings","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::spawn_flowers_buildings::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":567,"record_offset":6380,"parent_record_offset":6344,"ancestor_record_offset":2524,"inlinee":35735,"source_line":8268,"archive":"_pdb/C/modules/00567.jsonl.gz"}}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::spawn_vegetation_around_buildings::spawn_vegetation_around_buildings::closure$0
// [PDB Shader 类型/入口] {"shader_family":"country_core/garden/spawn_bushes_buildings","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::spawn_bushes_buildings::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":567,"record_offset":6284,"parent_record_offset":6248,"ancestor_record_offset":6208,"inlinee":35733,"source_line":8162,"archive":"_pdb/C/modules/00567.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/garden/spawn_flowers_buildings","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::spawn_flowers_buildings::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":567,"record_offset":6380,"parent_record_offset":6344,"ancestor_record_offset":6208,"inlinee":35735,"source_line":8268,"archive":"_pdb/C/modules/00567.jsonl.gz"}}
// [Shader 映射] {"shader_id":"5ba776e4f36b5dbceb8cb119","relative_path":"country-core/compiled/garden/spawn_bushes_buildings.precomp","sha256":"80091edbafee58f328699818d912155c30d87b172b319a01823e02538183ef7f","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::spawn_vegetation_around_buildings::spawn_vegetation_around_buildings","hint":{"shader_family":"country_core/garden/spawn_bushes_buildings","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::spawn_bushes_buildings::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":567,"record_offset":6284,"parent_record_offset":6248,"ancestor_record_offset":2524,"inlinee":35733,"source_line":8162,"archive":"_pdb/C/modules/00567.jsonl.gz"}},"shader_uri":"country_core/garden/spawn_bushes_buildings:main"}
// [Shader 映射] {"shader_id":"781a22af572c6e95b380a436","relative_path":"country-core/compiled/garden/spawn_flowers_buildings.precomp","sha256":"5e3450b7840530fe4a54dd59c3d2912318ea4d3bc5b9f77d90ab7fa4cd3d78e0","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::spawn_vegetation_around_buildings::spawn_vegetation_around_buildings","hint":{"shader_family":"country_core/garden/spawn_flowers_buildings","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::spawn_flowers_buildings::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":567,"record_offset":6380,"parent_record_offset":6344,"ancestor_record_offset":2524,"inlinee":35735,"source_line":8268,"archive":"_pdb/C/modules/00567.jsonl.gz"}},"shader_uri":"country_core/garden/spawn_flowers_buildings:main"}
// [Shader 映射] {"shader_id":"5ba776e4f36b5dbceb8cb119","relative_path":"country-core/compiled/garden/spawn_bushes_buildings.precomp","sha256":"80091edbafee58f328699818d912155c30d87b172b319a01823e02538183ef7f","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::spawn_vegetation_around_buildings::spawn_vegetation_around_buildings::closure$0","hint":{"shader_family":"country_core/garden/spawn_bushes_buildings","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::spawn_bushes_buildings::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":567,"record_offset":6284,"parent_record_offset":6248,"ancestor_record_offset":6208,"inlinee":35733,"source_line":8162,"archive":"_pdb/C/modules/00567.jsonl.gz"}},"shader_uri":"country_core/garden/spawn_bushes_buildings:main"}
// [Shader 映射] {"shader_id":"781a22af572c6e95b380a436","relative_path":"country-core/compiled/garden/spawn_flowers_buildings.precomp","sha256":"5e3450b7840530fe4a54dd59c3d2912318ea4d3bc5b9f77d90ab7fa4cd3d78e0","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::spawn_vegetation_around_buildings::spawn_vegetation_around_buildings::closure$0","hint":{"shader_family":"country_core/garden/spawn_flowers_buildings","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::spawn_flowers_buildings::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":567,"record_offset":6380,"parent_record_offset":6344,"ancestor_record_offset":6208,"inlinee":35735,"source_line":8268,"archive":"_pdb/C/modules/00567.jsonl.gz"}},"shader_uri":"country_core/garden/spawn_flowers_buildings:main"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/garden/spawn_bushes_buildings.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[67]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":58,"name":"globalParams","storage_class":"Uniform","type":{"id":57,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,55]},"decorations":[],"pointee":{"id":55,"name":"GlobalParams_natural","record":{"opcode":"OpTypeStruct","operands":[17,56]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"draw_indirect_buffer_offset","type_id":17,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"grid_dims","type_id":56,"decorations":[{"decoration":"Offset","operands":[4]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":168,"name":"path_mask","storage_class":"Uniform","type":{"id":167,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,166]},"decorations":[],"pointee":{"id":166,"name":"PathExclusionMask_natural","record":{"opcode":"OpTypeStruct","operands":[10]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"tex_dims","type_id":10,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":6,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[6]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":190,"name":"path_mask.tex","storage_class":"UniformConstant","type":{"id":189,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,187]},"decorations":[],"pointee":{"id":187,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[16,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":6,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[6]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":195,"name":"rocky_terrain.tex","storage_class":"UniformConstant","type":{"id":189,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,187]},"decorations":[],"pointee":{"id":187,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[16,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":9,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[9]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":200,"name":"water_mask.tex","storage_class":"UniformConstant","type":{"id":189,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,187]},"decorations":[],"pointee":{"id":187,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[16,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":10,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":205,"name":"foliage_exclusion_mask.tex","storage_class":"UniformConstant","type":{"id":189,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,187]},"decorations":[],"pointee":{"id":187,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[16,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":12,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[12]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":209,"name":"samplers.llb","storage_class":"UniformConstant","type":{"id":208,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,206]},"decorations":[],"pointee":{"id":206,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":4,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[4]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":219,"name":"terrain_heightmap","storage_class":"Uniform","type":{"id":218,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,216]},"decorations":[],"pointee":{"id":216,"name":"TerrainHeightmap_natural","record":{"opcode":"OpTypeStruct","operands":[10,217,217,26,27]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"terrain_heightmap_dims","type_id":10,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"bg_terrain_heightmap","type_id":217,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"bg_terrain_fog_height","type_id":217,"decorations":[{"decoration":"Offset","operands":[12]}]},{"index":3,"name":"bg_terrain_height_bounds_ws","type_id":26,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":4,"name":"bg_water_footprint_ws","type_id":27,"decorations":[{"decoration":"Offset","operands":[24]}]}]}},"descriptor_set":17,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[17]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":258,"name":"samplers.lnc_iso","storage_class":"UniformConstant","type":{"id":208,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,206]},"decorations":[],"pointee":{"id":206,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":5,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[5]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":260,"name":"terrain_heightmap.terrain_heightmap","storage_class":"UniformConstant","type":{"id":189,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,187]},"decorations":[],"pointee":{"id":187,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[16,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":17,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[17]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":387,"name":"plant_sample_mask.tex","storage_class":"UniformConstant","type":{"id":386,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,384]},"decorations":[],"pointee":{"id":384,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[9,1,2,0,0,1,32]},"decorations":[]}},"descriptor_set":8,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[8]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":417,"name":"garden_mask.tex","storage_class":"UniformConstant","type":{"id":189,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,187]},"decorations":[],"pointee":{"id":187,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[16,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":13,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[13]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":426,"name":"building_mask.tex","storage_class":"UniformConstant","type":{"id":189,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,187]},"decorations":[],"pointee":{"id":187,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[16,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":11,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":451,"name":"building_mask.non_blurred_tex","storage_class":"UniformConstant","type":{"id":189,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,187]},"decorations":[],"pointee":{"id":187,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[16,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":11,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":701,"name":"frame_constants","storage_class":"Uniform","type":{"id":700,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,699]},"decorations":[],"pointee":{"id":699,"name":"FrameConstants_natural","record":{"opcode":"OpTypeStruct","operands":[27,27,27,27,27,27,27,27,16,16,16,9,10,9,16,21,16,21,9,21,9,21,16,27,16,9,10,16,9,9,9,21,16,9,9,9,9,9,16,16,16,27,21,16]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":27,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":27,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":27,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":27,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":27,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":27,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":27,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":27,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"time_since_startup_secs","type_id":16,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":16,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":16,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":9,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"world_render_resolution","type_id":10,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":9,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":16,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":21,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":16,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":21,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":9,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":21,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":9,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"dir_to_primary_directional_light","type_id":21,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"main_view_lod_scale","type_id":16,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":23,"name":"main_view_eye_position","type_id":27,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":24,"name":"fog_relative_extinction","type_id":16,"decorations":[{"decoration":"Offset","operands":[240]}]},{"index":25,"name":"highlighted_transient_stair_edge_id","type_id":9,"decorations":[{"decoration":"Offset","operands":[244]}]},{"index":26,"name":"swapchain_dims","type_id":10,"decorations":[{"decoration":"Offset","operands":[248]}]},{"index":27,"name":"swapchain_aspect_ratio","type_id":16,"decorations":[{"decoration":"Offset","operands":[256]}]},{"index":28,"name":"is_photomode","type_id":9,"decorations":[{"decoration":"Offset","operands":[260]}]},{"index":29,"name":"is_topdown_mode","type_id":9,"decorations":[{"decoration":"Offset","operands":[264]}]},{"index":30,"name":"is_firstperson_mode","type_id":9,"decorations":[{"decoration":"Offset","operands":[268]}]},{"index":31,"name":"brush_position","type_id":21,"decorations":[{"decoration":"Offset","operands":[272]}]},{"index":32,"name":"eraser_size_multiplier","type_id":16,"decorations":[{"decoration":"Offset","operands":[284]}]},{"index":33,"name":"roof_highlighted_slot_id","type_id":9,"decorations":[{"decoration":"Offset","operands":[288]}]},{"index":34,"name":"roof_highlight_mode","type_id":9,"decorations":[{"decoration":"Offset","operands":[292]}]},{"index":35,"name":"wall_highlighted_slot_id","type_id":9,"decorations":[{"decoration":"Offset","operands":[296]}]},{"index":36,"name":"wall_highlight_mode","type_id":9,"decorations":[{"decoration":"Offset","operands":[300]}]},{"index":37,"name":"highlight_bricks_to_delete","type_id":9,"decorations":[{"decoration":"Offset","operands":[304]}]},{"index":38,"name":"wind_strength","type_id":16,"decorations":[{"decoration":"Offset","operands":[308]}]},{"index":39,"name":"tree_canopy_cone_angle_mult","type_id":16,"decorations":[{"decoration":"Offset","operands":[312]}]},{"index":40,"name":"tree_canopy_max_cone_cull_dist_range","type_id":16,"decorations":[{"decoration":"Offset","operands":[316]}]},{"index":41,"name":"debug_tweakers","type_id":27,"decorations":[{"decoration":"Offset","operands":[320]}]},{"index":42,"name":"ground_fog","type_id":21,"decorations":[{"decoration":"Offset","operands":[336]}]},{"index":43,"name":"moonswap","type_id":16,"decorations":[{"decoration":"Offset","operands":[348]}]}]}},"descriptor_set":4,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[4]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":950,"name":"draw_commands","storage_class":"Uniform","type":{"id":949,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,947]},"decorations":[],"pointee":{"id":947,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[948]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":948,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":974,"name":"bush_buffer","storage_class":"Uniform","type":{"id":973,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,971]},"decorations":[],"pointee":{"id":971,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[972]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":972,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":1075,"name":"bush_flowers","storage_class":"Uniform","type":{"id":1074,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,1072]},"decorations":[],"pointee":{"id":1072,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[1073]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":1073,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[3]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":67,"name":"unknown","storage_class":"Input","type":{"id":66,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,64]},"decorations":[],"pointee":{"id":64,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[9,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/garden/spawn_flowers_buildings.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[21]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"globalParams","storage_class":"Uniform","type":{"id":8,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,5]},"decorations":[],"pointee":{"id":5,"name":"GlobalParams_natural","record":{"opcode":"OpTypeStruct","operands":[6,7]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"draw_indirect_buffer_offset","type_id":6,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"grid_dims","type_id":7,"decorations":[{"decoration":"Offset","operands":[4]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":186,"name":"path_mask","storage_class":"Uniform","type":{"id":185,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,184]},"decorations":[],"pointee":{"id":184,"name":"PathExclusionMask_natural","record":{"opcode":"OpTypeStruct","operands":[22]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"tex_dims","type_id":22,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":6,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[6]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":208,"name":"path_mask.tex","storage_class":"UniformConstant","type":{"id":207,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,205]},"decorations":[],"pointee":{"id":205,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[14,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":6,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[6]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":213,"name":"rocky_terrain.tex","storage_class":"UniformConstant","type":{"id":207,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,205]},"decorations":[],"pointee":{"id":205,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[14,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":9,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[9]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":218,"name":"water_mask.tex","storage_class":"UniformConstant","type":{"id":207,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,205]},"decorations":[],"pointee":{"id":205,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[14,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":10,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":223,"name":"foliage_exclusion_mask.tex","storage_class":"UniformConstant","type":{"id":207,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,205]},"decorations":[],"pointee":{"id":205,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[14,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":12,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[12]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":227,"name":"samplers.llb","storage_class":"UniformConstant","type":{"id":226,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,224]},"decorations":[],"pointee":{"id":224,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":4,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[4]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":237,"name":"terrain_heightmap","storage_class":"Uniform","type":{"id":236,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,234]},"decorations":[],"pointee":{"id":234,"name":"TerrainHeightmap_natural","record":{"opcode":"OpTypeStruct","operands":[22,235,235,15,144]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"terrain_heightmap_dims","type_id":22,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"bg_terrain_heightmap","type_id":235,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"bg_terrain_fog_height","type_id":235,"decorations":[{"decoration":"Offset","operands":[12]}]},{"index":3,"name":"bg_terrain_height_bounds_ws","type_id":15,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":4,"name":"bg_water_footprint_ws","type_id":144,"decorations":[{"decoration":"Offset","operands":[24]}]}]}},"descriptor_set":17,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[17]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":276,"name":"samplers.lnc_iso","storage_class":"UniformConstant","type":{"id":226,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,224]},"decorations":[],"pointee":{"id":224,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":5,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[5]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":278,"name":"terrain_heightmap.terrain_heightmap","storage_class":"UniformConstant","type":{"id":207,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,205]},"decorations":[],"pointee":{"id":205,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[14,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":17,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[17]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":405,"name":"plant_sample_mask.tex","storage_class":"UniformConstant","type":{"id":404,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,402]},"decorations":[],"pointee":{"id":402,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[17,1,2,0,0,1,32]},"decorations":[]}},"descriptor_set":8,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[8]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":438,"name":"garden_mask.tex","storage_class":"UniformConstant","type":{"id":207,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,205]},"decorations":[],"pointee":{"id":205,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[14,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":13,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[13]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":447,"name":"building_mask.tex","storage_class":"UniformConstant","type":{"id":207,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,205]},"decorations":[],"pointee":{"id":205,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[14,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":11,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":497,"name":"building_mask.non_blurred_tex","storage_class":"UniformConstant","type":{"id":207,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,205]},"decorations":[],"pointee":{"id":205,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[14,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":11,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":1030,"name":"draw_commands","storage_class":"Uniform","type":{"id":1029,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,1027]},"decorations":[],"pointee":{"id":1027,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[1028]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":1028,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":1052,"name":"flower_buffer","storage_class":"Uniform","type":{"id":1051,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,1049]},"decorations":[],"pointee":{"id":1049,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[1050]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":1050,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":21,"name":"unknown","storage_class":"Input","type":{"id":20,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,18]},"decorations":[],"pointee":{"id":18,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[17,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":5678848,"caller_aliases":{"alias_group_rva":5678848,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5679695,"position_among_mapped_render_targets":22,"mapped_render_target_count":74,"previous_callsite":5679684,"next_callsite":5679706,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/compute_water.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/read_bush_positions_for_cpu.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: spawn_vegetation_around_buildings.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::spawn_vegetation_around_buildings::spawn_vegetation_around_buildings
// 信息 ID: b7cddb643fbaf799547a0d30
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56aa4f -> 0xb3ccc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::spawn_vegetation_around_buildings::spawn_vegetation_around_buildings::closure$0
// 信息 ID: 8eb34d99da819d4ae741d689
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::compute_passes::spawn_vegetation_around_buildings::spawn_vegetation_around_buildings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::spawn_vegetation_around_buildings::spawn_vegetation_around_buildings; module=567, inline_record=6208; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
