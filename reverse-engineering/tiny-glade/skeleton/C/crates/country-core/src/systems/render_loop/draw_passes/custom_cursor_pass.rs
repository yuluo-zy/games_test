// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/render_loop/draw_passes/custom_cursor_pass.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 620abaa377d6c1eca05a864c1c45ff9b82cb32d6254624d0352d7c4f37ba4eb5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: custom_cursor_pass.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 556, 1655
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
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::custom_cursor_pass::custom_cursor_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":556,"record_offset":48408,"rva":11448400,"va":5380157520}
// 编译实例 RVAs: 0xaeb050
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141536d80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: custom_cursor_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10794925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380157520,"raw_hex":"e8adb7a400","size":5,"soft_fail":false,"target_va":5390953856,"va":5380158926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415466f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: custom_cursor_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10858703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380157520,"raw_hex":"e8cfb0a500","size":5,"soft_fail":false,"target_va":5391017712,"va":5380159004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417671e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: custom_cursor_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13089739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380157520,"raw_hex":"e8cbbbc700","size":5,"soft_fail":false,"target_va":5393248736,"va":5380158992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: custom_cursor_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36225887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380157520,"raw_hex":"e85fc32802","size":5,"soft_fail":false,"target_va":5416385024,"va":5380159132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36462281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380157520,"raw_hex":"e8c95e2c02","size":5,"soft_fail":false,"target_va":5416621408,"va":5380159122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t36463211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380157520,"raw_hex":"e86b622c02","size":5,"soft_fail":false,"target_va":5416621600,"va":5380158384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t12378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380157520,"raw_hex":"e85a300000","size":5,"soft_fail":false,"target_va":5380169984,"va":5380157601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t36462162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380157520,"raw_hex":"e8525e2c02","size":5,"soft_fail":false,"target_va":5416621376,"va":5380159209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3805840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380157520,"raw_hex":"e890123a00","size":5,"soft_fail":false,"target_va":5383963792,"va":5380157947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3814344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380157520,"raw_hex":"e8c8333a00","size":5,"soft_fail":false,"target_va":5383972224,"va":5380157875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3805855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380157520,"raw_hex":"e89f123a00","size":5,"soft_fail":false,"target_va":5383964912,"va":5380159052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3805247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380157520,"raw_hex":"e83f103a00","size":5,"soft_fail":false,"target_va":5383962816,"va":5380157564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36460181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380157520,"raw_hex":"e895562c02","size":5,"soft_fail":false,"target_va":5416619360,"va":5380159174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":556,"record_offset":52528,"parent_record_offset":48408,"ancestor_record_offset":48408,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":556,"record_offset":56928,"parent_record_offset":48408,"ancestor_record_offset":48408,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":556,"record_offset":57084,"parent_record_offset":48408,"ancestor_record_offset":48408,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::WithRg::run
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":556,"record_offset":56956,"parent_record_offset":48408,"ancestor_record_offset":48408,"inlinee":12358,"source_line":489,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::window::WindowSize::as_vec2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/window.rs
//     evidence={"module":556,"record_offset":53988,"parent_record_offset":48408,"ancestor_record_offset":48408,"inlinee":32182,"source_line":46,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":556,"record_offset":50820,"parent_record_offset":48408,"ancestor_record_offset":48408,"inlinee":12326,"source_line":34,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":556,"record_offset":50012,"parent_record_offset":48408,"ancestor_record_offset":48408,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":556,"record_offset":48612,"parent_record_offset":48408,"ancestor_record_offset":48408,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":556,"record_offset":49000,"parent_record_offset":48408,"ancestor_record_offset":48408,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":556,"record_offset":52172,"parent_record_offset":52148,"ancestor_record_offset":48408,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":556,"record_offset":52220,"parent_record_offset":52196,"ancestor_record_offset":48408,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":556,"record_offset":52196,"parent_record_offset":52172,"ancestor_record_offset":48408,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":556,"record_offset":52148,"parent_record_offset":52124,"ancestor_record_offset":48408,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":556,"record_offset":52124,"parent_record_offset":52084,"ancestor_record_offset":48408,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":556,"record_offset":52084,"parent_record_offset":52060,"ancestor_record_offset":48408,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":556,"record_offset":50880,"parent_record_offset":50852,"ancestor_record_offset":48408,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":556,"record_offset":52444,"parent_record_offset":52420,"ancestor_record_offset":48408,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":556,"record_offset":52420,"parent_record_offset":52396,"ancestor_record_offset":48408,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":556,"record_offset":52368,"parent_record_offset":52336,"ancestor_record_offset":48408,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":556,"record_offset":52312,"parent_record_offset":51804,"ancestor_record_offset":48408,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":556,"record_offset":52336,"parent_record_offset":52312,"ancestor_record_offset":48408,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":556,"record_offset":51832,"parent_record_offset":51804,"ancestor_record_offset":48408,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":556,"record_offset":52396,"parent_record_offset":52336,"ancestor_record_offset":48408,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":556,"record_offset":51880,"parent_record_offset":51856,"ancestor_record_offset":48408,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":556,"record_offset":51856,"parent_record_offset":51832,"ancestor_record_offset":48408,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":556,"record_offset":52060,"parent_record_offset":52036,"ancestor_record_offset":48408,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":556,"record_offset":50852,"parent_record_offset":50820,"ancestor_record_offset":48408,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":556,"record_offset":51804,"parent_record_offset":50852,"ancestor_record_offset":48408,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":556,"record_offset":50904,"parent_record_offset":50880,"ancestor_record_offset":48408,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":556,"record_offset":51552,"parent_record_offset":51528,"ancestor_record_offset":48408,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":556,"record_offset":51580,"parent_record_offset":51552,"ancestor_record_offset":48408,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":556,"record_offset":50340,"parent_record_offset":50312,"ancestor_record_offset":48408,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":556,"record_offset":50368,"parent_record_offset":50340,"ancestor_record_offset":48408,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":556,"record_offset":50288,"parent_record_offset":50260,"ancestor_record_offset":48408,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":556,"record_offset":50312,"parent_record_offset":50288,"ancestor_record_offset":48408,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":556,"record_offset":50428,"parent_record_offset":50312,"ancestor_record_offset":48408,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":556,"record_offset":50540,"parent_record_offset":50516,"ancestor_record_offset":48408,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":556,"record_offset":51120,"parent_record_offset":51096,"ancestor_record_offset":48408,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":556,"record_offset":51204,"parent_record_offset":51180,"ancestor_record_offset":48408,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":556,"record_offset":50392,"parent_record_offset":50368,"ancestor_record_offset":48408,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":556,"record_offset":51352,"parent_record_offset":51324,"ancestor_record_offset":48408,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":556,"record_offset":51324,"parent_record_offset":51296,"ancestor_record_offset":48408,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":556,"record_offset":50160,"parent_record_offset":50120,"ancestor_record_offset":48408,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":556,"record_offset":51996,"parent_record_offset":51972,"ancestor_record_offset":48408,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":556,"record_offset":52036,"parent_record_offset":51856,"ancestor_record_offset":48408,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":556,"record_offset":52496,"parent_record_offset":50820,"ancestor_record_offset":48408,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":556,"record_offset":48948,"parent_record_offset":48920,"ancestor_record_offset":48408,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":556,"record_offset":50776,"parent_record_offset":50120,"ancestor_record_offset":48408,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":556,"record_offset":51144,"parent_record_offset":51120,"ancestor_record_offset":48408,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":556,"record_offset":50492,"parent_record_offset":50260,"ancestor_record_offset":48408,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":556,"record_offset":51468,"parent_record_offset":51444,"ancestor_record_offset":48408,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":556,"record_offset":49420,"parent_record_offset":49396,"ancestor_record_offset":48408,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":556,"record_offset":49396,"parent_record_offset":49372,"ancestor_record_offset":48408,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":556,"record_offset":49604,"parent_record_offset":49576,"ancestor_record_offset":48408,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":556,"record_offset":49704,"parent_record_offset":49676,"ancestor_record_offset":48408,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":556,"record_offset":49676,"parent_record_offset":49648,"ancestor_record_offset":48408,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":556,"record_offset":51908,"parent_record_offset":51880,"ancestor_record_offset":48408,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":556,"record_offset":51944,"parent_record_offset":51908,"ancestor_record_offset":48408,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":556,"record_offset":52264,"parent_record_offset":52084,"ancestor_record_offset":48408,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":556,"record_offset":49348,"parent_record_offset":49324,"ancestor_record_offset":48408,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":556,"record_offset":49372,"parent_record_offset":49348,"ancestor_record_offset":48408,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":556,"record_offset":49732,"parent_record_offset":49704,"ancestor_record_offset":48408,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":556,"record_offset":49324,"parent_record_offset":49208,"ancestor_record_offset":48408,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":556,"record_offset":48740,"parent_record_offset":48716,"ancestor_record_offset":48408,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":556,"record_offset":49036,"parent_record_offset":49000,"ancestor_record_offset":48408,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":556,"record_offset":49176,"parent_record_offset":49144,"ancestor_record_offset":48408,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":556,"record_offset":49144,"parent_record_offset":49108,"ancestor_record_offset":48408,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":556,"record_offset":49084,"parent_record_offset":49060,"ancestor_record_offset":48408,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":556,"record_offset":49108,"parent_record_offset":49084,"ancestor_record_offset":48408,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":556,"record_offset":49060,"parent_record_offset":49036,"ancestor_record_offset":48408,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":556,"record_offset":49648,"parent_record_offset":49108,"ancestor_record_offset":48408,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":556,"record_offset":48716,"parent_record_offset":48684,"ancestor_record_offset":48408,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":556,"record_offset":48684,"parent_record_offset":48656,"ancestor_record_offset":48408,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":556,"record_offset":48840,"parent_record_offset":48808,"ancestor_record_offset":48408,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":556,"record_offset":48896,"parent_record_offset":48840,"ancestor_record_offset":48408,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":556,"record_offset":48808,"parent_record_offset":48780,"ancestor_record_offset":48408,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":556,"record_offset":48656,"parent_record_offset":48612,"ancestor_record_offset":48408,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":556,"record_offset":48920,"parent_record_offset":48896,"ancestor_record_offset":48408,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":556,"record_offset":49208,"parent_record_offset":49176,"ancestor_record_offset":48408,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":556,"record_offset":49492,"parent_record_offset":49208,"ancestor_record_offset":48408,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":556,"record_offset":49464,"parent_record_offset":49208,"ancestor_record_offset":48408,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":556,"record_offset":49268,"parent_record_offset":49208,"ancestor_record_offset":48408,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::fetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":556,"record_offset":56988,"parent_record_offset":56956,"ancestor_record_offset":48408,"inlinee":9087,"source_line":545,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::unfetch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":556,"record_offset":57052,"parent_record_offset":56956,"ancestor_record_offset":48408,"inlinee":9089,"source_line":552,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::draw_passes::custom_cursor_pass::custom_cursor_pass::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/custom_cursor_pass.rs
//     evidence={"module":556,"record_offset":57016,"parent_record_offset":56956,"ancestor_record_offset":48408,"inlinee":34762,"source_line":45,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":556,"record_offset":51528,"parent_record_offset":51504,"ancestor_record_offset":48408,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":556,"record_offset":51264,"parent_record_offset":51236,"ancestor_record_offset":48408,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":556,"record_offset":51296,"parent_record_offset":51264,"ancestor_record_offset":48408,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":556,"record_offset":51236,"parent_record_offset":51016,"ancestor_record_offset":48408,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":556,"record_offset":51096,"parent_record_offset":51016,"ancestor_record_offset":48408,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":556,"record_offset":51656,"parent_record_offset":51016,"ancestor_record_offset":48408,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":556,"record_offset":51180,"parent_record_offset":51016,"ancestor_record_offset":48408,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":556,"record_offset":50940,"parent_record_offset":50904,"ancestor_record_offset":48408,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":556,"record_offset":51504,"parent_record_offset":51396,"ancestor_record_offset":48408,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":51444,"parent_record_offset":51420,"ancestor_record_offset":48408,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":51748,"parent_record_offset":51016,"ancestor_record_offset":48408,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":51420,"parent_record_offset":51396,"ancestor_record_offset":48408,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":50992,"parent_record_offset":50968,"ancestor_record_offset":48408,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":50968,"parent_record_offset":50940,"ancestor_record_offset":48408,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":51016,"parent_record_offset":50992,"ancestor_record_offset":48408,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":51068,"parent_record_offset":51016,"ancestor_record_offset":48408,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":556,"record_offset":51396,"parent_record_offset":51016,"ancestor_record_offset":48408,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":556,"record_offset":51628,"parent_record_offset":51016,"ancestor_record_offset":48408,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":556,"record_offset":53724,"parent_record_offset":53700,"ancestor_record_offset":48408,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":556,"record_offset":50048,"parent_record_offset":50012,"ancestor_record_offset":48408,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":556,"record_offset":57148,"parent_record_offset":57116,"ancestor_record_offset":48408,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":556,"record_offset":57588,"parent_record_offset":57384,"ancestor_record_offset":48408,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":556,"record_offset":50188,"parent_record_offset":50160,"ancestor_record_offset":48408,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":556,"record_offset":50260,"parent_record_offset":50120,"ancestor_record_offset":48408,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":556,"record_offset":50464,"parent_record_offset":50260,"ancestor_record_offset":48408,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":556,"record_offset":50216,"parent_record_offset":50188,"ancestor_record_offset":48408,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":556,"record_offset":50120,"parent_record_offset":50084,"ancestor_record_offset":48408,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":556,"record_offset":50084,"parent_record_offset":50048,"ancestor_record_offset":48408,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::custom_cursor_pass::custom_cursor_pass::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x87CA
// 首条位置: {"module":556,"inlinee":34762}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::custom_cursor_pass::custom_cursor_pass::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14074
// 首条位置: {"module":1655,"inlinee":82036}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1655,"record_offset":149376,"parent_record_offset":148372,"ancestor_record_offset":148372,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_bindless_handle_by_name_hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/texture_library.rs
//     evidence={"module":1655,"record_offset":148432,"parent_record_offset":148372,"ancestor_record_offset":148372,"inlinee":30266,"source_line":429,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1655,"record_offset":149160,"parent_record_offset":149136,"ancestor_record_offset":148372,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1655,"record_offset":148664,"parent_record_offset":148640,"ancestor_record_offset":148372,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1655,"record_offset":148748,"parent_record_offset":148724,"ancestor_record_offset":148372,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1655,"record_offset":149200,"parent_record_offset":148560,"ancestor_record_offset":148372,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1655,"record_offset":148900,"parent_record_offset":148872,"ancestor_record_offset":148372,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1655,"record_offset":148872,"parent_record_offset":148844,"ancestor_record_offset":148372,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1655,"record_offset":148688,"parent_record_offset":148664,"ancestor_record_offset":148372,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1655,"record_offset":149020,"parent_record_offset":148996,"ancestor_record_offset":148372,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":1655,"record_offset":148808,"parent_record_offset":148780,"ancestor_record_offset":148372,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":1655,"record_offset":148844,"parent_record_offset":148808,"ancestor_record_offset":148372,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":1655,"record_offset":148780,"parent_record_offset":148560,"ancestor_record_offset":148372,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":1655,"record_offset":148640,"parent_record_offset":148560,"ancestor_record_offset":148372,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":1655,"record_offset":149228,"parent_record_offset":148560,"ancestor_record_offset":148372,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":1655,"record_offset":148724,"parent_record_offset":148560,"ancestor_record_offset":148372,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1655,"record_offset":149136,"parent_record_offset":149112,"ancestor_record_offset":148372,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1655,"record_offset":148484,"parent_record_offset":148460,"ancestor_record_offset":148372,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":1655,"record_offset":149112,"parent_record_offset":148944,"ancestor_record_offset":148372,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":149052,"parent_record_offset":148968,"ancestor_record_offset":148372,"inlinee":8504,"source_line":435,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1655,"record_offset":148996,"parent_record_offset":148968,"ancestor_record_offset":148372,"inlinee":4324,"source_line":261,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1655,"record_offset":149320,"parent_record_offset":148560,"ancestor_record_offset":148372,"inlinee":4339,"source_line":81,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1655,"record_offset":148968,"parent_record_offset":148944,"ancestor_record_offset":148372,"inlinee":4322,"source_line":734,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1655,"record_offset":148536,"parent_record_offset":148512,"ancestor_record_offset":148372,"inlinee":4302,"source_line":1156,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":148512,"parent_record_offset":148484,"ancestor_record_offset":148372,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":148560,"parent_record_offset":148536,"ancestor_record_offset":148372,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1655,"record_offset":148612,"parent_record_offset":148560,"ancestor_record_offset":148372,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":1655,"record_offset":148944,"parent_record_offset":148560,"ancestor_record_offset":148372,"inlinee":4321,"source_line":1166,"archive":"_pdb/C/modules/01655.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1655,"record_offset":148460,"parent_record_offset":148432,"ancestor_record_offset":148372,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01655.jsonl.gz"}
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
// 完整结构及原始类型记录: custom_cursor_pass.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::read_rg_buffer<country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::read_rg_buffer", "arguments": ["country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_read_rg_buffer<country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_read_rg_buffer", "arguments": ["country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0<country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0", "arguments": ["country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::draw_passes::custom_cursor_pass::CustomCursorSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: custom_cursor_pass.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/custom_cursor_pass.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: custom_cursor_pass.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":3767344,"caller_aliases":{"alias_group_rva":3767344,"count":1,"display_examples":["rhapsody::rg::compile::compile"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3778323,"position_among_mapped_render_targets":21,"mapped_render_target_count":24,"previous_callsite":3778293,"next_callsite":3778368,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/ui_2d_pass.rs"],"next_target_files":["crates/tiny-glade/src/render_loop.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: custom_cursor_pass.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::system_param::Local<country_core::systems::render_loop::find_crash::State>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::system_param::Local<country_core::systems::render_loop::find_crash::State>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::render_loop::find_crash::State>

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::custom_cursor_pass::custom_cursor_pass
// 信息 ID: 1376e53c67b22e0b11a74ecf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x39a713 -> 0xaeb050 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::custom_cursor_pass::custom_cursor_pass::closure$0
// 信息 ID: dbbca6b910228fdb1dbe07f2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::custom_cursor_pass::custom_cursor_pass", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::custom_cursor_pass::custom_cursor_pass; module=556, inline_record=57016; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::custom_cursor_pass::custom_cursor_pass::closure$0::closure$0
// 信息 ID: 50428889d5ed72462bb585ea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::custom_cursor_pass::custom_cursor_pass", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::custom_cursor_pass::custom_cursor_pass::closure$0::closure_env$0 <- rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::custom_cursor_pass::custom_cursor_pass::closure$0::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::custom_cursor_pass::custom_cursor_pass::closure$0::closure_env$0 <- rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::custom_cursor_pass::custom_cursor_pass::closure$0::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] rhapsody::rg::api::GraphApi::raster<1,array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::custom_cursor_pass::custom_cursor_pass::closure$0::closure_env$0>; module=1655, inline_record=148372; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
