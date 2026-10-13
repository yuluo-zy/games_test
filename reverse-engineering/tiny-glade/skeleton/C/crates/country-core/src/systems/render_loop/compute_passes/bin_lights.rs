// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/render_loop/compute_passes/bin_lights.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: feb0ec5b4da944eb3193b3a7325c7640b159a5db146765e084137b1eb5900838
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: bin_lights.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 544
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
// 函数符号: country_core::systems::render_loop::compute_passes::bin_lights::bin_lights
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":544,"record_offset":82764,"rva":11063856,"va":5379772976}
// 编译实例 RVAs: 0xa8d230
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140c45c10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bin_lights.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1804472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379772976,"raw_hex":"e8b8881b00","size":5,"soft_fail":false,"target_va":5381577744,"va":5379773267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141537220
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bin_lights.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11181706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379772976,"raw_hex":"e88a9eaa00","size":5,"soft_fail":false,"target_va":5390955040,"va":5379773329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141547950
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bin_lights.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11247570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379772976,"raw_hex":"e8d29fab00","size":5,"soft_fail":false,"target_va":5391022416,"va":5379774841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141770e40
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: bin_lights.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13515237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379772976,"raw_hex":"e8e539ce00","size":5,"soft_fail":false,"target_va":5393288768,"va":5379773526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417792d0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: bin_lights.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13548502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379772976,"raw_hex":"e8d6bbce00","size":5,"soft_fail":false,"target_va":5393322704,"va":5379774197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14177b1f0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: bin_lights.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13557069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379772976,"raw_hex":"e84dddce00","size":5,"soft_fail":false,"target_va":5393330672,"va":5379773598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141784e50
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: bin_lights.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13596806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379772976,"raw_hex":"e88678cf00","size":5,"soft_fail":false,"target_va":5393370704,"va":5379773893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14178b8c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bin_lights.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13623465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379772976,"raw_hex":"e8a9e0cf00","size":5,"soft_fail":false,"target_va":5393397952,"va":5379774482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141794e00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: bin_lights.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13662110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379772976,"raw_hex":"e89e77d000","size":5,"soft_fail":false,"target_va":5393436160,"va":5379774045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36846951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379772976,"raw_hex":"e8673d3202","size":5,"soft_fail":false,"target_va":5416621600,"va":5379774644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::api::GraphApi::clear_buffer
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3767668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379772976,"raw_hex":"e8747d3900","size":5,"soft_fail":false,"target_va":5383541136,"va":5379773463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::api::GraphApi::create_raw_resource
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3774720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379772976,"raw_hex":"e800993900","size":5,"soft_fail":false,"target_va":5383548144,"va":5379773419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3763016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379772976,"raw_hex":"e8486b3900","size":5,"soft_fail":false,"target_va":5383536672,"va":5379773651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3765588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379772976,"raw_hex":"e854753900","size":5,"soft_fail":false,"target_va":5383539152,"va":5379773559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource::BufferDesc::new
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3750807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379772976,"raw_hex":"e8973b3900","size":5,"soft_fail":false,"target_va":5383524160,"va":5379773348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5379772976,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5379773288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":544,"record_offset":84704,"parent_record_offset":82764,"ancestor_record_offset":82764,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":544,"record_offset":84644,"parent_record_offset":82764,"ancestor_record_offset":82764,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::WithRg::run
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":544,"record_offset":84824,"parent_record_offset":82764,"ancestor_record_offset":82764,"inlinee":12358,"source_line":489,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::impl$9::instance_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":544,"record_offset":84740,"parent_record_offset":82764,"ancestor_record_offset":82764,"inlinee":12186,"source_line":407,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":544,"record_offset":82940,"parent_record_offset":82764,"ancestor_record_offset":82764,"inlinee":12326,"source_line":34,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":544,"record_offset":84288,"parent_record_offset":84264,"ancestor_record_offset":82764,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":544,"record_offset":84336,"parent_record_offset":84312,"ancestor_record_offset":82764,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":544,"record_offset":84312,"parent_record_offset":84288,"ancestor_record_offset":82764,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":544,"record_offset":84264,"parent_record_offset":84240,"ancestor_record_offset":82764,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":544,"record_offset":84240,"parent_record_offset":84200,"ancestor_record_offset":82764,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":544,"record_offset":84200,"parent_record_offset":84176,"ancestor_record_offset":82764,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":544,"record_offset":82996,"parent_record_offset":82968,"ancestor_record_offset":82764,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":544,"record_offset":84560,"parent_record_offset":84536,"ancestor_record_offset":82764,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":544,"record_offset":84536,"parent_record_offset":84512,"ancestor_record_offset":82764,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":544,"record_offset":84484,"parent_record_offset":84452,"ancestor_record_offset":82764,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":544,"record_offset":84428,"parent_record_offset":83920,"ancestor_record_offset":82764,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":544,"record_offset":84452,"parent_record_offset":84428,"ancestor_record_offset":82764,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":544,"record_offset":83948,"parent_record_offset":83920,"ancestor_record_offset":82764,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":544,"record_offset":84512,"parent_record_offset":84452,"ancestor_record_offset":82764,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":544,"record_offset":83996,"parent_record_offset":83972,"ancestor_record_offset":82764,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":544,"record_offset":83972,"parent_record_offset":83948,"ancestor_record_offset":82764,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":544,"record_offset":84176,"parent_record_offset":84152,"ancestor_record_offset":82764,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":544,"record_offset":82968,"parent_record_offset":82940,"ancestor_record_offset":82764,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":544,"record_offset":83920,"parent_record_offset":82968,"ancestor_record_offset":82764,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":544,"record_offset":83020,"parent_record_offset":82996,"ancestor_record_offset":82764,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":544,"record_offset":83668,"parent_record_offset":83644,"ancestor_record_offset":82764,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":544,"record_offset":83696,"parent_record_offset":83668,"ancestor_record_offset":82764,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":544,"record_offset":83236,"parent_record_offset":83212,"ancestor_record_offset":82764,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":544,"record_offset":83320,"parent_record_offset":83296,"ancestor_record_offset":82764,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":544,"record_offset":83468,"parent_record_offset":83440,"ancestor_record_offset":82764,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":544,"record_offset":83440,"parent_record_offset":83412,"ancestor_record_offset":82764,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":544,"record_offset":84088,"parent_record_offset":84024,"ancestor_record_offset":82764,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":544,"record_offset":84112,"parent_record_offset":84088,"ancestor_record_offset":82764,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":544,"record_offset":84152,"parent_record_offset":83972,"ancestor_record_offset":82764,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":544,"record_offset":84612,"parent_record_offset":82940,"ancestor_record_offset":82764,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":544,"record_offset":83260,"parent_record_offset":83236,"ancestor_record_offset":82764,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":544,"record_offset":83584,"parent_record_offset":83560,"ancestor_record_offset":82764,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":544,"record_offset":84024,"parent_record_offset":83996,"ancestor_record_offset":82764,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":544,"record_offset":84060,"parent_record_offset":84024,"ancestor_record_offset":82764,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":544,"record_offset":84380,"parent_record_offset":84200,"ancestor_record_offset":82764,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":544,"record_offset":84764,"parent_record_offset":84740,"ancestor_record_offset":82764,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":544,"record_offset":84788,"parent_record_offset":84764,"ancestor_record_offset":82764,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":544,"record_offset":84668,"parent_record_offset":84644,"ancestor_record_offset":82764,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::fetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":544,"record_offset":84868,"parent_record_offset":84824,"ancestor_record_offset":82764,"inlinee":9087,"source_line":545,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::unfetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":544,"record_offset":85644,"parent_record_offset":84824,"ancestor_record_offset":82764,"inlinee":9089,"source_line":552,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::light_binning::bin_lights_to_cells_0::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":544,"record_offset":85076,"parent_record_offset":85040,"ancestor_record_offset":82764,"inlinee":33762,"source_line":18359,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::light_binning::bin_lights_to_cells_1::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":544,"record_offset":85524,"parent_record_offset":85488,"ancestor_record_offset":82764,"inlinee":33768,"source_line":18409,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::light_binning::cell_prefix_merge::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":544,"record_offset":85396,"parent_record_offset":85360,"ancestor_record_offset":82764,"inlinee":30678,"source_line":18444,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::light_binning::cell_prefix_sum::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":544,"record_offset":85204,"parent_record_offset":85168,"ancestor_record_offset":82764,"inlinee":33764,"source_line":18479,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::light_binning::cell_subtotal_prefix_sum::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":544,"record_offset":85300,"parent_record_offset":85264,"ancestor_record_offset":82764,"inlinee":33766,"source_line":18658,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::compute_passes::bin_lights::bin_lights::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/bin_lights.rs
//     evidence={"module":544,"record_offset":84896,"parent_record_offset":84824,"ancestor_record_offset":82764,"inlinee":33760,"source_line":32,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":544,"record_offset":83644,"parent_record_offset":83620,"ancestor_record_offset":82764,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":544,"record_offset":83380,"parent_record_offset":83352,"ancestor_record_offset":82764,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":544,"record_offset":83412,"parent_record_offset":83380,"ancestor_record_offset":82764,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":544,"record_offset":83352,"parent_record_offset":83132,"ancestor_record_offset":82764,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":544,"record_offset":83212,"parent_record_offset":83132,"ancestor_record_offset":82764,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":544,"record_offset":83772,"parent_record_offset":83132,"ancestor_record_offset":82764,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":544,"record_offset":83296,"parent_record_offset":83132,"ancestor_record_offset":82764,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":544,"record_offset":83056,"parent_record_offset":83020,"ancestor_record_offset":82764,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":544,"record_offset":83620,"parent_record_offset":83512,"ancestor_record_offset":82764,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":544,"record_offset":83560,"parent_record_offset":83536,"ancestor_record_offset":82764,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":544,"record_offset":83864,"parent_record_offset":83132,"ancestor_record_offset":82764,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":544,"record_offset":83536,"parent_record_offset":83512,"ancestor_record_offset":82764,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":544,"record_offset":83108,"parent_record_offset":83084,"ancestor_record_offset":82764,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":544,"record_offset":83084,"parent_record_offset":83056,"ancestor_record_offset":82764,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":544,"record_offset":83132,"parent_record_offset":83108,"ancestor_record_offset":82764,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":544,"record_offset":83184,"parent_record_offset":83132,"ancestor_record_offset":82764,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":544,"record_offset":83512,"parent_record_offset":83132,"ancestor_record_offset":82764,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":544,"record_offset":83744,"parent_record_offset":83132,"ancestor_record_offset":82764,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":544,"record_offset":85040,"parent_record_offset":84896,"ancestor_record_offset":82764,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::impl$6::create
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":544,"record_offset":85008,"parent_record_offset":84896,"ancestor_record_offset":82764,"inlinee":12324,"source_line":1280,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":544,"record_offset":85108,"parent_record_offset":84896,"ancestor_record_offset":82764,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::bin_lights::bin_lights::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x83E0
// 首条位置: {"module":544,"inlinee":33760}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":544,"record_offset":85040,"parent_record_offset":84896,"ancestor_record_offset":84896,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::impl$6::create
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":544,"record_offset":85008,"parent_record_offset":84896,"ancestor_record_offset":84896,"inlinee":12324,"source_line":1280,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":544,"record_offset":85108,"parent_record_offset":84896,"ancestor_record_offset":84896,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::light_binning::bin_lights_to_cells_0::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":544,"record_offset":85076,"parent_record_offset":85040,"ancestor_record_offset":84896,"inlinee":33762,"source_line":18359,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::light_binning::bin_lights_to_cells_1::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":544,"record_offset":85524,"parent_record_offset":85488,"ancestor_record_offset":84896,"inlinee":33768,"source_line":18409,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::light_binning::cell_prefix_merge::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":544,"record_offset":85396,"parent_record_offset":85360,"ancestor_record_offset":84896,"inlinee":30678,"source_line":18444,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::light_binning::cell_prefix_sum::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":544,"record_offset":85204,"parent_record_offset":85168,"ancestor_record_offset":84896,"inlinee":33764,"source_line":18479,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::light_binning::cell_subtotal_prefix_sum::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":544,"record_offset":85300,"parent_record_offset":85264,"ancestor_record_offset":84896,"inlinee":33766,"source_line":18658,"archive":"_pdb/C/modules/00544.jsonl.gz"}
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
// 完整结构及原始类型记录: bin_lights.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: bin_lights.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/bin_lights.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: bin_lights.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::bin_lights::bin_lights
// [PDB Shader 类型/入口] {"shader_family":"country_core/light_binning/bin_lights_to_cells_0","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::light_binning::bin_lights_to_cells_0::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":544,"record_offset":85076,"parent_record_offset":85040,"ancestor_record_offset":82764,"inlinee":33762,"source_line":18359,"archive":"_pdb/C/modules/00544.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/light_binning/cell_prefix_sum","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::light_binning::cell_prefix_sum::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":544,"record_offset":85204,"parent_record_offset":85168,"ancestor_record_offset":82764,"inlinee":33764,"source_line":18479,"archive":"_pdb/C/modules/00544.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/light_binning/cell_subtotal_prefix_sum","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::light_binning::cell_subtotal_prefix_sum::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":544,"record_offset":85300,"parent_record_offset":85264,"ancestor_record_offset":82764,"inlinee":33766,"source_line":18658,"archive":"_pdb/C/modules/00544.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/light_binning/cell_prefix_merge","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::light_binning::cell_prefix_merge::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":544,"record_offset":85396,"parent_record_offset":85360,"ancestor_record_offset":82764,"inlinee":30678,"source_line":18444,"archive":"_pdb/C/modules/00544.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/light_binning/bin_lights_to_cells_1","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::light_binning::bin_lights_to_cells_1::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":544,"record_offset":85524,"parent_record_offset":85488,"ancestor_record_offset":82764,"inlinee":33768,"source_line":18409,"archive":"_pdb/C/modules/00544.jsonl.gz"}}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::bin_lights::bin_lights::closure$0
// [PDB Shader 类型/入口] {"shader_family":"country_core/light_binning/bin_lights_to_cells_0","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::light_binning::bin_lights_to_cells_0::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":544,"record_offset":85076,"parent_record_offset":85040,"ancestor_record_offset":84896,"inlinee":33762,"source_line":18359,"archive":"_pdb/C/modules/00544.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/light_binning/cell_prefix_sum","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::light_binning::cell_prefix_sum::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":544,"record_offset":85204,"parent_record_offset":85168,"ancestor_record_offset":84896,"inlinee":33764,"source_line":18479,"archive":"_pdb/C/modules/00544.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/light_binning/cell_subtotal_prefix_sum","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::light_binning::cell_subtotal_prefix_sum::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":544,"record_offset":85300,"parent_record_offset":85264,"ancestor_record_offset":84896,"inlinee":33766,"source_line":18658,"archive":"_pdb/C/modules/00544.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/light_binning/cell_prefix_merge","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::light_binning::cell_prefix_merge::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":544,"record_offset":85396,"parent_record_offset":85360,"ancestor_record_offset":84896,"inlinee":30678,"source_line":18444,"archive":"_pdb/C/modules/00544.jsonl.gz"}}
// [PDB Shader 类型/入口] {"shader_family":"country_core/light_binning/bin_lights_to_cells_1","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::light_binning::bin_lights_to_cells_1::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":544,"record_offset":85524,"parent_record_offset":85488,"ancestor_record_offset":84896,"inlinee":33768,"source_line":18409,"archive":"_pdb/C/modules/00544.jsonl.gz"}}
// [Shader 映射] {"shader_id":"d0c08caa29950204817692e3","relative_path":"country-core/compiled/light_binning/bin_lights_to_cells_0.precomp","sha256":"6b2a352969b41d8a5fdf99158859a6ce8f36362efff6b9f30bf9245280bf67af","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::bin_lights::bin_lights","hint":{"shader_family":"country_core/light_binning/bin_lights_to_cells_0","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::light_binning::bin_lights_to_cells_0::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":544,"record_offset":85076,"parent_record_offset":85040,"ancestor_record_offset":82764,"inlinee":33762,"source_line":18359,"archive":"_pdb/C/modules/00544.jsonl.gz"}},"shader_uri":"country_core/light_binning/bin_lights_to_cells_0:main"}
// [Shader 映射] {"shader_id":"58e04a6c8344508184e9b645","relative_path":"country-core/compiled/light_binning/cell_prefix_sum.precomp","sha256":"21e487e61ff5a0922953042d65ce6578ce4cf7d72722a39dd91781541896a4f2","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::bin_lights::bin_lights","hint":{"shader_family":"country_core/light_binning/cell_prefix_sum","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::light_binning::cell_prefix_sum::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":544,"record_offset":85204,"parent_record_offset":85168,"ancestor_record_offset":82764,"inlinee":33764,"source_line":18479,"archive":"_pdb/C/modules/00544.jsonl.gz"}},"shader_uri":"country_core/light_binning/cell_prefix_sum:main"}
// [Shader 映射] {"shader_id":"870fcb2ba63600c2f03efdd5","relative_path":"country-core/compiled/light_binning/cell_subtotal_prefix_sum.precomp","sha256":"f1084e577acd7fe0a4fa39dd951cca0e69ec8f9ad87c135e325e3f11e162fe5e","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::bin_lights::bin_lights","hint":{"shader_family":"country_core/light_binning/cell_subtotal_prefix_sum","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::light_binning::cell_subtotal_prefix_sum::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":544,"record_offset":85300,"parent_record_offset":85264,"ancestor_record_offset":82764,"inlinee":33766,"source_line":18658,"archive":"_pdb/C/modules/00544.jsonl.gz"}},"shader_uri":"country_core/light_binning/cell_subtotal_prefix_sum:main"}
// [Shader 映射] {"shader_id":"f29d80b75b4756f728e8abbb","relative_path":"country-core/compiled/light_binning/cell_prefix_merge.precomp","sha256":"42c7b47b1baf6d3522528d580d9dc6c6c29d5da7ef79c57892291af227359b45","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::bin_lights::bin_lights","hint":{"shader_family":"country_core/light_binning/cell_prefix_merge","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::light_binning::cell_prefix_merge::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":544,"record_offset":85396,"parent_record_offset":85360,"ancestor_record_offset":82764,"inlinee":30678,"source_line":18444,"archive":"_pdb/C/modules/00544.jsonl.gz"}},"shader_uri":"country_core/light_binning/cell_prefix_merge:main"}
// [Shader 映射] {"shader_id":"cd937fb1343344eed98c5ecb","relative_path":"country-core/compiled/light_binning/bin_lights_to_cells_1.precomp","sha256":"55e6abf436f191e79c8c50255cdb7ce0958f1df433d5b7805094884a12b2d9a8","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::bin_lights::bin_lights","hint":{"shader_family":"country_core/light_binning/bin_lights_to_cells_1","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::light_binning::bin_lights_to_cells_1::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":544,"record_offset":85524,"parent_record_offset":85488,"ancestor_record_offset":82764,"inlinee":33768,"source_line":18409,"archive":"_pdb/C/modules/00544.jsonl.gz"}},"shader_uri":"country_core/light_binning/bin_lights_to_cells_1:main"}
// [Shader 映射] {"shader_id":"d0c08caa29950204817692e3","relative_path":"country-core/compiled/light_binning/bin_lights_to_cells_0.precomp","sha256":"6b2a352969b41d8a5fdf99158859a6ce8f36362efff6b9f30bf9245280bf67af","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::bin_lights::bin_lights::closure$0","hint":{"shader_family":"country_core/light_binning/bin_lights_to_cells_0","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::light_binning::bin_lights_to_cells_0::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":544,"record_offset":85076,"parent_record_offset":85040,"ancestor_record_offset":84896,"inlinee":33762,"source_line":18359,"archive":"_pdb/C/modules/00544.jsonl.gz"}},"shader_uri":"country_core/light_binning/bin_lights_to_cells_0:main"}
// [Shader 映射] {"shader_id":"58e04a6c8344508184e9b645","relative_path":"country-core/compiled/light_binning/cell_prefix_sum.precomp","sha256":"21e487e61ff5a0922953042d65ce6578ce4cf7d72722a39dd91781541896a4f2","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::bin_lights::bin_lights::closure$0","hint":{"shader_family":"country_core/light_binning/cell_prefix_sum","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::light_binning::cell_prefix_sum::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":544,"record_offset":85204,"parent_record_offset":85168,"ancestor_record_offset":84896,"inlinee":33764,"source_line":18479,"archive":"_pdb/C/modules/00544.jsonl.gz"}},"shader_uri":"country_core/light_binning/cell_prefix_sum:main"}
// [Shader 映射] {"shader_id":"870fcb2ba63600c2f03efdd5","relative_path":"country-core/compiled/light_binning/cell_subtotal_prefix_sum.precomp","sha256":"f1084e577acd7fe0a4fa39dd951cca0e69ec8f9ad87c135e325e3f11e162fe5e","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::bin_lights::bin_lights::closure$0","hint":{"shader_family":"country_core/light_binning/cell_subtotal_prefix_sum","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::light_binning::cell_subtotal_prefix_sum::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":544,"record_offset":85300,"parent_record_offset":85264,"ancestor_record_offset":84896,"inlinee":33766,"source_line":18658,"archive":"_pdb/C/modules/00544.jsonl.gz"}},"shader_uri":"country_core/light_binning/cell_subtotal_prefix_sum:main"}
// [Shader 映射] {"shader_id":"f29d80b75b4756f728e8abbb","relative_path":"country-core/compiled/light_binning/cell_prefix_merge.precomp","sha256":"42c7b47b1baf6d3522528d580d9dc6c6c29d5da7ef79c57892291af227359b45","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::bin_lights::bin_lights::closure$0","hint":{"shader_family":"country_core/light_binning/cell_prefix_merge","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::light_binning::cell_prefix_merge::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":544,"record_offset":85396,"parent_record_offset":85360,"ancestor_record_offset":84896,"inlinee":30678,"source_line":18444,"archive":"_pdb/C/modules/00544.jsonl.gz"}},"shader_uri":"country_core/light_binning/cell_prefix_merge:main"}
// [Shader 映射] {"shader_id":"cd937fb1343344eed98c5ecb","relative_path":"country-core/compiled/light_binning/bin_lights_to_cells_1.precomp","sha256":"55e6abf436f191e79c8c50255cdb7ce0958f1df433d5b7805094884a12b2d9a8","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::bin_lights::bin_lights::closure$0","hint":{"shader_family":"country_core/light_binning/bin_lights_to_cells_1","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::light_binning::bin_lights_to_cells_1::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":544,"record_offset":85524,"parent_record_offset":85488,"ancestor_record_offset":84896,"inlinee":33768,"source_line":18409,"archive":"_pdb/C/modules/00544.jsonl.gz"}},"shader_uri":"country_core/light_binning/bin_lights_to_cells_1:main"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/light_binning/cell_prefix_sum.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[30,14]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[512,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":65,"name":"inout_buf","storage_class":"Uniform","type":{"id":64,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,62]},"decorations":[],"pointee":{"id":62,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[63]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":63,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":291,"name":"subtotals_buf","storage_class":"Uniform","type":{"id":290,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,289]},"decorations":[],"pointee":{"id":289,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[63]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":63,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":14,"name":"unknown","storage_class":"Input","type":{"id":13,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,11]},"decorations":[],"pointee":{"id":11,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[6,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":26,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[26]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":30,"name":"unknown","storage_class":"Input","type":{"id":13,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,11]},"decorations":[],"pointee":{"id":11,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[6,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":27,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[27]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/light_binning/cell_subtotal_prefix_sum.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[36]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[256,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":11,"name":"entryPointParams","storage_class":"PushConstant","type":{"id":10,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[9,9]},"decorations":[],"pointee":{"id":9,"name":"EntryPointParams_natural","record":{"opcode":"OpTypeStruct","operands":[6]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"range_end","type_id":6,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":"unknown","declared_direction":"push_constant_input","decorations":[],"access_qualifiers":["Block"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":83,"name":"inout_buf","storage_class":"Uniform","type":{"id":82,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,80]},"decorations":[],"pointee":{"id":80,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[81]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":81,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":36,"name":"unknown","storage_class":"Input","type":{"id":35,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,33]},"decorations":[],"pointee":{"id":33,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[6,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":27,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[27]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/light_binning/bin_lights_to_cells_1.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[58]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[64,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":62,"name":"globalParams","storage_class":"Uniform","type":{"id":61,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,60]},"decorations":[],"pointee":{"id":60,"name":"GlobalParams_natural","record":{"opcode":"OpTypeStruct","operands":[8]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"lantern_count","type_id":8,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":80,"name":"lantern_buf","storage_class":"Uniform","type":{"id":79,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,77]},"decorations":[],"pointee":{"id":77,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[78]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":78,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":212,"name":"frame_counter","storage_class":"Uniform","type":{"id":211,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,210]},"decorations":[],"pointee":{"id":210,"name":"FrameCounter_natural","record":{"opcode":"OpTypeStruct","operands":[8]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"index","type_id":8,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":2,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":446,"name":"indirection_grid","storage_class":"Uniform","type":{"id":445,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,443]},"decorations":[],"pointee":{"id":443,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[444]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":444,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":465,"name":"light_indices","storage_class":"Uniform","type":{"id":445,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,443]},"decorations":[],"pointee":{"id":443,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[444]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":444,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":3,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[3]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":58,"name":"unknown","storage_class":"Input","type":{"id":57,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,55]},"decorations":[],"pointee":{"id":55,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[8,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/light_binning/bin_lights_to_cells_0.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[58]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[64,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":62,"name":"globalParams","storage_class":"Uniform","type":{"id":61,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,60]},"decorations":[],"pointee":{"id":60,"name":"GlobalParams_natural","record":{"opcode":"OpTypeStruct","operands":[8]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"lantern_count","type_id":8,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":80,"name":"lantern_buf","storage_class":"Uniform","type":{"id":79,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,77]},"decorations":[],"pointee":{"id":77,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[78]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":78,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":212,"name":"frame_counter","storage_class":"Uniform","type":{"id":211,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,210]},"decorations":[],"pointee":{"id":210,"name":"FrameCounter_natural","record":{"opcode":"OpTypeStruct","operands":[8]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"index","type_id":8,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":2,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":441,"name":"indirection_grid","storage_class":"Uniform","type":{"id":440,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,438]},"decorations":[],"pointee":{"id":438,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[439]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":439,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":58,"name":"unknown","storage_class":"Input","type":{"id":57,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,55]},"decorations":[],"pointee":{"id":55,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[8,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/light_binning/cell_prefix_merge.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[9]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[64,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":22,"name":"inout_buf","storage_class":"Uniform","type":{"id":21,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,19]},"decorations":[],"pointee":{"id":19,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[20]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":20,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":27,"name":"subtotals_buf","storage_class":"Uniform","type":{"id":26,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,25]},"decorations":[],"pointee":{"id":25,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[20]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":20,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"unknown","storage_class":"Input","type":{"id":8,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,6]},"decorations":[],"pointee":{"id":6,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[5,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":5678848,"caller_aliases":{"alias_group_rva":5678848,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5687683,"position_among_mapped_render_targets":59,"mapped_render_target_count":74,"previous_callsite":5687564,"next_callsite":5687741,"previous_target_files":["rhapsody2/render-std/rhapsody-gi/src/lib.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/restir_di.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: bin_lights.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::bin_lights::bin_lights
// 信息 ID: 3db8cb1b987e4db9ce019fc1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56c983 -> 0xa8d230 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::bin_lights::bin_lights::closure$0
// 信息 ID: f09c8d32e0ff6fe6bc5e51e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::compute_passes::bin_lights::bin_lights", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::bin_lights::bin_lights; module=544, inline_record=84896; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
