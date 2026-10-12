// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/render_loop/draw_passes/ui_2d_pass.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 675107a419e0f94a4e5167d8b97d52fca5e60cd766174a088c5db7678c69035b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ui_2d_pass.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 528, 556, 1639, 1655, 1813, 1844, 3393
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
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use tg_text; // namespace candidate
//   package identities: [{"name":"tg-text","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":556,"record_offset":15584,"rva":11433664,"va":5380142784}
// 编译实例 RVAs: 0xae76c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14175a1e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13051675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380142784,"raw_hex":"e81b27c700","size":5,"soft_fail":false,"target_va":5393195488,"va":5380143808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36241083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380142784,"raw_hex":"e8bbfe2802","size":5,"soft_fail":false,"target_va":5416385024,"va":5380143936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36477477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380142784,"raw_hex":"e8259a2c02","size":5,"soft_fail":false,"target_va":5416621408,"va":5380143926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36477699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380142784,"raw_hex":"e8039b2c02","size":5,"soft_fail":false,"target_va":5416621600,"va":5380143896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380142784,"raw_hex":"e8e6690000","size":5,"soft_fail":false,"target_va":5380169984,"va":5380142869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t36477358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380142784,"raw_hex":"e8ae992c02","size":5,"soft_fail":false,"target_va":5416621376,"va":5380144013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3820560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380142784,"raw_hex":"e8104c3a00","size":5,"soft_fail":false,"target_va":5383963792,"va":5380143227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3829064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380142784,"raw_hex":"e8486d3a00","size":5,"soft_fail":false,"target_va":5383972224,"va":5380143155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3821051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380142784,"raw_hex":"e8fb4d3a00","size":5,"soft_fail":false,"target_va":5383964912,"va":5380143856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3819979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380142784,"raw_hex":"e8cb493a00","size":5,"soft_fail":false,"target_va":5383962816,"va":5380142832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36475377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380142784,"raw_hex":"e8f1912c02","size":5,"soft_fail":false,"target_va":5416619360,"va":5380143978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":556,"record_offset":17976,"parent_record_offset":15584,"ancestor_record_offset":15584,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":556,"record_offset":22032,"parent_record_offset":15584,"ancestor_record_offset":15584,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":556,"record_offset":21164,"parent_record_offset":15584,"ancestor_record_offset":15584,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::window::WindowSize::as_vec2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/window.rs
//     evidence={"module":556,"record_offset":19620,"parent_record_offset":15584,"ancestor_record_offset":15584,"inlinee":32182,"source_line":46,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":556,"record_offset":19648,"parent_record_offset":15584,"ancestor_record_offset":15584,"inlinee":12326,"source_line":34,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":556,"record_offset":17168,"parent_record_offset":15584,"ancestor_record_offset":15584,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":556,"record_offset":15768,"parent_record_offset":15584,"ancestor_record_offset":15584,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":556,"record_offset":16156,"parent_record_offset":15584,"ancestor_record_offset":15584,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":556,"record_offset":19296,"parent_record_offset":19272,"ancestor_record_offset":15584,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":556,"record_offset":19344,"parent_record_offset":19320,"ancestor_record_offset":15584,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":556,"record_offset":19320,"parent_record_offset":19296,"ancestor_record_offset":15584,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":556,"record_offset":19272,"parent_record_offset":19248,"ancestor_record_offset":15584,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":556,"record_offset":19248,"parent_record_offset":19208,"ancestor_record_offset":15584,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":556,"record_offset":19208,"parent_record_offset":19184,"ancestor_record_offset":15584,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":556,"record_offset":18004,"parent_record_offset":17976,"ancestor_record_offset":15584,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":556,"record_offset":19568,"parent_record_offset":19544,"ancestor_record_offset":15584,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":556,"record_offset":19544,"parent_record_offset":19520,"ancestor_record_offset":15584,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":556,"record_offset":19492,"parent_record_offset":19460,"ancestor_record_offset":15584,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":556,"record_offset":19436,"parent_record_offset":18928,"ancestor_record_offset":15584,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":556,"record_offset":19460,"parent_record_offset":19436,"ancestor_record_offset":15584,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":556,"record_offset":18956,"parent_record_offset":18928,"ancestor_record_offset":15584,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":556,"record_offset":19520,"parent_record_offset":19460,"ancestor_record_offset":15584,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":556,"record_offset":19004,"parent_record_offset":18980,"ancestor_record_offset":15584,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":556,"record_offset":18980,"parent_record_offset":18956,"ancestor_record_offset":15584,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":556,"record_offset":19184,"parent_record_offset":19160,"ancestor_record_offset":15584,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":556,"record_offset":19680,"parent_record_offset":19648,"ancestor_record_offset":15584,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":556,"record_offset":18928,"parent_record_offset":17976,"ancestor_record_offset":15584,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":556,"record_offset":18028,"parent_record_offset":18004,"ancestor_record_offset":15584,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":556,"record_offset":18676,"parent_record_offset":18652,"ancestor_record_offset":15584,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":556,"record_offset":18704,"parent_record_offset":18676,"ancestor_record_offset":15584,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":556,"record_offset":17496,"parent_record_offset":17468,"ancestor_record_offset":15584,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":556,"record_offset":17524,"parent_record_offset":17496,"ancestor_record_offset":15584,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":556,"record_offset":17444,"parent_record_offset":17416,"ancestor_record_offset":15584,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":556,"record_offset":17468,"parent_record_offset":17444,"ancestor_record_offset":15584,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":556,"record_offset":17584,"parent_record_offset":17468,"ancestor_record_offset":15584,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":556,"record_offset":17696,"parent_record_offset":17672,"ancestor_record_offset":15584,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":556,"record_offset":18244,"parent_record_offset":18220,"ancestor_record_offset":15584,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":556,"record_offset":18328,"parent_record_offset":18304,"ancestor_record_offset":15584,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":556,"record_offset":17548,"parent_record_offset":17524,"ancestor_record_offset":15584,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":556,"record_offset":18476,"parent_record_offset":18448,"ancestor_record_offset":15584,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":556,"record_offset":18448,"parent_record_offset":18420,"ancestor_record_offset":15584,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":556,"record_offset":17316,"parent_record_offset":17276,"ancestor_record_offset":15584,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":556,"record_offset":19120,"parent_record_offset":19096,"ancestor_record_offset":15584,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":556,"record_offset":19160,"parent_record_offset":18980,"ancestor_record_offset":15584,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":556,"record_offset":21132,"parent_record_offset":19648,"ancestor_record_offset":15584,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":556,"record_offset":16104,"parent_record_offset":16076,"ancestor_record_offset":15584,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":556,"record_offset":17932,"parent_record_offset":17276,"ancestor_record_offset":15584,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":556,"record_offset":18268,"parent_record_offset":18244,"ancestor_record_offset":15584,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":556,"record_offset":17648,"parent_record_offset":17416,"ancestor_record_offset":15584,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":556,"record_offset":18592,"parent_record_offset":18568,"ancestor_record_offset":15584,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":556,"record_offset":16576,"parent_record_offset":16552,"ancestor_record_offset":15584,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":556,"record_offset":16552,"parent_record_offset":16528,"ancestor_record_offset":15584,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":556,"record_offset":16760,"parent_record_offset":16732,"ancestor_record_offset":15584,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":556,"record_offset":16860,"parent_record_offset":16832,"ancestor_record_offset":15584,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":556,"record_offset":16832,"parent_record_offset":16804,"ancestor_record_offset":15584,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":556,"record_offset":19032,"parent_record_offset":19004,"ancestor_record_offset":15584,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":556,"record_offset":19068,"parent_record_offset":19032,"ancestor_record_offset":15584,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":556,"record_offset":19388,"parent_record_offset":19208,"ancestor_record_offset":15584,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":556,"record_offset":16504,"parent_record_offset":16480,"ancestor_record_offset":15584,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":556,"record_offset":16528,"parent_record_offset":16504,"ancestor_record_offset":15584,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":556,"record_offset":16888,"parent_record_offset":16860,"ancestor_record_offset":15584,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":556,"record_offset":16480,"parent_record_offset":16364,"ancestor_record_offset":15584,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":556,"record_offset":15896,"parent_record_offset":15872,"ancestor_record_offset":15584,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":556,"record_offset":16192,"parent_record_offset":16156,"ancestor_record_offset":15584,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":556,"record_offset":16332,"parent_record_offset":16300,"ancestor_record_offset":15584,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":556,"record_offset":16300,"parent_record_offset":16264,"ancestor_record_offset":15584,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":556,"record_offset":16240,"parent_record_offset":16216,"ancestor_record_offset":15584,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":556,"record_offset":16264,"parent_record_offset":16240,"ancestor_record_offset":15584,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":556,"record_offset":16216,"parent_record_offset":16192,"ancestor_record_offset":15584,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":556,"record_offset":16804,"parent_record_offset":16264,"ancestor_record_offset":15584,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":556,"record_offset":15872,"parent_record_offset":15840,"ancestor_record_offset":15584,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":556,"record_offset":15840,"parent_record_offset":15812,"ancestor_record_offset":15584,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":556,"record_offset":15996,"parent_record_offset":15964,"ancestor_record_offset":15584,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":556,"record_offset":16052,"parent_record_offset":15996,"ancestor_record_offset":15584,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":556,"record_offset":15964,"parent_record_offset":15936,"ancestor_record_offset":15584,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":556,"record_offset":15812,"parent_record_offset":15768,"ancestor_record_offset":15584,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":556,"record_offset":16076,"parent_record_offset":16052,"ancestor_record_offset":15584,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":556,"record_offset":16364,"parent_record_offset":16332,"ancestor_record_offset":15584,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":556,"record_offset":16648,"parent_record_offset":16364,"ancestor_record_offset":15584,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":556,"record_offset":16620,"parent_record_offset":16364,"ancestor_record_offset":15584,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":556,"record_offset":16424,"parent_record_offset":16364,"ancestor_record_offset":15584,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":556,"record_offset":18652,"parent_record_offset":18628,"ancestor_record_offset":15584,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":556,"record_offset":18388,"parent_record_offset":18360,"ancestor_record_offset":15584,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":556,"record_offset":18420,"parent_record_offset":18388,"ancestor_record_offset":15584,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":556,"record_offset":18360,"parent_record_offset":18140,"ancestor_record_offset":15584,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":556,"record_offset":18220,"parent_record_offset":18140,"ancestor_record_offset":15584,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":556,"record_offset":18780,"parent_record_offset":18140,"ancestor_record_offset":15584,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":556,"record_offset":18304,"parent_record_offset":18140,"ancestor_record_offset":15584,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":556,"record_offset":18064,"parent_record_offset":18028,"ancestor_record_offset":15584,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":556,"record_offset":18628,"parent_record_offset":18520,"ancestor_record_offset":15584,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":18568,"parent_record_offset":18544,"ancestor_record_offset":15584,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":18872,"parent_record_offset":18140,"ancestor_record_offset":15584,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":18544,"parent_record_offset":18520,"ancestor_record_offset":15584,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":18116,"parent_record_offset":18092,"ancestor_record_offset":15584,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":18092,"parent_record_offset":18064,"ancestor_record_offset":15584,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":18140,"parent_record_offset":18116,"ancestor_record_offset":15584,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":18192,"parent_record_offset":18140,"ancestor_record_offset":15584,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":18520,"parent_record_offset":18140,"ancestor_record_offset":15584,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":556,"record_offset":18752,"parent_record_offset":18140,"ancestor_record_offset":15584,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":556,"record_offset":20868,"parent_record_offset":20844,"ancestor_record_offset":15584,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":556,"record_offset":17204,"parent_record_offset":17168,"ancestor_record_offset":15584,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":556,"record_offset":21228,"parent_record_offset":21196,"ancestor_record_offset":15584,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":556,"record_offset":21668,"parent_record_offset":21464,"ancestor_record_offset":15584,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":556,"record_offset":17344,"parent_record_offset":17316,"ancestor_record_offset":15584,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":556,"record_offset":17416,"parent_record_offset":17276,"ancestor_record_offset":15584,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":556,"record_offset":17620,"parent_record_offset":17416,"ancestor_record_offset":15584,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":556,"record_offset":17372,"parent_record_offset":17344,"ancestor_record_offset":15584,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":556,"record_offset":17276,"parent_record_offset":17240,"ancestor_record_offset":15584,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":556,"record_offset":17240,"parent_record_offset":17204,"ancestor_record_offset":15584,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1404F
// 首条位置: {"module":1655,"inlinee":81999}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1655,"record_offset":69100,"parent_record_offset":68200,"ancestor_record_offset":68200,"inlinee":13088,"source_line":192,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1655,"record_offset":69272,"parent_record_offset":68200,"ancestor_record_offset":68200,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1655,"record_offset":69328,"parent_record_offset":68200,"ancestor_record_offset":68200,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::state::QueryState::iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/state.rs
//     evidence={"module":1655,"record_offset":68476,"parent_record_offset":68200,"ancestor_record_offset":68200,"inlinee":29998,"source_line":1092,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::query
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":1655,"record_offset":68420,"parent_record_offset":68200,"ancestor_record_offset":68200,"inlinee":29996,"source_line":1549,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1655,"record_offset":69016,"parent_record_offset":68200,"ancestor_record_offset":68200,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1655,"record_offset":69508,"parent_record_offset":68200,"ancestor_record_offset":68200,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1655,"record_offset":69540,"parent_record_offset":68200,"ancestor_record_offset":68200,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1655,"record_offset":69444,"parent_record_offset":68200,"ancestor_record_offset":68200,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1655,"record_offset":70584,"parent_record_offset":68200,"ancestor_record_offset":68200,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_bindless_handle_by_name_hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/texture_library.rs
//     evidence={"module":1655,"record_offset":69572,"parent_record_offset":68200,"ancestor_record_offset":68200,"inlinee":30266,"source_line":429,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::TextureType::is_array
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/texture.rs
//     evidence={"module":1655,"record_offset":69480,"parent_record_offset":68200,"ancestor_record_offset":68200,"inlinee":30184,"source_line":313,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1655,"record_offset":70840,"parent_record_offset":70816,"ancestor_record_offset":68200,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1655,"record_offset":70864,"parent_record_offset":70840,"ancestor_record_offset":68200,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1655,"record_offset":70888,"parent_record_offset":70864,"ancestor_record_offset":68200,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1655,"record_offset":70816,"parent_record_offset":70728,"ancestor_record_offset":68200,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1655,"record_offset":71544,"parent_record_offset":71520,"ancestor_record_offset":68200,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1655,"record_offset":70756,"parent_record_offset":70728,"ancestor_record_offset":68200,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1655,"record_offset":70728,"parent_record_offset":70704,"ancestor_record_offset":68200,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1655,"record_offset":71592,"parent_record_offset":71568,"ancestor_record_offset":68200,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1655,"record_offset":71568,"parent_record_offset":71544,"ancestor_record_offset":68200,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1655,"record_offset":70704,"parent_record_offset":70680,"ancestor_record_offset":68200,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1655,"record_offset":69132,"parent_record_offset":69100,"ancestor_record_offset":68200,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1655,"record_offset":71520,"parent_record_offset":71496,"ancestor_record_offset":68200,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1655,"record_offset":71496,"parent_record_offset":71464,"ancestor_record_offset":68200,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1655,"record_offset":69040,"parent_record_offset":69016,"ancestor_record_offset":68200,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1655,"record_offset":71464,"parent_record_offset":71432,"ancestor_record_offset":68200,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1655,"record_offset":70608,"parent_record_offset":70584,"ancestor_record_offset":68200,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1655,"record_offset":70656,"parent_record_offset":70632,"ancestor_record_offset":68200,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":1655,"record_offset":68936,"parent_record_offset":68912,"ancestor_record_offset":68200,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":1655,"record_offset":71432,"parent_record_offset":71408,"ancestor_record_offset":68200,"inlinee":9124,"source_line":2408,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::query
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/state.rs
//     evidence={"module":1655,"record_offset":68508,"parent_record_offset":68476,"ancestor_record_offset":68200,"inlinee":34686,"source_line":300,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::query_manual
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/state.rs
//     evidence={"module":1655,"record_offset":68568,"parent_record_offset":68508,"ancestor_record_offset":68200,"inlinee":34687,"source_line":316,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::query_unchecked_manual
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/state.rs
//     evidence={"module":1655,"record_offset":68704,"parent_record_offset":68568,"ancestor_record_offset":68200,"inlinee":34688,"source_line":366,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::update_archetypes
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/state.rs
//     evidence={"module":1655,"record_offset":68540,"parent_record_offset":68508,"ancestor_record_offset":68200,"inlinee":9051,"source_line":487,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::impl$4::into_iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":1655,"record_offset":68912,"parent_record_offset":68476,"ancestor_record_offset":68200,"inlinee":10314,"source_line":2505,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":1655,"record_offset":68604,"parent_record_offset":68568,"ancestor_record_offset":68200,"inlinee":4097,"source_line":183,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::last_change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":1655,"record_offset":68756,"parent_record_offset":68732,"ancestor_record_offset":68200,"inlinee":17701,"source_line":2890,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::query_filtered
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":1655,"record_offset":68444,"parent_record_offset":68420,"ancestor_record_offset":68200,"inlinee":29997,"source_line":1573,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":1655,"record_offset":68812,"parent_record_offset":68788,"ancestor_record_offset":68200,"inlinee":4201,"source_line":2868,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::identifier::impl$10::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/identifier.rs
//     evidence={"module":1655,"record_offset":68668,"parent_record_offset":68644,"ancestor_record_offset":68200,"inlinee":4099,"source_line":12,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::archetypes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":1655,"record_offset":68976,"parent_record_offset":68936,"ancestor_record_offset":68200,"inlinee":11874,"source_line":264,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":1655,"record_offset":68788,"parent_record_offset":68704,"ancestor_record_offset":68200,"inlinee":4200,"source_line":302,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::last_change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":1655,"record_offset":68732,"parent_record_offset":68704,"ancestor_record_offset":68200,"inlinee":17700,"source_line":327,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1655,"record_offset":68644,"parent_record_offset":68604,"ancestor_record_offset":68200,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1655,"record_offset":70340,"parent_record_offset":70316,"ancestor_record_offset":68200,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1655,"record_offset":69848,"parent_record_offset":69824,"ancestor_record_offset":68200,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1655,"record_offset":69932,"parent_record_offset":69908,"ancestor_record_offset":68200,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1655,"record_offset":69204,"parent_record_offset":69164,"ancestor_record_offset":68200,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1655,"record_offset":69064,"parent_record_offset":69040,"ancestor_record_offset":68200,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1655,"record_offset":70784,"parent_record_offset":70756,"ancestor_record_offset":68200,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1655,"record_offset":70080,"parent_record_offset":70052,"ancestor_record_offset":68200,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1655,"record_offset":70052,"parent_record_offset":70024,"ancestor_record_offset":68200,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1655,"record_offset":69872,"parent_record_offset":69848,"ancestor_record_offset":68200,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1655,"record_offset":70632,"parent_record_offset":70608,"ancestor_record_offset":68200,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1655,"record_offset":69296,"parent_record_offset":69272,"ancestor_record_offset":68200,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1655,"record_offset":70200,"parent_record_offset":70176,"ancestor_record_offset":68200,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1655,"record_offset":69384,"parent_record_offset":69328,"ancestor_record_offset":68200,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1655,"record_offset":69356,"parent_record_offset":69328,"ancestor_record_offset":68200,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1655,"record_offset":69408,"parent_record_offset":69384,"ancestor_record_offset":68200,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1655,"record_offset":71636,"parent_record_offset":71464,"ancestor_record_offset":68200,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":1655,"record_offset":69164,"parent_record_offset":69132,"ancestor_record_offset":68200,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1655,"record_offset":68836,"parent_record_offset":68812,"ancestor_record_offset":68200,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1655,"record_offset":68860,"parent_record_offset":68836,"ancestor_record_offset":68200,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":1655,"record_offset":69992,"parent_record_offset":69964,"ancestor_record_offset":68200,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":1655,"record_offset":70024,"parent_record_offset":69992,"ancestor_record_offset":68200,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":1655,"record_offset":69964,"parent_record_offset":69712,"ancestor_record_offset":68200,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":1655,"record_offset":69824,"parent_record_offset":69712,"ancestor_record_offset":68200,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":1655,"record_offset":70408,"parent_record_offset":69712,"ancestor_record_offset":68200,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":1655,"record_offset":69908,"parent_record_offset":69712,"ancestor_record_offset":68200,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/tag.rs
//     evidence={"module":1655,"record_offset":69768,"parent_record_offset":69712,"ancestor_record_offset":68200,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1655,"record_offset":70316,"parent_record_offset":70292,"ancestor_record_offset":68200,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1655,"record_offset":69632,"parent_record_offset":69604,"ancestor_record_offset":68200,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":1655,"record_offset":70292,"parent_record_offset":70124,"ancestor_record_offset":68200,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":70232,"parent_record_offset":70148,"ancestor_record_offset":68200,"inlinee":8504,"source_line":435,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1655,"record_offset":70176,"parent_record_offset":70148,"ancestor_record_offset":68200,"inlinee":4324,"source_line":261,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1655,"record_offset":70500,"parent_record_offset":69712,"ancestor_record_offset":68200,"inlinee":4339,"source_line":81,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1655,"record_offset":70148,"parent_record_offset":70124,"ancestor_record_offset":68200,"inlinee":4322,"source_line":734,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1655,"record_offset":69688,"parent_record_offset":69664,"ancestor_record_offset":68200,"inlinee":4302,"source_line":1156,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":69664,"parent_record_offset":69632,"ancestor_record_offset":68200,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":69712,"parent_record_offset":69688,"ancestor_record_offset":68200,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":69796,"parent_record_offset":69712,"ancestor_record_offset":68200,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1655,"record_offset":70124,"parent_record_offset":69712,"ancestor_record_offset":68200,"inlinee":4321,"source_line":1166,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1655,"record_offset":69604,"parent_record_offset":69572,"ancestor_record_offset":68200,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=31
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7B65
// 首条位置: {"module":528,"inlinee":31589}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=50
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":528,"record_offset":61528,"parent_record_offset":61484,"ancestor_record_offset":61484,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 163
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":556,"record_offset":22784,"rva":11435248,"va":5380144368}
// 编译实例 RVAs: 0xae7cf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..texture..Texture$u20$as$u20$rhapsody..rg..import_export..GraphImportable$GT$::import
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3348249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380144368,"raw_hex":"e819173300","size":5,"soft_fail":false,"target_va":5383494288,"va":5380146034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..texture..Texture$u20$as$u20$rhapsody..rg..resource..GraphResource$GT$::init_as_debug_garbage
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t3326933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380144368,"raw_hex":"e8d5c33200","size":5,"soft_fail":false,"target_va":5383472768,"va":5380145830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141538de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10817029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380144368,"raw_hex":"e8050ea500","size":5,"soft_fail":false,"target_va":5390962144,"va":5380145110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14153dc80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10837176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380144368,"raw_hex":"e8b85ca500","size":5,"soft_fail":false,"target_va":5390982272,"va":5380145091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141548280
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10878259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380144368,"raw_hex":"e833fda500","size":5,"soft_fail":false,"target_va":5391024768,"va":5380146504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14154a110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10886095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380144368,"raw_hex":"e8cf1ba600","size":5,"soft_fail":false,"target_va":5391032592,"va":5380146492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141754140
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13024267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380144368,"raw_hex":"e80bbcc600","size":5,"soft_fail":false,"target_va":5393170752,"va":5380146480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141760b00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13076014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380144368,"raw_hex":"e82e86c700","size":5,"soft_fail":false,"target_va":5393222400,"va":5380146381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36238405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380144368,"raw_hex":"e845f42802","size":5,"soft_fail":false,"target_va":5416385024,"va":5380146614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36474799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380144368,"raw_hex":"e8af8f2c02","size":5,"soft_fail":false,"target_va":5416621408,"va":5380146604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t36476010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380144368,"raw_hex":"e86a942c02","size":5,"soft_fail":false,"target_va":5416621600,"va":5380145585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t25517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380144368,"raw_hex":"e8ad630000","size":5,"soft_fail":false,"target_va":5380169984,"va":5380144462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t36474677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380144368,"raw_hex":"e8358f2c02","size":5,"soft_fail":false,"target_va":5416621376,"va":5380146694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3818978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380144368,"raw_hex":"e8e2453a00","size":5,"soft_fail":false,"target_va":5383963792,"va":5380144809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3827482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380144368,"raw_hex":"e81a673a00","size":5,"soft_fail":false,"target_va":5383972224,"va":5380144737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3818346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380144368,"raw_hex":"e86a433a00","size":5,"soft_fail":false,"target_va":5383964912,"va":5380146561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3818389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380144368,"raw_hex":"e895433a00","size":5,"soft_fail":false,"target_va":5383962816,"va":5380144422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::api::GraphApi::clear_color_texture_f32
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3402236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380144368,"raw_hex":"e8fce93300","size":5,"soft_fail":false,"target_va":5383548512,"va":5380146271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::api::GraphApi::create_raw_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3402370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380144368,"raw_hex":"e882ea3300","size":5,"soft_fail":false,"target_va":5383548144,"va":5380145769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36472720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380144368,"raw_hex":"e890872c02","size":5,"soft_fail":false,"target_va":5416619360,"va":5380146635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextRenderer::render
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1876199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380144368,"raw_hex":"e8e7a01c00","size":5,"soft_fail":false,"target_va":5382021856,"va":5380145652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":556,"record_offset":30580,"parent_record_offset":22784,"ancestor_record_offset":22784,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::WithRg::run
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":556,"record_offset":26876,"parent_record_offset":22784,"ancestor_record_offset":22784,"inlinee":12358,"source_line":489,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":556,"record_offset":25168,"parent_record_offset":22784,"ancestor_record_offset":22784,"inlinee":12326,"source_line":34,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":556,"record_offset":24360,"parent_record_offset":22784,"ancestor_record_offset":22784,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":556,"record_offset":22960,"parent_record_offset":22784,"ancestor_record_offset":22784,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":556,"record_offset":23348,"parent_record_offset":22784,"ancestor_record_offset":22784,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":556,"record_offset":26520,"parent_record_offset":26496,"ancestor_record_offset":22784,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":556,"record_offset":26568,"parent_record_offset":26544,"ancestor_record_offset":22784,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":556,"record_offset":26544,"parent_record_offset":26520,"ancestor_record_offset":22784,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":556,"record_offset":26496,"parent_record_offset":26472,"ancestor_record_offset":22784,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":556,"record_offset":26472,"parent_record_offset":26432,"ancestor_record_offset":22784,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":556,"record_offset":26432,"parent_record_offset":26408,"ancestor_record_offset":22784,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":556,"record_offset":25228,"parent_record_offset":25200,"ancestor_record_offset":22784,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":556,"record_offset":26792,"parent_record_offset":26768,"ancestor_record_offset":22784,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":556,"record_offset":26768,"parent_record_offset":26744,"ancestor_record_offset":22784,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":556,"record_offset":26716,"parent_record_offset":26684,"ancestor_record_offset":22784,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":556,"record_offset":26660,"parent_record_offset":26152,"ancestor_record_offset":22784,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":556,"record_offset":26684,"parent_record_offset":26660,"ancestor_record_offset":22784,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":556,"record_offset":26180,"parent_record_offset":26152,"ancestor_record_offset":22784,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":556,"record_offset":26744,"parent_record_offset":26684,"ancestor_record_offset":22784,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":556,"record_offset":26228,"parent_record_offset":26204,"ancestor_record_offset":22784,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":556,"record_offset":26204,"parent_record_offset":26180,"ancestor_record_offset":22784,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":556,"record_offset":26408,"parent_record_offset":26384,"ancestor_record_offset":22784,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":556,"record_offset":25200,"parent_record_offset":25168,"ancestor_record_offset":22784,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":556,"record_offset":26152,"parent_record_offset":25200,"ancestor_record_offset":22784,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":556,"record_offset":25252,"parent_record_offset":25228,"ancestor_record_offset":22784,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":556,"record_offset":25900,"parent_record_offset":25876,"ancestor_record_offset":22784,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":556,"record_offset":25928,"parent_record_offset":25900,"ancestor_record_offset":22784,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":556,"record_offset":24688,"parent_record_offset":24660,"ancestor_record_offset":22784,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":556,"record_offset":24716,"parent_record_offset":24688,"ancestor_record_offset":22784,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":556,"record_offset":24636,"parent_record_offset":24608,"ancestor_record_offset":22784,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":556,"record_offset":24660,"parent_record_offset":24636,"ancestor_record_offset":22784,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":556,"record_offset":24776,"parent_record_offset":24660,"ancestor_record_offset":22784,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":556,"record_offset":24888,"parent_record_offset":24864,"ancestor_record_offset":22784,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":556,"record_offset":25468,"parent_record_offset":25444,"ancestor_record_offset":22784,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":556,"record_offset":25552,"parent_record_offset":25528,"ancestor_record_offset":22784,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":556,"record_offset":24740,"parent_record_offset":24716,"ancestor_record_offset":22784,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":556,"record_offset":25700,"parent_record_offset":25672,"ancestor_record_offset":22784,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":556,"record_offset":25672,"parent_record_offset":25644,"ancestor_record_offset":22784,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":556,"record_offset":24508,"parent_record_offset":24468,"ancestor_record_offset":22784,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":556,"record_offset":26344,"parent_record_offset":26320,"ancestor_record_offset":22784,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":556,"record_offset":26384,"parent_record_offset":26204,"ancestor_record_offset":22784,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":556,"record_offset":26844,"parent_record_offset":25168,"ancestor_record_offset":22784,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":556,"record_offset":23296,"parent_record_offset":23268,"ancestor_record_offset":22784,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":556,"record_offset":25124,"parent_record_offset":24468,"ancestor_record_offset":22784,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":556,"record_offset":25492,"parent_record_offset":25468,"ancestor_record_offset":22784,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":556,"record_offset":24840,"parent_record_offset":24608,"ancestor_record_offset":22784,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":556,"record_offset":25816,"parent_record_offset":25792,"ancestor_record_offset":22784,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":556,"record_offset":23768,"parent_record_offset":23744,"ancestor_record_offset":22784,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":556,"record_offset":23744,"parent_record_offset":23720,"ancestor_record_offset":22784,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":556,"record_offset":23952,"parent_record_offset":23924,"ancestor_record_offset":22784,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":556,"record_offset":24052,"parent_record_offset":24024,"ancestor_record_offset":22784,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":556,"record_offset":24024,"parent_record_offset":23996,"ancestor_record_offset":22784,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":556,"record_offset":26256,"parent_record_offset":26228,"ancestor_record_offset":22784,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":556,"record_offset":26292,"parent_record_offset":26256,"ancestor_record_offset":22784,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":556,"record_offset":26612,"parent_record_offset":26432,"ancestor_record_offset":22784,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":556,"record_offset":23696,"parent_record_offset":23672,"ancestor_record_offset":22784,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":556,"record_offset":23720,"parent_record_offset":23696,"ancestor_record_offset":22784,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":556,"record_offset":24080,"parent_record_offset":24052,"ancestor_record_offset":22784,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":556,"record_offset":23672,"parent_record_offset":23556,"ancestor_record_offset":22784,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":556,"record_offset":23088,"parent_record_offset":23064,"ancestor_record_offset":22784,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":556,"record_offset":23384,"parent_record_offset":23348,"ancestor_record_offset":22784,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":556,"record_offset":23524,"parent_record_offset":23492,"ancestor_record_offset":22784,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":556,"record_offset":23492,"parent_record_offset":23456,"ancestor_record_offset":22784,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":556,"record_offset":23432,"parent_record_offset":23408,"ancestor_record_offset":22784,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":556,"record_offset":23456,"parent_record_offset":23432,"ancestor_record_offset":22784,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":556,"record_offset":23408,"parent_record_offset":23384,"ancestor_record_offset":22784,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":556,"record_offset":23996,"parent_record_offset":23456,"ancestor_record_offset":22784,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":556,"record_offset":23064,"parent_record_offset":23032,"ancestor_record_offset":22784,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":556,"record_offset":23032,"parent_record_offset":23004,"ancestor_record_offset":22784,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":556,"record_offset":23188,"parent_record_offset":23156,"ancestor_record_offset":22784,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":556,"record_offset":23244,"parent_record_offset":23188,"ancestor_record_offset":22784,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":556,"record_offset":23156,"parent_record_offset":23128,"ancestor_record_offset":22784,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":556,"record_offset":23004,"parent_record_offset":22960,"ancestor_record_offset":22784,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":556,"record_offset":23268,"parent_record_offset":23244,"ancestor_record_offset":22784,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":556,"record_offset":23556,"parent_record_offset":23524,"ancestor_record_offset":22784,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":556,"record_offset":23840,"parent_record_offset":23556,"ancestor_record_offset":22784,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":556,"record_offset":23812,"parent_record_offset":23556,"ancestor_record_offset":22784,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":556,"record_offset":23616,"parent_record_offset":23556,"ancestor_record_offset":22784,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::fetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":556,"record_offset":26968,"parent_record_offset":26916,"ancestor_record_offset":22784,"inlinee":9087,"source_line":545,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::unfetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":556,"record_offset":30544,"parent_record_offset":30492,"ancestor_record_offset":22784,"inlinee":9089,"source_line":552,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$11::fetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":556,"record_offset":26940,"parent_record_offset":26916,"ancestor_record_offset":22784,"inlinee":30203,"source_line":568,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$11::unfetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":556,"record_offset":30516,"parent_record_offset":30492,"ancestor_record_offset":22784,"inlinee":30215,"source_line":575,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$21::fetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":556,"record_offset":26916,"parent_record_offset":26876,"ancestor_record_offset":22784,"inlinee":30229,"source_line":593,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$21::unfetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":556,"record_offset":30492,"parent_record_offset":26876,"ancestor_record_offset":22784,"inlinee":30234,"source_line":596,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::window::WindowSize::as_vec2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/window.rs
//     evidence={"module":556,"record_offset":28740,"parent_record_offset":27000,"ancestor_record_offset":22784,"inlinee":32182,"source_line":46,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/ui_2d_pass.rs
//     evidence={"module":556,"record_offset":27000,"parent_record_offset":26876,"ancestor_record_offset":22784,"inlinee":34735,"source_line":181,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":556,"record_offset":25876,"parent_record_offset":25852,"ancestor_record_offset":22784,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":556,"record_offset":25612,"parent_record_offset":25584,"ancestor_record_offset":22784,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":556,"record_offset":25644,"parent_record_offset":25612,"ancestor_record_offset":22784,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":556,"record_offset":25584,"parent_record_offset":25364,"ancestor_record_offset":22784,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":556,"record_offset":25444,"parent_record_offset":25364,"ancestor_record_offset":22784,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":556,"record_offset":26004,"parent_record_offset":25364,"ancestor_record_offset":22784,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":556,"record_offset":25528,"parent_record_offset":25364,"ancestor_record_offset":22784,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":556,"record_offset":25288,"parent_record_offset":25252,"ancestor_record_offset":22784,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":556,"record_offset":25852,"parent_record_offset":25744,"ancestor_record_offset":22784,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":25792,"parent_record_offset":25768,"ancestor_record_offset":22784,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":26096,"parent_record_offset":25364,"ancestor_record_offset":22784,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":25768,"parent_record_offset":25744,"ancestor_record_offset":22784,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":25340,"parent_record_offset":25316,"ancestor_record_offset":22784,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":25316,"parent_record_offset":25288,"ancestor_record_offset":22784,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":25364,"parent_record_offset":25340,"ancestor_record_offset":22784,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":25416,"parent_record_offset":25364,"ancestor_record_offset":22784,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":25744,"parent_record_offset":25364,"ancestor_record_offset":22784,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":556,"record_offset":25976,"parent_record_offset":25364,"ancestor_record_offset":22784,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":556,"record_offset":29964,"parent_record_offset":29940,"ancestor_record_offset":22784,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":556,"record_offset":24396,"parent_record_offset":24360,"ancestor_record_offset":22784,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":556,"record_offset":30644,"parent_record_offset":30612,"ancestor_record_offset":22784,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":556,"record_offset":31084,"parent_record_offset":30880,"ancestor_record_offset":22784,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":556,"record_offset":24536,"parent_record_offset":24508,"ancestor_record_offset":22784,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":556,"record_offset":24608,"parent_record_offset":24468,"ancestor_record_offset":22784,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":556,"record_offset":24812,"parent_record_offset":24608,"ancestor_record_offset":22784,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphResourceApi::import
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":556,"record_offset":30356,"parent_record_offset":27000,"ancestor_record_offset":22784,"inlinee":12355,"source_line":79,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::impl$0::into_graph_borrow
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":556,"record_offset":30408,"parent_record_offset":30380,"ancestor_record_offset":22784,"inlinee":8323,"source_line":186,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::impl$6::create
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":556,"record_offset":30256,"parent_record_offset":27000,"ancestor_record_offset":22784,"inlinee":12324,"source_line":1280,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::impl$6::import_with_access_type
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":556,"record_offset":30380,"parent_record_offset":30356,"ancestor_record_offset":22784,"inlinee":12356,"source_line":1308,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::TextureDesc::dims_2d
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":556,"record_offset":30228,"parent_record_offset":27000,"ancestor_record_offset":22784,"inlinee":9291,"source_line":410,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":556,"record_offset":24564,"parent_record_offset":24536,"ancestor_record_offset":22784,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":556,"record_offset":24468,"parent_record_offset":24432,"ancestor_record_offset":22784,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":556,"record_offset":24432,"parent_record_offset":24396,"ancestor_record_offset":22784,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 181
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x87AF
// 首条位置: {"module":556,"inlinee":34735}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":556,"record_offset":27096,"parent_record_offset":27000,"ancestor_record_offset":27000,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":556,"record_offset":30300,"parent_record_offset":27000,"ancestor_record_offset":27000,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::window::WindowSize::as_vec2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/window.rs
//     evidence={"module":556,"record_offset":28740,"parent_record_offset":27000,"ancestor_record_offset":27000,"inlinee":32182,"source_line":46,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphResourceApi::import
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":556,"record_offset":30356,"parent_record_offset":27000,"ancestor_record_offset":27000,"inlinee":12355,"source_line":79,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::impl$6::create
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":556,"record_offset":30256,"parent_record_offset":27000,"ancestor_record_offset":27000,"inlinee":12324,"source_line":1280,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::TextureDesc::dims_2d
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":556,"record_offset":30228,"parent_record_offset":27000,"ancestor_record_offset":27000,"inlinee":9291,"source_line":410,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":556,"record_offset":28416,"parent_record_offset":28392,"ancestor_record_offset":27000,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":556,"record_offset":28464,"parent_record_offset":28440,"ancestor_record_offset":27000,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":556,"record_offset":28440,"parent_record_offset":28416,"ancestor_record_offset":27000,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":556,"record_offset":28392,"parent_record_offset":28368,"ancestor_record_offset":27000,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":556,"record_offset":28368,"parent_record_offset":28328,"ancestor_record_offset":27000,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":556,"record_offset":28328,"parent_record_offset":28304,"ancestor_record_offset":27000,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":556,"record_offset":27124,"parent_record_offset":27096,"ancestor_record_offset":27000,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":556,"record_offset":28688,"parent_record_offset":28664,"ancestor_record_offset":27000,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":556,"record_offset":28664,"parent_record_offset":28640,"ancestor_record_offset":27000,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":556,"record_offset":28612,"parent_record_offset":28580,"ancestor_record_offset":27000,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":556,"record_offset":28556,"parent_record_offset":28048,"ancestor_record_offset":27000,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":556,"record_offset":28580,"parent_record_offset":28556,"ancestor_record_offset":27000,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":556,"record_offset":28076,"parent_record_offset":28048,"ancestor_record_offset":27000,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":556,"record_offset":28640,"parent_record_offset":28580,"ancestor_record_offset":27000,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":556,"record_offset":28124,"parent_record_offset":28100,"ancestor_record_offset":27000,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":556,"record_offset":28100,"parent_record_offset":28076,"ancestor_record_offset":27000,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":556,"record_offset":28304,"parent_record_offset":28280,"ancestor_record_offset":27000,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":556,"record_offset":28048,"parent_record_offset":27096,"ancestor_record_offset":27000,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":556,"record_offset":27148,"parent_record_offset":27124,"ancestor_record_offset":27000,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":556,"record_offset":27796,"parent_record_offset":27772,"ancestor_record_offset":27000,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":556,"record_offset":27824,"parent_record_offset":27796,"ancestor_record_offset":27000,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":556,"record_offset":27364,"parent_record_offset":27340,"ancestor_record_offset":27000,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":556,"record_offset":27448,"parent_record_offset":27424,"ancestor_record_offset":27000,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":556,"record_offset":27596,"parent_record_offset":27568,"ancestor_record_offset":27000,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":556,"record_offset":27568,"parent_record_offset":27540,"ancestor_record_offset":27000,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":556,"record_offset":28216,"parent_record_offset":28152,"ancestor_record_offset":27000,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":556,"record_offset":28240,"parent_record_offset":28216,"ancestor_record_offset":27000,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":556,"record_offset":28280,"parent_record_offset":28100,"ancestor_record_offset":27000,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":556,"record_offset":27388,"parent_record_offset":27364,"ancestor_record_offset":27000,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":556,"record_offset":27712,"parent_record_offset":27688,"ancestor_record_offset":27000,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":556,"record_offset":28152,"parent_record_offset":28124,"ancestor_record_offset":27000,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":556,"record_offset":28188,"parent_record_offset":28152,"ancestor_record_offset":27000,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":556,"record_offset":28508,"parent_record_offset":28328,"ancestor_record_offset":27000,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":556,"record_offset":27772,"parent_record_offset":27748,"ancestor_record_offset":27000,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":556,"record_offset":27508,"parent_record_offset":27480,"ancestor_record_offset":27000,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":556,"record_offset":27540,"parent_record_offset":27508,"ancestor_record_offset":27000,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":556,"record_offset":27480,"parent_record_offset":27260,"ancestor_record_offset":27000,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":556,"record_offset":27340,"parent_record_offset":27260,"ancestor_record_offset":27000,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":556,"record_offset":27900,"parent_record_offset":27260,"ancestor_record_offset":27000,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":556,"record_offset":27424,"parent_record_offset":27260,"ancestor_record_offset":27000,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":556,"record_offset":27184,"parent_record_offset":27148,"ancestor_record_offset":27000,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":556,"record_offset":27748,"parent_record_offset":27640,"ancestor_record_offset":27000,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":27688,"parent_record_offset":27664,"ancestor_record_offset":27000,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":27992,"parent_record_offset":27260,"ancestor_record_offset":27000,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":27664,"parent_record_offset":27640,"ancestor_record_offset":27000,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":27236,"parent_record_offset":27212,"ancestor_record_offset":27000,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":27212,"parent_record_offset":27184,"ancestor_record_offset":27000,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":27260,"parent_record_offset":27236,"ancestor_record_offset":27000,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":27312,"parent_record_offset":27260,"ancestor_record_offset":27000,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":27640,"parent_record_offset":27260,"ancestor_record_offset":27000,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":556,"record_offset":27872,"parent_record_offset":27260,"ancestor_record_offset":27000,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":556,"record_offset":29964,"parent_record_offset":29940,"ancestor_record_offset":27000,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::impl$0::into_graph_borrow
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":556,"record_offset":30408,"parent_record_offset":30380,"ancestor_record_offset":27000,"inlinee":8323,"source_line":186,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::impl$6::import_with_access_type
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":556,"record_offset":30380,"parent_record_offset":30356,"ancestor_record_offset":27000,"inlinee":12356,"source_line":1308,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 218
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1405B
// 首条位置: {"module":1655,"inlinee":82011}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure$0::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 237
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14034
// 首条位置: {"module":1655,"inlinee":81972}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: ui_2d_pass.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::read_rg_buffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::read_rg_buffer", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_read_rg_buffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_read_rg_buffer", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ui_2d_pass.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/ui_2d_pass.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: ui_2d_pass.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":3756576,"caller_aliases":{"alias_group_rva":3756576,"count":1,"display_examples":["rhapsody::rg::compile::compile"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3759905,"position_among_mapped_render_targets":2,"mapped_render_target_count":3,"previous_callsite":3759867,"next_callsite":3759942,"previous_target_files":["crates/country-core/src/systems/render_loop/vermeer.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/ui_pass.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":3767344,"caller_aliases":{"alias_group_rva":3767344,"count":1,"display_examples":["rhapsody::rg::compile::compile"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3778232,"position_among_mapped_render_targets":19,"mapped_render_target_count":24,"previous_callsite":3778202,"next_callsite":3778293,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/ui_pass.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/ui_2d_pass.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":3767344,"caller_aliases":{"alias_group_rva":3767344,"count":1,"display_examples":["rhapsody::rg::compile::compile"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3778293,"position_among_mapped_render_targets":20,"mapped_render_target_count":24,"previous_callsite":3778232,"next_callsite":3778323,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/ui_2d_pass.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/custom_cursor_pass.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ui_2d_pass.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 15; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<ref$<alloc::string::String> >,ref$<ref$<alloc::string::String> >)
// 候选参数类型: ["ref$<ref$<alloc::string::String> >", "ref$<ref$<alloc::string::String> >"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<tuple$<alloc::string::String,std::path::PathBuf> >,ref$<tuple$<alloc::string::String,std::path::PathBuf> >)
// 候选参数类型: ["ref$<tuple$<alloc::string::String,std::path::PathBuf> >", "ref$<tuple$<alloc::string::String,std::path::PathBuf> >"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::clutter_registry::ClutterId>,ref$<country_core::resources::clutter_registry::ClutterId>)
// 候选参数类型: ["ref$<country_core::resources::clutter_registry::ClutterId>", "ref$<country_core::resources::clutter_registry::ClutterId>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<ref$<str$> >,ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >", "ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<mesh_to_sdf::generate::grid::State>,ref$<mesh_to_sdf::generate::grid::State>)
// 候选参数类型: ["ref$<mesh_to_sdf::generate::grid::State>", "ref$<mesh_to_sdf::generate::grid::State>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<u64>,ref$<u64>)
// 候选参数类型: ["ref$<u64>", "ref$<u64>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorId>)
// 候选参数类型: ["ref$<country_core::resources::walls::decorator::DecoratorId>", "ref$<country_core::resources::walls::decorator::DecoratorId>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<u32>,ref$<u32>)
// 候选参数类型: ["ref$<u32>", "ref$<u32>"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass
// 信息 ID: 29442e3b6d0b02f4fa11546f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0> @ 0x395f21 -> 0xae7cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x39a6f5 -> 0xae7cf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure$0
// 信息 ID: 2ff7cf350c7cc94f01fa483e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass; module=556, inline_record=27000; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure$0::closure$0
// 信息 ID: 4d4ea954cac6bd974441fd3a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure$0::closure_env$0 <- rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure$0::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure$0::closure_env$0 <- rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure$0::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure$0::closure_env$0>; module=1655, inline_record=112920; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure$0::closure$1
// 信息 ID: aac0db97d43ceb598c8ca2d2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure$0::closure_env$1 <- rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure$0::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure$0::closure_env$1 <- rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure$0::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure$0::closure_env$1>; module=1655, inline_record=34784; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass
// 信息 ID: 4267a2057e697617bf764a34
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x39a6b8 -> 0xae76c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure$0
// 信息 ID: d7b552f232f4d946440fdc44
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0 <- rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0 <- rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0>; module=1655, inline_record=68200; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure$0::closure$0
// 信息 ID: 967f25aa90e5f10dbab0c443
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure$0::closure_env$0 <- core::slice::sort::shared::pivot::median3_rec<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure$0::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure$0::closure_env$0 <- core::slice::sort::stable::driftsort_main<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure$0::closure_env$0>,alloc::vec::Vec<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure$0::closure_env$0 <- core::slice::sort::shared::smallsort::sort4_stable<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure$0::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure$0::closure_env$0 <- core::slice::sort::shared::smallsort::insertion_sort_shift_left<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure$0::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure$0::closure_env$0 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure$0::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure$0::closure_env$0 <- core::slice::sort::stable::drift::sort<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure$0::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure$0::closure_env$0 <- core::slice::sort::stable::quicksort::quicksort<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure$0::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 25 条；闭包环境类型引用 7 条。
// [PDB 内联宿主] core::slice::sort::shared::pivot::median3_rec<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure$0::closure_env$0> >; module=528, inline_record=61484; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::pivot::median3_rec<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure$0::closure_env$0> >; module=528, inline_record=61676; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::pivot::median3_rec<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure$0::closure_env$0> >; module=528, inline_record=61864; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure$0::closure_env$0> >; module=1639, inline_record=11144; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure$0::closure_env$0> >; module=1639, inline_record=11288; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure$0::closure_env$0> >; module=1639, inline_record=11532; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure$0::closure_env$0> >; module=1639, inline_record=11696; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure$0::closure_env$0> >; module=1639, inline_record=11956; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
