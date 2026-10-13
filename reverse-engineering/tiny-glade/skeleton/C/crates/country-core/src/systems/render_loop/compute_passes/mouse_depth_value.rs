// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/render_loop/compute_passes/mouse_depth_value.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: e4b9c2226e86a2da5baaaf86bd2dc49276c92ed4833d4fb302cb49b471f9cf4e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mouse_depth_value.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 564
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
// 函数符号: country_core::systems::render_loop::compute_passes::mouse_depth_value::mouse_depth_compute
// 记录形式: compiled_procedure; instances=1
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":564,"record_offset":776,"rva":11670928,"va":5380380048}
// 编译实例 RVAs: 0xb21590
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14153fa90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mouse_depth_value.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10609604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380380048,"raw_hex":"e8c4e3a100","size":5,"soft_fail":false,"target_va":5390989968,"va":5380380359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14154a4d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mouse_depth_value.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10652271","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380380048,"raw_hex":"e86f8aa200","size":5,"soft_fail":false,"target_va":5391033552,"va":5380381276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141770e40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mouse_depth_value.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12907608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380380048,"raw_hex":"e858f4c400","size":5,"soft_fail":false,"target_va":5393288768,"va":5380381155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417902b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mouse_depth_value.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13035641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380380048,"raw_hex":"e879e8c600","size":5,"soft_fail":false,"target_va":5393416880,"va":5380381234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t36240828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380380048,"raw_hex":"e8bcfd2802","size":5,"soft_fail":false,"target_va":5416621600,"va":5380380767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3155402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380380048,"raw_hex":"e8ca253000","size":5,"soft_fail":false,"target_va":5383536672,"va":5380381265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3157959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380380048,"raw_hex":"e8c72f3000","size":5,"soft_fail":false,"target_va":5383539152,"va":5380381188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::WithRg::run
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":564,"record_offset":2660,"parent_record_offset":776,"ancestor_record_offset":776,"inlinee":12358,"source_line":489,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":564,"record_offset":956,"parent_record_offset":776,"ancestor_record_offset":776,"inlinee":12326,"source_line":34,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":564,"record_offset":2304,"parent_record_offset":2280,"ancestor_record_offset":776,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":564,"record_offset":2352,"parent_record_offset":2328,"ancestor_record_offset":776,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":564,"record_offset":2328,"parent_record_offset":2304,"ancestor_record_offset":776,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":564,"record_offset":2280,"parent_record_offset":2256,"ancestor_record_offset":776,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":564,"record_offset":2256,"parent_record_offset":2216,"ancestor_record_offset":776,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":564,"record_offset":2216,"parent_record_offset":2192,"ancestor_record_offset":776,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":564,"record_offset":1012,"parent_record_offset":984,"ancestor_record_offset":776,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":564,"record_offset":2576,"parent_record_offset":2552,"ancestor_record_offset":776,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":564,"record_offset":2552,"parent_record_offset":2528,"ancestor_record_offset":776,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":564,"record_offset":2500,"parent_record_offset":2468,"ancestor_record_offset":776,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":564,"record_offset":2444,"parent_record_offset":1936,"ancestor_record_offset":776,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":564,"record_offset":2468,"parent_record_offset":2444,"ancestor_record_offset":776,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":564,"record_offset":1964,"parent_record_offset":1936,"ancestor_record_offset":776,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":564,"record_offset":2528,"parent_record_offset":2468,"ancestor_record_offset":776,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":564,"record_offset":2012,"parent_record_offset":1988,"ancestor_record_offset":776,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":564,"record_offset":1988,"parent_record_offset":1964,"ancestor_record_offset":776,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":564,"record_offset":2192,"parent_record_offset":2168,"ancestor_record_offset":776,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":564,"record_offset":984,"parent_record_offset":956,"ancestor_record_offset":776,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":564,"record_offset":1936,"parent_record_offset":984,"ancestor_record_offset":776,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":564,"record_offset":1036,"parent_record_offset":1012,"ancestor_record_offset":776,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":564,"record_offset":1684,"parent_record_offset":1660,"ancestor_record_offset":776,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":564,"record_offset":1712,"parent_record_offset":1684,"ancestor_record_offset":776,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":564,"record_offset":1252,"parent_record_offset":1228,"ancestor_record_offset":776,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":564,"record_offset":1336,"parent_record_offset":1312,"ancestor_record_offset":776,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":564,"record_offset":7584,"parent_record_offset":2732,"ancestor_record_offset":776,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":564,"record_offset":1484,"parent_record_offset":1456,"ancestor_record_offset":776,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":564,"record_offset":1456,"parent_record_offset":1428,"ancestor_record_offset":776,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":564,"record_offset":2104,"parent_record_offset":2040,"ancestor_record_offset":776,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":564,"record_offset":2128,"parent_record_offset":2104,"ancestor_record_offset":776,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":564,"record_offset":2168,"parent_record_offset":1988,"ancestor_record_offset":776,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":564,"record_offset":2628,"parent_record_offset":956,"ancestor_record_offset":776,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":564,"record_offset":1276,"parent_record_offset":1252,"ancestor_record_offset":776,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":564,"record_offset":1600,"parent_record_offset":1576,"ancestor_record_offset":776,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":564,"record_offset":2040,"parent_record_offset":2012,"ancestor_record_offset":776,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":564,"record_offset":2076,"parent_record_offset":2040,"ancestor_record_offset":776,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":564,"record_offset":2396,"parent_record_offset":2216,"ancestor_record_offset":776,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$11::fetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":564,"record_offset":2704,"parent_record_offset":2660,"ancestor_record_offset":776,"inlinee":30203,"source_line":568,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$11::unfetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":564,"record_offset":7752,"parent_record_offset":2660,"ancestor_record_offset":776,"inlinee":30215,"source_line":575,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::window::WindowSize::as_vec2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/window.rs
//     evidence={"module":564,"record_offset":7692,"parent_record_offset":2732,"ancestor_record_offset":776,"inlinee":32182,"source_line":46,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::read_mouse_depth_value::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":564,"record_offset":7660,"parent_record_offset":7616,"ancestor_record_offset":776,"inlinee":34622,"source_line":31707,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::compute_passes::mouse_depth_value::mouse_depth_compute::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/mouse_depth_value.rs
//     evidence={"module":564,"record_offset":2732,"parent_record_offset":2660,"ancestor_record_offset":776,"inlinee":35451,"source_line":19,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":564,"record_offset":2808,"parent_record_offset":2732,"ancestor_record_offset":776,"inlinee":12326,"source_line":34,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":564,"record_offset":1660,"parent_record_offset":1636,"ancestor_record_offset":776,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":564,"record_offset":1396,"parent_record_offset":1368,"ancestor_record_offset":776,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":564,"record_offset":1428,"parent_record_offset":1396,"ancestor_record_offset":776,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":564,"record_offset":1368,"parent_record_offset":1148,"ancestor_record_offset":776,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":564,"record_offset":1228,"parent_record_offset":1148,"ancestor_record_offset":776,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":564,"record_offset":1788,"parent_record_offset":1148,"ancestor_record_offset":776,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":564,"record_offset":1312,"parent_record_offset":1148,"ancestor_record_offset":776,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":564,"record_offset":1072,"parent_record_offset":1036,"ancestor_record_offset":776,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":564,"record_offset":1636,"parent_record_offset":1528,"ancestor_record_offset":776,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":564,"record_offset":1576,"parent_record_offset":1552,"ancestor_record_offset":776,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":564,"record_offset":1880,"parent_record_offset":1148,"ancestor_record_offset":776,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":564,"record_offset":1552,"parent_record_offset":1528,"ancestor_record_offset":776,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":564,"record_offset":1124,"parent_record_offset":1100,"ancestor_record_offset":776,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":564,"record_offset":1100,"parent_record_offset":1072,"ancestor_record_offset":776,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":564,"record_offset":1148,"parent_record_offset":1124,"ancestor_record_offset":776,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":564,"record_offset":1200,"parent_record_offset":1148,"ancestor_record_offset":776,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":564,"record_offset":1528,"parent_record_offset":1148,"ancestor_record_offset":776,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":564,"record_offset":1760,"parent_record_offset":1148,"ancestor_record_offset":776,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":564,"record_offset":5744,"parent_record_offset":5720,"ancestor_record_offset":776,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":564,"record_offset":7616,"parent_record_offset":2732,"ancestor_record_offset":776,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":564,"record_offset":7720,"parent_record_offset":2732,"ancestor_record_offset":776,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::mouse_depth_value::mouse_depth_compute::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8A7B
// 首条位置: {"module":564,"inlinee":35451}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":564,"record_offset":7584,"parent_record_offset":2732,"ancestor_record_offset":2732,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::window::WindowSize::as_vec2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/window.rs
//     evidence={"module":564,"record_offset":7692,"parent_record_offset":2732,"ancestor_record_offset":2732,"inlinee":32182,"source_line":46,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":564,"record_offset":2808,"parent_record_offset":2732,"ancestor_record_offset":2732,"inlinee":12326,"source_line":34,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":564,"record_offset":7616,"parent_record_offset":2732,"ancestor_record_offset":2732,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":564,"record_offset":7720,"parent_record_offset":2732,"ancestor_record_offset":2732,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":564,"record_offset":4160,"parent_record_offset":4136,"ancestor_record_offset":2732,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":564,"record_offset":4208,"parent_record_offset":4184,"ancestor_record_offset":2732,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":564,"record_offset":4184,"parent_record_offset":4160,"ancestor_record_offset":2732,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":564,"record_offset":4136,"parent_record_offset":4112,"ancestor_record_offset":2732,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":564,"record_offset":4112,"parent_record_offset":4072,"ancestor_record_offset":2732,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":564,"record_offset":4072,"parent_record_offset":4048,"ancestor_record_offset":2732,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":564,"record_offset":2868,"parent_record_offset":2840,"ancestor_record_offset":2732,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":564,"record_offset":4432,"parent_record_offset":4408,"ancestor_record_offset":2732,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":564,"record_offset":4408,"parent_record_offset":4384,"ancestor_record_offset":2732,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":564,"record_offset":4356,"parent_record_offset":4324,"ancestor_record_offset":2732,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":564,"record_offset":4300,"parent_record_offset":3792,"ancestor_record_offset":2732,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":564,"record_offset":4324,"parent_record_offset":4300,"ancestor_record_offset":2732,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":564,"record_offset":3820,"parent_record_offset":3792,"ancestor_record_offset":2732,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":564,"record_offset":4384,"parent_record_offset":4324,"ancestor_record_offset":2732,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":564,"record_offset":3868,"parent_record_offset":3844,"ancestor_record_offset":2732,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":564,"record_offset":3844,"parent_record_offset":3820,"ancestor_record_offset":2732,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":564,"record_offset":4048,"parent_record_offset":4024,"ancestor_record_offset":2732,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":564,"record_offset":2840,"parent_record_offset":2808,"ancestor_record_offset":2732,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":564,"record_offset":3792,"parent_record_offset":2840,"ancestor_record_offset":2732,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":564,"record_offset":2892,"parent_record_offset":2868,"ancestor_record_offset":2732,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":564,"record_offset":3540,"parent_record_offset":3516,"ancestor_record_offset":2732,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":564,"record_offset":3568,"parent_record_offset":3540,"ancestor_record_offset":2732,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":564,"record_offset":3108,"parent_record_offset":3084,"ancestor_record_offset":2732,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":564,"record_offset":3192,"parent_record_offset":3168,"ancestor_record_offset":2732,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":564,"record_offset":3340,"parent_record_offset":3312,"ancestor_record_offset":2732,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":564,"record_offset":3312,"parent_record_offset":3284,"ancestor_record_offset":2732,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":564,"record_offset":3960,"parent_record_offset":3896,"ancestor_record_offset":2732,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":564,"record_offset":3984,"parent_record_offset":3960,"ancestor_record_offset":2732,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":564,"record_offset":4024,"parent_record_offset":3844,"ancestor_record_offset":2732,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":564,"record_offset":4484,"parent_record_offset":2808,"ancestor_record_offset":2732,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":564,"record_offset":3132,"parent_record_offset":3108,"ancestor_record_offset":2732,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":564,"record_offset":3456,"parent_record_offset":3432,"ancestor_record_offset":2732,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":564,"record_offset":3896,"parent_record_offset":3868,"ancestor_record_offset":2732,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":564,"record_offset":3932,"parent_record_offset":3896,"ancestor_record_offset":2732,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":564,"record_offset":4252,"parent_record_offset":4072,"ancestor_record_offset":2732,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::read_mouse_depth_value::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":564,"record_offset":7660,"parent_record_offset":7616,"ancestor_record_offset":2732,"inlinee":34622,"source_line":31707,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":564,"record_offset":3516,"parent_record_offset":3492,"ancestor_record_offset":2732,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":564,"record_offset":3252,"parent_record_offset":3224,"ancestor_record_offset":2732,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":564,"record_offset":3284,"parent_record_offset":3252,"ancestor_record_offset":2732,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":564,"record_offset":3224,"parent_record_offset":3004,"ancestor_record_offset":2732,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":564,"record_offset":3084,"parent_record_offset":3004,"ancestor_record_offset":2732,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":564,"record_offset":3644,"parent_record_offset":3004,"ancestor_record_offset":2732,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":564,"record_offset":3168,"parent_record_offset":3004,"ancestor_record_offset":2732,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":564,"record_offset":2928,"parent_record_offset":2892,"ancestor_record_offset":2732,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":564,"record_offset":3492,"parent_record_offset":3384,"ancestor_record_offset":2732,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":564,"record_offset":3432,"parent_record_offset":3408,"ancestor_record_offset":2732,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":564,"record_offset":3736,"parent_record_offset":3004,"ancestor_record_offset":2732,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":564,"record_offset":3408,"parent_record_offset":3384,"ancestor_record_offset":2732,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":564,"record_offset":2980,"parent_record_offset":2956,"ancestor_record_offset":2732,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":564,"record_offset":2956,"parent_record_offset":2928,"ancestor_record_offset":2732,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":564,"record_offset":3004,"parent_record_offset":2980,"ancestor_record_offset":2732,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":564,"record_offset":3056,"parent_record_offset":3004,"ancestor_record_offset":2732,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":564,"record_offset":3384,"parent_record_offset":3004,"ancestor_record_offset":2732,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":564,"record_offset":3616,"parent_record_offset":3004,"ancestor_record_offset":2732,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":564,"record_offset":5744,"parent_record_offset":5720,"ancestor_record_offset":2732,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00564.jsonl.gz"}
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
// 完整结构及原始类型记录: mouse_depth_value.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mouse_depth_value.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/mouse_depth_value.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: mouse_depth_value.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::mouse_depth_value::mouse_depth_compute
// [PDB Shader 类型/入口] {"shader_family":"country_core/read_mouse_depth_value","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::read_mouse_depth_value::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":564,"record_offset":7660,"parent_record_offset":7616,"ancestor_record_offset":776,"inlinee":34622,"source_line":31707,"archive":"_pdb/C/modules/00564.jsonl.gz"}}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::mouse_depth_value::mouse_depth_compute::closure$0
// [PDB Shader 类型/入口] {"shader_family":"country_core/read_mouse_depth_value","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::read_mouse_depth_value::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":564,"record_offset":7660,"parent_record_offset":7616,"ancestor_record_offset":2732,"inlinee":34622,"source_line":31707,"archive":"_pdb/C/modules/00564.jsonl.gz"}}
// [Shader 映射] {"shader_id":"c579a5d213193bcf16d0f063","relative_path":"country-core/compiled/read_mouse_depth_value.precomp","sha256":"25f38d2d6db2507007de6d11eca5ea8209dad4ae81b2b536b430c23ddbc06e42","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::mouse_depth_value::mouse_depth_compute","hint":{"shader_family":"country_core/read_mouse_depth_value","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::read_mouse_depth_value::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":564,"record_offset":7660,"parent_record_offset":7616,"ancestor_record_offset":776,"inlinee":34622,"source_line":31707,"archive":"_pdb/C/modules/00564.jsonl.gz"}},"shader_uri":"country_core/read_mouse_depth_value:main"}
// [Shader 映射] {"shader_id":"c579a5d213193bcf16d0f063","relative_path":"country-core/compiled/read_mouse_depth_value.precomp","sha256":"25f38d2d6db2507007de6d11eca5ea8209dad4ae81b2b536b430c23ddbc06e42","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::mouse_depth_value::mouse_depth_compute::closure$0","hint":{"shader_family":"country_core/read_mouse_depth_value","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::read_mouse_depth_value::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":564,"record_offset":7660,"parent_record_offset":7616,"ancestor_record_offset":2732,"inlinee":34622,"source_line":31707,"archive":"_pdb/C/modules/00564.jsonl.gz"}},"shader_uri":"country_core/read_mouse_depth_value:main"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/read_mouse_depth_value.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[217,180]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[16,16,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":15,"name":"globalParams","storage_class":"Uniform","type":{"id":14,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,9]},"decorations":[],"pointee":{"id":9,"name":"GlobalParams_natural","record":{"opcode":"OpTypeStruct","operands":[11,13,13,12]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"depth_map_dims","type_id":11,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"mouse_position_screenspace","type_id":13,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"window_size","type_id":13,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"filter_box_y_frac","type_id":12,"decorations":[{"decoration":"Offset","operands":[24]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":54,"name":"depth_map","storage_class":"UniformConstant","type":{"id":53,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,51]},"decorations":[],"pointee":{"id":51,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[12,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":118,"name":"current_view","storage_class":"Uniform","type":{"id":117,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,115]},"decorations":[],"pointee":{"id":115,"name":"ViewConstants_natural","record":{"opcode":"OpTypeStruct","operands":[116,116,116,116,116,116,116,116,116,116,116,116,116,13,13,13,13,116,116,33,12,12,12,12]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"view_to_clip","type_id":116,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":1,"name":"clip_to_view","type_id":116,"decorations":[{"decoration":"Offset","operands":[64]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":2,"name":"view_to_sample","type_id":116,"decorations":[{"decoration":"Offset","operands":[128]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":3,"name":"sample_to_view","type_id":116,"decorations":[{"decoration":"Offset","operands":[192]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":4,"name":"world_to_view","type_id":116,"decorations":[{"decoration":"Offset","operands":[256]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":5,"name":"view_to_world","type_id":116,"decorations":[{"decoration":"Offset","operands":[320]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":6,"name":"world_to_sample","type_id":116,"decorations":[{"decoration":"Offset","operands":[384]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":7,"name":"clip_to_prev_clip","type_id":116,"decorations":[{"decoration":"Offset","operands":[448]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":8,"name":"prev_clip_to_clip","type_id":116,"decorations":[{"decoration":"Offset","operands":[512]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":9,"name":"prev_view_to_prev_clip","type_id":116,"decorations":[{"decoration":"Offset","operands":[576]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":10,"name":"prev_clip_to_prev_view","type_id":116,"decorations":[{"decoration":"Offset","operands":[640]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":11,"name":"prev_world_to_prev_view","type_id":116,"decorations":[{"decoration":"Offset","operands":[704]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":12,"name":"prev_view_to_prev_world","type_id":116,"decorations":[{"decoration":"Offset","operands":[768]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":13,"name":"sample_offset_pixels","type_id":13,"decorations":[{"decoration":"Offset","operands":[832]}]},{"index":14,"name":"sample_offset_clip","type_id":13,"decorations":[{"decoration":"Offset","operands":[840]}]},{"index":15,"name":"prev_sample_offset_pixels","type_id":13,"decorations":[{"decoration":"Offset","operands":[848]}]},{"index":16,"name":"prev_sample_offset_clip","type_id":13,"decorations":[{"decoration":"Offset","operands":[856]}]},{"index":17,"name":"pad0","type_id":116,"decorations":[{"decoration":"Offset","operands":[864]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":18,"name":"pad1","type_id":116,"decorations":[{"decoration":"Offset","operands":[928]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":19,"name":"pad2","type_id":33,"decorations":[{"decoration":"Offset","operands":[992]}]},{"index":20,"name":"pad3","type_id":12,"decorations":[{"decoration":"Offset","operands":[1008]}]},{"index":21,"name":"pad4","type_id":12,"decorations":[{"decoration":"Offset","operands":[1012]}]},{"index":22,"name":"pad5","type_id":12,"decorations":[{"decoration":"Offset","operands":[1016]}]},{"index":23,"name":"mip_bias","type_id":12,"decorations":[{"decoration":"Offset","operands":[1020]}]}]}},"descriptor_set":4,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[4]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":190,"name":"out_buffer","storage_class":"Uniform","type":{"id":189,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,187]},"decorations":[],"pointee":{"id":187,"name":"RWStructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[188]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":188,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":180,"name":"unknown","storage_class":"Input","type":{"id":179,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,10]},"decorations":[],"pointee":{"id":10,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":29,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[29]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":217,"name":"unknown","storage_class":"Input","type":{"id":216,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,214]},"decorations":[],"pointee":{"id":214,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[10,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":27,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[27]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":5678848,"caller_aliases":{"alias_group_rva":5678848,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5686328,"position_among_mapped_render_targets":50,"mapped_render_target_count":74,"previous_callsite":5686299,"next_callsite":5686383,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/dof_autofocus.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/main_pass.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mouse_depth_value.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::mouse_depth_value::mouse_depth_compute
// 信息 ID: f42240b4f4a78c0cfc9c157e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56c438 -> 0xb21590 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::mouse_depth_value::mouse_depth_compute::closure$0
// 信息 ID: 6ab89cc1826abe38aa77a6cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::compute_passes::mouse_depth_value::mouse_depth_compute", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::mouse_depth_value::mouse_depth_compute; module=564, inline_record=2732; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
