// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/render_loop/compute_passes/read_bush_positions_for_cpu.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 235d7c7e251d839f7d6fafa28fdaf0fb735b52469cfe32e4c9783b90932ce18d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: read_bush_positions_for_cpu.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 518
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
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::read_bush_positions_for_cpu
// 记录形式: compiled_procedure; instances=1
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":518,"record_offset":54368,"rva":10224784,"va":5378933904}
// 编译实例 RVAs: 0x9c0490
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1415394d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: read_bush_positions_for_cpu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12029726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378933904,"raw_hex":"e81e8fb700","size":5,"soft_fail":false,"target_va":5390963920,"va":5378934189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14153b9d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: read_bush_positions_for_cpu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12039179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378933904,"raw_hex":"e80bb4b700","size":5,"soft_fail":false,"target_va":5390973392,"va":5378934208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14153e5c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: read_bush_positions_for_cpu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12050408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378933904,"raw_hex":"e8e8dfb700","size":5,"soft_fail":false,"target_va":5390984640,"va":5378934227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141547170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: read_bush_positions_for_cpu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12085916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378933904,"raw_hex":"e89c6ab800","size":5,"soft_fail":false,"target_va":5391020400,"va":5378934479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141548e50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: read_bush_positions_for_cpu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12093297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378933904,"raw_hex":"e87187b800","size":5,"soft_fail":false,"target_va":5391027792,"va":5378934490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141549ad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: read_bush_positions_for_cpu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12096486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378933904,"raw_hex":"e8e693b800","size":5,"soft_fail":false,"target_va":5391030992,"va":5378934501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141770e40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: read_bush_positions_for_cpu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14354406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378933904,"raw_hex":"e8e607db00","size":5,"soft_fail":false,"target_va":5393288768,"va":5378934357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141782fe0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: read_bush_positions_for_cpu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14428470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378933904,"raw_hex":"e83629dc00","size":5,"soft_fail":false,"target_va":5393362912,"va":5378934437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37687062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378933904,"raw_hex":"e8160f3f02","size":5,"soft_fail":false,"target_va":5416621600,"va":5378934533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4602199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378933904,"raw_hex":"e857394600","size":5,"soft_fail":false,"target_va":5383536672,"va":5378934468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4604757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378933904,"raw_hex":"e855434600","size":5,"soft_fail":false,"target_va":5383539152,"va":5378934390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::WithRg::run
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":518,"record_offset":56380,"parent_record_offset":54368,"ancestor_record_offset":54368,"inlinee":12358,"source_line":489,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":518,"record_offset":54564,"parent_record_offset":54368,"ancestor_record_offset":54368,"inlinee":12361,"source_line":41,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":518,"record_offset":55912,"parent_record_offset":55888,"ancestor_record_offset":54368,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":518,"record_offset":55960,"parent_record_offset":55936,"ancestor_record_offset":54368,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":518,"record_offset":55936,"parent_record_offset":55912,"ancestor_record_offset":54368,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":518,"record_offset":55888,"parent_record_offset":55864,"ancestor_record_offset":54368,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":518,"record_offset":55864,"parent_record_offset":55824,"ancestor_record_offset":54368,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":518,"record_offset":55824,"parent_record_offset":55800,"ancestor_record_offset":54368,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":518,"record_offset":54620,"parent_record_offset":54592,"ancestor_record_offset":54368,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":518,"record_offset":56184,"parent_record_offset":56160,"ancestor_record_offset":54368,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":518,"record_offset":56160,"parent_record_offset":56136,"ancestor_record_offset":54368,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":518,"record_offset":56108,"parent_record_offset":56076,"ancestor_record_offset":54368,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":518,"record_offset":56052,"parent_record_offset":55540,"ancestor_record_offset":54368,"inlinee":4185,"source_line":125,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":518,"record_offset":56076,"parent_record_offset":56052,"ancestor_record_offset":54368,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":518,"record_offset":55572,"parent_record_offset":55540,"ancestor_record_offset":54368,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":518,"record_offset":56136,"parent_record_offset":56076,"ancestor_record_offset":54368,"inlinee":4197,"source_line":132,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":518,"record_offset":55620,"parent_record_offset":55596,"ancestor_record_offset":54368,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":518,"record_offset":55596,"parent_record_offset":55572,"ancestor_record_offset":54368,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":518,"record_offset":55800,"parent_record_offset":55776,"ancestor_record_offset":54368,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":518,"record_offset":56252,"parent_record_offset":56228,"ancestor_record_offset":54368,"inlinee":4201,"source_line":2868,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":518,"record_offset":56228,"parent_record_offset":55540,"ancestor_record_offset":54368,"inlinee":4200,"source_line":302,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":518,"record_offset":54592,"parent_record_offset":54564,"ancestor_record_offset":54368,"inlinee":9395,"source_line":515,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":518,"record_offset":55540,"parent_record_offset":54592,"ancestor_record_offset":54368,"inlinee":4195,"source_line":540,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":518,"record_offset":54644,"parent_record_offset":54620,"ancestor_record_offset":54368,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":518,"record_offset":55288,"parent_record_offset":55264,"ancestor_record_offset":54368,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":518,"record_offset":55316,"parent_record_offset":55288,"ancestor_record_offset":54368,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":518,"record_offset":54860,"parent_record_offset":54836,"ancestor_record_offset":54368,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":518,"record_offset":54944,"parent_record_offset":54920,"ancestor_record_offset":54368,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":518,"record_offset":55092,"parent_record_offset":55064,"ancestor_record_offset":54368,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":518,"record_offset":55064,"parent_record_offset":55036,"ancestor_record_offset":54368,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":518,"record_offset":55712,"parent_record_offset":55648,"ancestor_record_offset":54368,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":518,"record_offset":55736,"parent_record_offset":55712,"ancestor_record_offset":54368,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":518,"record_offset":55776,"parent_record_offset":55596,"ancestor_record_offset":54368,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":518,"record_offset":56348,"parent_record_offset":54564,"ancestor_record_offset":54368,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":518,"record_offset":54884,"parent_record_offset":54860,"ancestor_record_offset":54368,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":518,"record_offset":55204,"parent_record_offset":55180,"ancestor_record_offset":54368,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":518,"record_offset":55648,"parent_record_offset":55620,"ancestor_record_offset":54368,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":518,"record_offset":55684,"parent_record_offset":55648,"ancestor_record_offset":54368,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":518,"record_offset":56004,"parent_record_offset":55824,"ancestor_record_offset":54368,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":518,"record_offset":56276,"parent_record_offset":56252,"ancestor_record_offset":54368,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":518,"record_offset":56300,"parent_record_offset":56276,"ancestor_record_offset":54368,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::fetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":518,"record_offset":56436,"parent_record_offset":56412,"ancestor_record_offset":54368,"inlinee":9087,"source_line":545,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::unfetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":518,"record_offset":56676,"parent_record_offset":56652,"ancestor_record_offset":54368,"inlinee":9089,"source_line":552,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$11::fetch
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":518,"record_offset":56464,"parent_record_offset":56412,"ancestor_record_offset":54368,"inlinee":30203,"source_line":568,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$11::unfetch
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":518,"record_offset":56704,"parent_record_offset":56652,"ancestor_record_offset":54368,"inlinee":30215,"source_line":575,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$22::fetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":518,"record_offset":56412,"parent_record_offset":56380,"ancestor_record_offset":54368,"inlinee":30273,"source_line":593,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$22::unfetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":518,"record_offset":56652,"parent_record_offset":56380,"ancestor_record_offset":54368,"inlinee":30280,"source_line":596,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::garden::read_bush_positions_for_cpu::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":518,"record_offset":56588,"parent_record_offset":56552,"ancestor_record_offset":54368,"inlinee":30439,"source_line":8106,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::read_bush_positions_for_cpu::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/read_bush_positions_for_cpu.rs
//     evidence={"module":518,"record_offset":56524,"parent_record_offset":56380,"ancestor_record_offset":54368,"inlinee":30437,"source_line":20,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":518,"record_offset":55264,"parent_record_offset":55240,"ancestor_record_offset":54368,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":518,"record_offset":55004,"parent_record_offset":54976,"ancestor_record_offset":54368,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":518,"record_offset":55036,"parent_record_offset":55004,"ancestor_record_offset":54368,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":518,"record_offset":54976,"parent_record_offset":54756,"ancestor_record_offset":54368,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":518,"record_offset":54836,"parent_record_offset":54756,"ancestor_record_offset":54368,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":518,"record_offset":55392,"parent_record_offset":54756,"ancestor_record_offset":54368,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":518,"record_offset":54920,"parent_record_offset":54756,"ancestor_record_offset":54368,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":518,"record_offset":54680,"parent_record_offset":54644,"ancestor_record_offset":54368,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":518,"record_offset":55240,"parent_record_offset":55136,"ancestor_record_offset":54368,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":518,"record_offset":55180,"parent_record_offset":55156,"ancestor_record_offset":54368,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":518,"record_offset":55484,"parent_record_offset":54756,"ancestor_record_offset":54368,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":518,"record_offset":55156,"parent_record_offset":55136,"ancestor_record_offset":54368,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":518,"record_offset":54732,"parent_record_offset":54708,"ancestor_record_offset":54368,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":518,"record_offset":54708,"parent_record_offset":54680,"ancestor_record_offset":54368,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":518,"record_offset":54756,"parent_record_offset":54732,"ancestor_record_offset":54368,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":518,"record_offset":54808,"parent_record_offset":54756,"ancestor_record_offset":54368,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":518,"record_offset":55136,"parent_record_offset":54756,"ancestor_record_offset":54368,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":518,"record_offset":55364,"parent_record_offset":54756,"ancestor_record_offset":54368,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":518,"record_offset":56552,"parent_record_offset":56524,"ancestor_record_offset":54368,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":518,"record_offset":56620,"parent_record_offset":56524,"ancestor_record_offset":54368,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::read_bush_positions_for_cpu::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x76E5
// 首条位置: {"module":518,"inlinee":30437}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":518,"record_offset":56552,"parent_record_offset":56524,"ancestor_record_offset":56524,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":518,"record_offset":56620,"parent_record_offset":56524,"ancestor_record_offset":56524,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::garden::read_bush_positions_for_cpu::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":518,"record_offset":56588,"parent_record_offset":56552,"ancestor_record_offset":56524,"inlinee":30439,"source_line":8106,"archive":"_pdb/C/modules/00518.jsonl.gz"}
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
// 完整结构及原始类型记录: read_bush_positions_for_cpu.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::BushCpuSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer<country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::BushCpuSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::BushCpuSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer<country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::BushCpuSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::BushCpuSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::BushCpuSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::BushCpuSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0<country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::BushCpuSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0", "arguments": ["country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::BushCpuSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$4::new_storage_buffer<country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::BushCpuSsbo>", "constructor": "country_core::render::ssbo::impl$4::new_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::BushCpuSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$1::as_any_mut<country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::BushCpuSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$1::as_any_mut", "arguments": ["country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::BushCpuSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$1::as_any<country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::BushCpuSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$1::as_any", "arguments": ["country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::BushCpuSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::MappedStorageBuffer<country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::BushCpuSsbo>", "constructor": "country_core::render::ssbo::MappedStorageBuffer", "arguments": ["country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::BushCpuSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::MappedStorageBuffer<country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::BushCpuSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::MappedStorageBuffer<country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::BushCpuSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: read_bush_positions_for_cpu.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/read_bush_positions_for_cpu.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: read_bush_positions_for_cpu.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::read_bush_positions_for_cpu
// [PDB Shader 类型/入口] {"shader_family":"country_core/garden/read_bush_positions_for_cpu","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::read_bush_positions_for_cpu::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":518,"record_offset":56588,"parent_record_offset":56552,"ancestor_record_offset":54368,"inlinee":30439,"source_line":8106,"archive":"_pdb/C/modules/00518.jsonl.gz"}}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::read_bush_positions_for_cpu::closure$0
// [PDB Shader 类型/入口] {"shader_family":"country_core/garden/read_bush_positions_for_cpu","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::read_bush_positions_for_cpu::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":518,"record_offset":56588,"parent_record_offset":56552,"ancestor_record_offset":56524,"inlinee":30439,"source_line":8106,"archive":"_pdb/C/modules/00518.jsonl.gz"}}
// [Shader 映射] {"shader_id":"d37b0e3fdcb17733c3e7ff36","relative_path":"country-core/compiled/garden/read_bush_positions_for_cpu.precomp","sha256":"dc91d64265c2a78e779df11dd2a6fd69ff7a9a343471784006a5507e13efdd96","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::read_bush_positions_for_cpu","hint":{"shader_family":"country_core/garden/read_bush_positions_for_cpu","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::read_bush_positions_for_cpu::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":518,"record_offset":56588,"parent_record_offset":56552,"ancestor_record_offset":54368,"inlinee":30439,"source_line":8106,"archive":"_pdb/C/modules/00518.jsonl.gz"}},"shader_uri":"country_core/garden/read_bush_positions_for_cpu:main"}
// [Shader 映射] {"shader_id":"d37b0e3fdcb17733c3e7ff36","relative_path":"country-core/compiled/garden/read_bush_positions_for_cpu.precomp","sha256":"dc91d64265c2a78e779df11dd2a6fd69ff7a9a343471784006a5507e13efdd96","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::read_bush_positions_for_cpu::closure$0","hint":{"shader_family":"country_core/garden/read_bush_positions_for_cpu","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::garden::read_bush_positions_for_cpu::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":518,"record_offset":56588,"parent_record_offset":56552,"ancestor_record_offset":56524,"inlinee":30439,"source_line":8106,"archive":"_pdb/C/modules/00518.jsonl.gz"}},"shader_uri":"country_core/garden/read_bush_positions_for_cpu:main"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/garden/read_bush_positions_for_cpu.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[59]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[64,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":17,"name":"globalParams","storage_class":"Uniform","type":{"id":16,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,14]},"decorations":[],"pointee":{"id":14,"name":"GlobalParams_natural","record":{"opcode":"OpTypeStruct","operands":[15]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"draw_indirect_buffer_offset","type_id":15,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":28,"name":"draw_commands","storage_class":"Uniform","type":{"id":27,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,25]},"decorations":[],"pointee":{"id":25,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[26]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":26,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":82,"name":"bush_cpu","storage_class":"Uniform","type":{"id":81,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,79]},"decorations":[],"pointee":{"id":79,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[80]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":80,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[3]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":120,"name":"bush_buffer","storage_class":"Uniform","type":{"id":119,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,117]},"decorations":[],"pointee":{"id":117,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[118]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":118,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[0]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":59,"name":"unknown","storage_class":"Input","type":{"id":58,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,56]},"decorations":[],"pointee":{"id":56,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[6,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":5678848,"caller_aliases":{"alias_group_rva":5678848,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5679706,"position_among_mapped_render_targets":23,"mapped_render_target_count":74,"previous_callsite":5679695,"next_callsite":5679717,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/spawn_vegetation_around_buildings.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/compute_garden.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: read_bush_positions_for_cpu.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::audio::forest_audio::ForestAudioState>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::InteractableForestTree>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::audio::forest_audio::ForestAudioState>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::InteractableForestTree>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::forest_audio::ForestAudioState>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::InteractableForestTree>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> > (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] tuple$<i64,std::time::Instant> (*)()
// 候选参数类型: []

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::read_bush_positions_for_cpu
// 信息 ID: f89e82830c87a306f3418788
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56aa5a -> 0x9c0490 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::read_bush_positions_for_cpu::closure$0
// 信息 ID: c6d494a6fa5194696ea6c6e1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::read_bush_positions_for_cpu", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::read_bush_positions_for_cpu; module=518, inline_record=56524; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
