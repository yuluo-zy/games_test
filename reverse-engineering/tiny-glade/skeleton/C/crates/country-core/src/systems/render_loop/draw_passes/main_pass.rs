// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/render_loop/draw_passes/main_pass.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 25e2d5205b35bacb4cf1701c4842e19c0a8665cfe710799f481793c5a769818c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: main_pass.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 517, 1655
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
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use typed_index_collections; // namespace candidate
//   package identities: [{"name":"typed-index-collections","source":"registry+https://github.com/rust-lang/crates.io-index","version":"3.4.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::main_pass::main_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":517,"record_offset":35652,"rva":10167328,"va":5378876448}
// 编译实例 RVAs: 0x9b2420
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f4de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13902331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378876448,"raw_hex":"e8fb21d400","size":5,"soft_fail":false,"target_va":5392780768,"va":5378878432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417554b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14297980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378876448,"raw_hex":"e87c2bda00","size":5,"soft_fail":false,"target_va":5393175728,"va":5378877743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141759c60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14316225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378876448,"raw_hex":"e8c172da00","size":5,"soft_fail":false,"target_va":5393194080,"va":5378877850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14175de30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14333771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378876448,"raw_hex":"e84bb7da00","size":5,"soft_fail":false,"target_va":5393210928,"va":5378877152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417610c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14345921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378876448,"raw_hex":"e8c1e6da00","size":5,"soft_fail":false,"target_va":5393223872,"va":5378877946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141762170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14350754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378876448,"raw_hex":"e8a2f9da00","size":5,"soft_fail":false,"target_va":5393228144,"va":5378877385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417695f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14380692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378876448,"raw_hex":"e8946edb00","size":5,"soft_fail":false,"target_va":5393257968,"va":5378877271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37506887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378876448,"raw_hex":"e8474f3c02","size":5,"soft_fail":false,"target_va":5416385024,"va":5378878132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37493877","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378876448,"raw_hex":"e8751c3c02","size":5,"soft_fail":false,"target_va":5416370864,"va":5378876982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37743281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378876448,"raw_hex":"e8b1ea3f02","size":5,"soft_fail":false,"target_va":5416621408,"va":5378878122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37743488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378876448,"raw_hex":"e880eb3f02","size":5,"soft_fail":false,"target_va":5416621600,"va":5378878107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378876448,"raw_hex":"e839860000","size":5,"soft_fail":false,"target_va":5378910912,"va":5378876546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t37743155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378876448,"raw_hex":"e833ea3f02","size":5,"soft_fail":false,"target_va":5416621376,"va":5378878216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5086864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378876448,"raw_hex":"e8909e4d00","size":5,"soft_fail":false,"target_va":5383963792,"va":5378876923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5095371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378876448,"raw_hex":"e8cbbf4d00","size":5,"soft_fail":false,"target_va":5383972224,"va":5378876848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5086843","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378876448,"raw_hex":"e87b9e4d00","size":5,"soft_fail":false,"target_va":5383964912,"va":5378878064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5086305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378876448,"raw_hex":"e8619c4d00","size":5,"soft_fail":false,"target_va":5383962816,"va":5378876506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37741174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378876448,"raw_hex":"e876e23f02","size":5,"soft_fail":false,"target_va":5416619360,"va":5378878181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":40456,"parent_record_offset":35652,"ancestor_record_offset":35652,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":517,"record_offset":39648,"parent_record_offset":35652,"ancestor_record_offset":35652,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":517,"record_offset":35824,"parent_record_offset":35652,"ancestor_record_offset":35652,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":517,"record_offset":36212,"parent_record_offset":35652,"ancestor_record_offset":35652,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::label
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":517,"record_offset":37224,"parent_record_offset":35652,"ancestor_record_offset":35652,"inlinee":16150,"source_line":436,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":517,"record_offset":37416,"parent_record_offset":37384,"ancestor_record_offset":35652,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":517,"record_offset":37464,"parent_record_offset":37440,"ancestor_record_offset":35652,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":517,"record_offset":37440,"parent_record_offset":37416,"ancestor_record_offset":35652,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":37384,"parent_record_offset":37336,"ancestor_record_offset":35652,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":38988,"parent_record_offset":38964,"ancestor_record_offset":35652,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":38964,"parent_record_offset":38924,"ancestor_record_offset":35652,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":37304,"parent_record_offset":37272,"ancestor_record_offset":35652,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":37336,"parent_record_offset":37304,"ancestor_record_offset":35652,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":38924,"parent_record_offset":38900,"ancestor_record_offset":35652,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":517,"record_offset":37720,"parent_record_offset":37692,"ancestor_record_offset":35652,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":517,"record_offset":39284,"parent_record_offset":39260,"ancestor_record_offset":35652,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":517,"record_offset":39260,"parent_record_offset":39236,"ancestor_record_offset":35652,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":517,"record_offset":39208,"parent_record_offset":39176,"ancestor_record_offset":35652,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":517,"record_offset":39152,"parent_record_offset":38644,"ancestor_record_offset":35652,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":517,"record_offset":39176,"parent_record_offset":39152,"ancestor_record_offset":35652,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":517,"record_offset":38672,"parent_record_offset":38644,"ancestor_record_offset":35652,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":517,"record_offset":39236,"parent_record_offset":39176,"ancestor_record_offset":35652,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":517,"record_offset":38720,"parent_record_offset":38696,"ancestor_record_offset":35652,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":517,"record_offset":38696,"parent_record_offset":38672,"ancestor_record_offset":35652,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":517,"record_offset":38900,"parent_record_offset":38876,"ancestor_record_offset":35652,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":517,"record_offset":37692,"parent_record_offset":37660,"ancestor_record_offset":35652,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":517,"record_offset":38644,"parent_record_offset":37692,"ancestor_record_offset":35652,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":517,"record_offset":37744,"parent_record_offset":37720,"ancestor_record_offset":35652,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":517,"record_offset":38392,"parent_record_offset":38368,"ancestor_record_offset":35652,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":517,"record_offset":38420,"parent_record_offset":38392,"ancestor_record_offset":35652,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":517,"record_offset":39976,"parent_record_offset":39948,"ancestor_record_offset":35652,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":517,"record_offset":40004,"parent_record_offset":39976,"ancestor_record_offset":35652,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":517,"record_offset":39924,"parent_record_offset":39896,"ancestor_record_offset":35652,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":517,"record_offset":39948,"parent_record_offset":39924,"ancestor_record_offset":35652,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":517,"record_offset":40064,"parent_record_offset":39948,"ancestor_record_offset":35652,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":517,"record_offset":40176,"parent_record_offset":40152,"ancestor_record_offset":35652,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":37960,"parent_record_offset":37936,"ancestor_record_offset":35652,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":38044,"parent_record_offset":38020,"ancestor_record_offset":35652,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":517,"record_offset":40028,"parent_record_offset":40004,"ancestor_record_offset":35652,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":517,"record_offset":38192,"parent_record_offset":38164,"ancestor_record_offset":35652,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":517,"record_offset":38164,"parent_record_offset":38136,"ancestor_record_offset":35652,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":517,"record_offset":38812,"parent_record_offset":38748,"ancestor_record_offset":35652,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":517,"record_offset":38836,"parent_record_offset":38812,"ancestor_record_offset":35652,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":517,"record_offset":38876,"parent_record_offset":38696,"ancestor_record_offset":35652,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":517,"record_offset":39336,"parent_record_offset":37660,"ancestor_record_offset":35652,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":517,"record_offset":36160,"parent_record_offset":36132,"ancestor_record_offset":35652,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":517,"record_offset":40412,"parent_record_offset":39756,"ancestor_record_offset":35652,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":37984,"parent_record_offset":37960,"ancestor_record_offset":35652,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":40128,"parent_record_offset":39896,"ancestor_record_offset":35652,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":517,"record_offset":38308,"parent_record_offset":38284,"ancestor_record_offset":35652,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":517,"record_offset":36632,"parent_record_offset":36608,"ancestor_record_offset":35652,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":517,"record_offset":36608,"parent_record_offset":36584,"ancestor_record_offset":35652,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":517,"record_offset":36816,"parent_record_offset":36788,"ancestor_record_offset":35652,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":37504,"parent_record_offset":37336,"ancestor_record_offset":35652,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":517,"record_offset":36916,"parent_record_offset":36888,"ancestor_record_offset":35652,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":517,"record_offset":36888,"parent_record_offset":36860,"ancestor_record_offset":35652,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":517,"record_offset":38748,"parent_record_offset":38720,"ancestor_record_offset":35652,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":517,"record_offset":38784,"parent_record_offset":38748,"ancestor_record_offset":35652,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":517,"record_offset":39104,"parent_record_offset":38924,"ancestor_record_offset":35652,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":517,"record_offset":36560,"parent_record_offset":36536,"ancestor_record_offset":35652,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":517,"record_offset":36584,"parent_record_offset":36560,"ancestor_record_offset":35652,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":517,"record_offset":36944,"parent_record_offset":36916,"ancestor_record_offset":35652,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":517,"record_offset":36536,"parent_record_offset":36420,"ancestor_record_offset":35652,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":517,"record_offset":35952,"parent_record_offset":35928,"ancestor_record_offset":35652,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":517,"record_offset":36248,"parent_record_offset":36212,"ancestor_record_offset":35652,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":517,"record_offset":36388,"parent_record_offset":36356,"ancestor_record_offset":35652,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":517,"record_offset":36356,"parent_record_offset":36320,"ancestor_record_offset":35652,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":517,"record_offset":36296,"parent_record_offset":36272,"ancestor_record_offset":35652,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":517,"record_offset":36320,"parent_record_offset":36296,"ancestor_record_offset":35652,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":517,"record_offset":36272,"parent_record_offset":36248,"ancestor_record_offset":35652,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":517,"record_offset":36860,"parent_record_offset":36320,"ancestor_record_offset":35652,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":517,"record_offset":35928,"parent_record_offset":35896,"ancestor_record_offset":35652,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":517,"record_offset":35896,"parent_record_offset":35868,"ancestor_record_offset":35652,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":517,"record_offset":36052,"parent_record_offset":36020,"ancestor_record_offset":35652,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":517,"record_offset":36108,"parent_record_offset":36052,"ancestor_record_offset":35652,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":517,"record_offset":36020,"parent_record_offset":35992,"ancestor_record_offset":35652,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":517,"record_offset":35868,"parent_record_offset":35824,"ancestor_record_offset":35652,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":517,"record_offset":36132,"parent_record_offset":36108,"ancestor_record_offset":35652,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":517,"record_offset":36420,"parent_record_offset":36388,"ancestor_record_offset":35652,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":517,"record_offset":36704,"parent_record_offset":36420,"ancestor_record_offset":35652,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":517,"record_offset":36676,"parent_record_offset":36420,"ancestor_record_offset":35652,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":517,"record_offset":36480,"parent_record_offset":36420,"ancestor_record_offset":35652,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::shader_refactoring_state::ShaderRefactoringState::disabled
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/shader_refactoring_state.rs
//     evidence={"module":517,"record_offset":39368,"parent_record_offset":37544,"ancestor_record_offset":35652,"inlinee":30177,"source_line":37,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/main_pass.rs
//     evidence={"module":517,"record_offset":37544,"parent_record_offset":37224,"ancestor_record_offset":35652,"inlinee":30175,"source_line":24,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":517,"record_offset":37660,"parent_record_offset":37544,"ancestor_record_offset":35652,"inlinee":12326,"source_line":34,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":517,"record_offset":38368,"parent_record_offset":38344,"ancestor_record_offset":35652,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":38104,"parent_record_offset":38076,"ancestor_record_offset":35652,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":38136,"parent_record_offset":38104,"ancestor_record_offset":35652,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":38076,"parent_record_offset":37856,"ancestor_record_offset":35652,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":37936,"parent_record_offset":37856,"ancestor_record_offset":35652,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":38496,"parent_record_offset":37856,"ancestor_record_offset":35652,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":38020,"parent_record_offset":37856,"ancestor_record_offset":35652,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":517,"record_offset":37780,"parent_record_offset":37744,"ancestor_record_offset":35652,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":517,"record_offset":38344,"parent_record_offset":38236,"ancestor_record_offset":35652,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":38284,"parent_record_offset":38260,"ancestor_record_offset":35652,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":38588,"parent_record_offset":37856,"ancestor_record_offset":35652,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":38260,"parent_record_offset":38236,"ancestor_record_offset":35652,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":37832,"parent_record_offset":37808,"ancestor_record_offset":35652,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":37808,"parent_record_offset":37780,"ancestor_record_offset":35652,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":37856,"parent_record_offset":37832,"ancestor_record_offset":35652,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":37908,"parent_record_offset":37856,"ancestor_record_offset":35652,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":38236,"parent_record_offset":37856,"ancestor_record_offset":35652,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":517,"record_offset":38468,"parent_record_offset":37856,"ancestor_record_offset":35652,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":517,"record_offset":39684,"parent_record_offset":39648,"ancestor_record_offset":35652,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":517,"record_offset":40520,"parent_record_offset":40488,"ancestor_record_offset":35652,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":517,"record_offset":40960,"parent_record_offset":40756,"ancestor_record_offset":35652,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":517,"record_offset":39824,"parent_record_offset":39796,"ancestor_record_offset":35652,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":517,"record_offset":39896,"parent_record_offset":39756,"ancestor_record_offset":35652,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":517,"record_offset":40100,"parent_record_offset":39896,"ancestor_record_offset":35652,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":517,"record_offset":39852,"parent_record_offset":39824,"ancestor_record_offset":35652,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":517,"record_offset":39756,"parent_record_offset":39720,"ancestor_record_offset":35652,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":517,"record_offset":39720,"parent_record_offset":39684,"ancestor_record_offset":35652,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] typed_index_collections::vec::TiVec::push
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"typed-index-collections","version":"3.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/typed-index-collections-3.4.0/src/vec.rs
//     evidence={"module":517,"record_offset":37272,"parent_record_offset":37224,"ancestor_record_offset":35652,"inlinee":14011,"source_line":489,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x75DF
// 首条位置: {"module":517,"inlinee":30175}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::shader_refactoring_state::ShaderRefactoringState::disabled
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/shader_refactoring_state.rs
//     evidence={"module":517,"record_offset":39368,"parent_record_offset":37544,"ancestor_record_offset":37544,"inlinee":30177,"source_line":37,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":517,"record_offset":37660,"parent_record_offset":37544,"ancestor_record_offset":37544,"inlinee":12326,"source_line":34,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":517,"record_offset":39012,"parent_record_offset":38988,"ancestor_record_offset":37544,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":517,"record_offset":39060,"parent_record_offset":39036,"ancestor_record_offset":37544,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":517,"record_offset":39036,"parent_record_offset":39012,"ancestor_record_offset":37544,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":38988,"parent_record_offset":38964,"ancestor_record_offset":37544,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":38964,"parent_record_offset":38924,"ancestor_record_offset":37544,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":38924,"parent_record_offset":38900,"ancestor_record_offset":37544,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":517,"record_offset":37720,"parent_record_offset":37692,"ancestor_record_offset":37544,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":517,"record_offset":39284,"parent_record_offset":39260,"ancestor_record_offset":37544,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":517,"record_offset":39260,"parent_record_offset":39236,"ancestor_record_offset":37544,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":517,"record_offset":39208,"parent_record_offset":39176,"ancestor_record_offset":37544,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":517,"record_offset":39152,"parent_record_offset":38644,"ancestor_record_offset":37544,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":517,"record_offset":39176,"parent_record_offset":39152,"ancestor_record_offset":37544,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":517,"record_offset":38672,"parent_record_offset":38644,"ancestor_record_offset":37544,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":517,"record_offset":39236,"parent_record_offset":39176,"ancestor_record_offset":37544,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":517,"record_offset":38720,"parent_record_offset":38696,"ancestor_record_offset":37544,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":517,"record_offset":38696,"parent_record_offset":38672,"ancestor_record_offset":37544,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":517,"record_offset":38900,"parent_record_offset":38876,"ancestor_record_offset":37544,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":517,"record_offset":37692,"parent_record_offset":37660,"ancestor_record_offset":37544,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":517,"record_offset":38644,"parent_record_offset":37692,"ancestor_record_offset":37544,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":517,"record_offset":37744,"parent_record_offset":37720,"ancestor_record_offset":37544,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":517,"record_offset":38392,"parent_record_offset":38368,"ancestor_record_offset":37544,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":517,"record_offset":38420,"parent_record_offset":38392,"ancestor_record_offset":37544,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":37960,"parent_record_offset":37936,"ancestor_record_offset":37544,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":38044,"parent_record_offset":38020,"ancestor_record_offset":37544,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":517,"record_offset":38192,"parent_record_offset":38164,"ancestor_record_offset":37544,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":517,"record_offset":38164,"parent_record_offset":38136,"ancestor_record_offset":37544,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":517,"record_offset":38812,"parent_record_offset":38748,"ancestor_record_offset":37544,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":517,"record_offset":38836,"parent_record_offset":38812,"ancestor_record_offset":37544,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":517,"record_offset":38876,"parent_record_offset":38696,"ancestor_record_offset":37544,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":517,"record_offset":39336,"parent_record_offset":37660,"ancestor_record_offset":37544,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":37984,"parent_record_offset":37960,"ancestor_record_offset":37544,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":517,"record_offset":38308,"parent_record_offset":38284,"ancestor_record_offset":37544,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":517,"record_offset":38748,"parent_record_offset":38720,"ancestor_record_offset":37544,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":517,"record_offset":38784,"parent_record_offset":38748,"ancestor_record_offset":37544,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":517,"record_offset":39104,"parent_record_offset":38924,"ancestor_record_offset":37544,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":517,"record_offset":38368,"parent_record_offset":38344,"ancestor_record_offset":37544,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":38104,"parent_record_offset":38076,"ancestor_record_offset":37544,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":38136,"parent_record_offset":38104,"ancestor_record_offset":37544,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":38076,"parent_record_offset":37856,"ancestor_record_offset":37544,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":37936,"parent_record_offset":37856,"ancestor_record_offset":37544,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":38496,"parent_record_offset":37856,"ancestor_record_offset":37544,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":38020,"parent_record_offset":37856,"ancestor_record_offset":37544,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":517,"record_offset":37780,"parent_record_offset":37744,"ancestor_record_offset":37544,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":517,"record_offset":38344,"parent_record_offset":38236,"ancestor_record_offset":37544,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":38284,"parent_record_offset":38260,"ancestor_record_offset":37544,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":38588,"parent_record_offset":37856,"ancestor_record_offset":37544,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":38260,"parent_record_offset":38236,"ancestor_record_offset":37544,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":37832,"parent_record_offset":37808,"ancestor_record_offset":37544,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":37808,"parent_record_offset":37780,"ancestor_record_offset":37544,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":37856,"parent_record_offset":37832,"ancestor_record_offset":37544,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":37908,"parent_record_offset":37856,"ancestor_record_offset":37544,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":517,"record_offset":38236,"parent_record_offset":37856,"ancestor_record_offset":37544,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":517,"record_offset":38468,"parent_record_offset":37856,"ancestor_record_offset":37544,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14053
// 首条位置: {"module":1655,"inlinee":82003}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14077
// 首条位置: {"module":1655,"inlinee":82039}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1405F
// 首条位置: {"module":1655,"inlinee":82015}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 99
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1403C
// 首条位置: {"module":1655,"inlinee":81980}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 118
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1404D
// 首条位置: {"module":1655,"inlinee":81997}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1405C
// 首条位置: {"module":1655,"inlinee":82012}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2
// 记录形式: compiled_procedure; instances=1
// 源码行: 149
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":517,"record_offset":28244,"rva":10164576,"va":5378873696}
// 编译实例 RVAs: 0x9b1960
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f4de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13905630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378873696,"raw_hex":"e8de2ed400","size":5,"soft_fail":false,"target_va":5392780768,"va":5378875133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141753bc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14294724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378873696,"raw_hex":"e8c41eda00","size":5,"soft_fail":false,"target_va":5393169344,"va":5378874615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141755a30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14302674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378873696,"raw_hex":"e8d23dda00","size":5,"soft_fail":false,"target_va":5393177136,"va":5378874457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417596e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14318298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378873696,"raw_hex":"e8da7ada00","size":5,"soft_fail":false,"target_va":5393192672,"va":5378874369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141761640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14350730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378873696,"raw_hex":"e88af9da00","size":5,"soft_fail":false,"target_va":5393225280,"va":5378874545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417666e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14371227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378873696,"raw_hex":"e89b49db00","size":5,"soft_fail":false,"target_va":5393245920,"va":5378874688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37510172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378873696,"raw_hex":"e81c5c3c02","size":5,"soft_fail":false,"target_va":5416385024,"va":5378874847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37496638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378873696,"raw_hex":"e83e273c02","size":5,"soft_fail":false,"target_va":5416370864,"va":5378874221,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37746566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378873696,"raw_hex":"e886f73f02","size":5,"soft_fail":false,"target_va":5416621408,"va":5378874837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378873696,"raw_hex":"e806910000","size":5,"soft_fail":false,"target_va":5378910912,"va":5378873781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t37746447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378873696,"raw_hex":"e80ff73f02","size":5,"soft_fail":false,"target_va":5416621376,"va":5378874924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5089619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378873696,"raw_hex":"e853a94d00","size":5,"soft_fail":false,"target_va":5383963792,"va":5378874168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5098123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378873696,"raw_hex":"e88bca4d00","size":5,"soft_fail":false,"target_va":5383972224,"va":5378874096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5090110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378873696,"raw_hex":"e83eab4d00","size":5,"soft_fail":false,"target_va":5383964912,"va":5378874797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5089067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378873696,"raw_hex":"e82ba74d00","size":5,"soft_fail":false,"target_va":5383962816,"va":5378873744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37744466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378873696,"raw_hex":"e852ef3f02","size":5,"soft_fail":false,"target_va":5416619360,"va":5378874889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":31288,"parent_record_offset":28244,"ancestor_record_offset":28244,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":517,"record_offset":30480,"parent_record_offset":28244,"ancestor_record_offset":28244,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":517,"record_offset":28420,"parent_record_offset":28244,"ancestor_record_offset":28244,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":517,"record_offset":28808,"parent_record_offset":28244,"ancestor_record_offset":28244,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::label
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":517,"record_offset":29820,"parent_record_offset":28244,"ancestor_record_offset":28244,"inlinee":16150,"source_line":436,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":517,"record_offset":30012,"parent_record_offset":29980,"ancestor_record_offset":28244,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":517,"record_offset":30060,"parent_record_offset":30036,"ancestor_record_offset":28244,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":517,"record_offset":30036,"parent_record_offset":30012,"ancestor_record_offset":28244,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":29980,"parent_record_offset":29932,"ancestor_record_offset":28244,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":29900,"parent_record_offset":29868,"ancestor_record_offset":28244,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":517,"record_offset":29932,"parent_record_offset":29900,"ancestor_record_offset":28244,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":517,"record_offset":30808,"parent_record_offset":30780,"ancestor_record_offset":28244,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":517,"record_offset":30836,"parent_record_offset":30808,"ancestor_record_offset":28244,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":517,"record_offset":30756,"parent_record_offset":30728,"ancestor_record_offset":28244,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":517,"record_offset":30780,"parent_record_offset":30756,"ancestor_record_offset":28244,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":517,"record_offset":30896,"parent_record_offset":30780,"ancestor_record_offset":28244,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":517,"record_offset":31008,"parent_record_offset":30984,"ancestor_record_offset":28244,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":517,"record_offset":30860,"parent_record_offset":30836,"ancestor_record_offset":28244,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":517,"record_offset":30628,"parent_record_offset":30588,"ancestor_record_offset":28244,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":517,"record_offset":28756,"parent_record_offset":28728,"ancestor_record_offset":28244,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":517,"record_offset":31244,"parent_record_offset":30588,"ancestor_record_offset":28244,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":30960,"parent_record_offset":30728,"ancestor_record_offset":28244,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":517,"record_offset":29228,"parent_record_offset":29204,"ancestor_record_offset":28244,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":517,"record_offset":29204,"parent_record_offset":29180,"ancestor_record_offset":28244,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":517,"record_offset":29412,"parent_record_offset":29384,"ancestor_record_offset":28244,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":30100,"parent_record_offset":29932,"ancestor_record_offset":28244,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":517,"record_offset":29512,"parent_record_offset":29484,"ancestor_record_offset":28244,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":517,"record_offset":29484,"parent_record_offset":29456,"ancestor_record_offset":28244,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":517,"record_offset":29156,"parent_record_offset":29132,"ancestor_record_offset":28244,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":517,"record_offset":29180,"parent_record_offset":29156,"ancestor_record_offset":28244,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":517,"record_offset":29540,"parent_record_offset":29512,"ancestor_record_offset":28244,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":517,"record_offset":29132,"parent_record_offset":29016,"ancestor_record_offset":28244,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":517,"record_offset":28548,"parent_record_offset":28524,"ancestor_record_offset":28244,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":517,"record_offset":28844,"parent_record_offset":28808,"ancestor_record_offset":28244,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":517,"record_offset":28984,"parent_record_offset":28952,"ancestor_record_offset":28244,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":517,"record_offset":28952,"parent_record_offset":28916,"ancestor_record_offset":28244,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":517,"record_offset":28892,"parent_record_offset":28868,"ancestor_record_offset":28244,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":517,"record_offset":28916,"parent_record_offset":28892,"ancestor_record_offset":28244,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":517,"record_offset":28868,"parent_record_offset":28844,"ancestor_record_offset":28244,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":517,"record_offset":29456,"parent_record_offset":28916,"ancestor_record_offset":28244,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":517,"record_offset":28524,"parent_record_offset":28492,"ancestor_record_offset":28244,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":517,"record_offset":28492,"parent_record_offset":28464,"ancestor_record_offset":28244,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":517,"record_offset":28648,"parent_record_offset":28616,"ancestor_record_offset":28244,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":517,"record_offset":28704,"parent_record_offset":28648,"ancestor_record_offset":28244,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":517,"record_offset":28616,"parent_record_offset":28588,"ancestor_record_offset":28244,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":517,"record_offset":28464,"parent_record_offset":28420,"ancestor_record_offset":28244,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":517,"record_offset":28728,"parent_record_offset":28704,"ancestor_record_offset":28244,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":517,"record_offset":29016,"parent_record_offset":28984,"ancestor_record_offset":28244,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":517,"record_offset":29300,"parent_record_offset":29016,"ancestor_record_offset":28244,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":517,"record_offset":29272,"parent_record_offset":29016,"ancestor_record_offset":28244,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":517,"record_offset":29076,"parent_record_offset":29016,"ancestor_record_offset":28244,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/main_pass.rs
//     evidence={"module":517,"record_offset":30140,"parent_record_offset":29820,"ancestor_record_offset":28244,"inlinee":30173,"source_line":158,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":517,"record_offset":30516,"parent_record_offset":30480,"ancestor_record_offset":28244,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":517,"record_offset":31352,"parent_record_offset":31320,"ancestor_record_offset":28244,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":517,"record_offset":31792,"parent_record_offset":31588,"ancestor_record_offset":28244,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":517,"record_offset":30656,"parent_record_offset":30628,"ancestor_record_offset":28244,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":517,"record_offset":30728,"parent_record_offset":30588,"ancestor_record_offset":28244,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":517,"record_offset":30932,"parent_record_offset":30728,"ancestor_record_offset":28244,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":517,"record_offset":30684,"parent_record_offset":30656,"ancestor_record_offset":28244,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":517,"record_offset":30588,"parent_record_offset":30552,"ancestor_record_offset":28244,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":517,"record_offset":30552,"parent_record_offset":30516,"ancestor_record_offset":28244,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] typed_index_collections::vec::TiVec::push
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"typed-index-collections","version":"3.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/typed-index-collections-3.4.0/src/vec.rs
//     evidence={"module":517,"record_offset":29868,"parent_record_offset":29820,"ancestor_record_offset":28244,"inlinee":14011,"source_line":489,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 158
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x75DD
// 首条位置: {"module":517,"inlinee":30173}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 169
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1404C
// 首条位置: {"module":1655,"inlinee":81996}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 190
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1403D
// 首条位置: {"module":1655,"inlinee":81981}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 210
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1405D
// 首条位置: {"module":1655,"inlinee":82013}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 228
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14032
// 首条位置: {"module":1655,"inlinee":81970}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 246
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14070
// 首条位置: {"module":1655,"inlinee":82032}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 258
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":517,"record_offset":32272,"rva":10166112,"va":5378875232}
// 编译实例 RVAs: 0x9b1f60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f4de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13904414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378875232,"raw_hex":"e81e2ad400","size":5,"soft_fail":false,"target_va":5392780768,"va":5378876349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141759160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14315363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378875232,"raw_hex":"e8636fda00","size":5,"soft_fail":false,"target_va":5393191264,"va":5378875896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141760580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14345014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378875232,"raw_hex":"e836e3da00","size":5,"soft_fail":false,"target_va":5393220992,"va":5378875973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141765330
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14365067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378875232,"raw_hex":"e88b31db00","size":5,"soft_fail":false,"target_va":5393240880,"va":5378875808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37508948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378875232,"raw_hex":"e854573c02","size":5,"soft_fail":false,"target_va":5416385024,"va":5378876071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37745342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378875232,"raw_hex":"e8bef23f02","size":5,"soft_fail":false,"target_va":5416621408,"va":5378876061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378875232,"raw_hex":"e8038b0000","size":5,"soft_fail":false,"target_va":5378910912,"va":5378875320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t37745227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378875232,"raw_hex":"e84bf23f02","size":5,"soft_fail":false,"target_va":5416621376,"va":5378876144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5088107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378875232,"raw_hex":"e86ba34d00","size":5,"soft_fail":false,"target_va":5383963792,"va":5378875680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5096611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378875232,"raw_hex":"e8a3c44d00","size":5,"soft_fail":false,"target_va":5383972224,"va":5378875608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5088886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378875232,"raw_hex":"e876a64d00","size":5,"soft_fail":false,"target_va":5383964912,"va":5378876021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5087532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378875232,"raw_hex":"e82ca14d00","size":5,"soft_fail":false,"target_va":5383962816,"va":5378875279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37743246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378875232,"raw_hex":"e88eea3f02","size":5,"soft_fail":false,"target_va":5416619360,"va":5378876109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":34668,"parent_record_offset":32272,"ancestor_record_offset":32272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":517,"record_offset":33860,"parent_record_offset":32272,"ancestor_record_offset":32272,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":517,"record_offset":32460,"parent_record_offset":32272,"ancestor_record_offset":32272,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":517,"record_offset":32848,"parent_record_offset":32272,"ancestor_record_offset":32272,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":517,"record_offset":34188,"parent_record_offset":34160,"ancestor_record_offset":32272,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":517,"record_offset":34216,"parent_record_offset":34188,"ancestor_record_offset":32272,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":517,"record_offset":34136,"parent_record_offset":34108,"ancestor_record_offset":32272,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":517,"record_offset":34160,"parent_record_offset":34136,"ancestor_record_offset":32272,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":517,"record_offset":34276,"parent_record_offset":34160,"ancestor_record_offset":32272,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":517,"record_offset":34388,"parent_record_offset":34364,"ancestor_record_offset":32272,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":517,"record_offset":34240,"parent_record_offset":34216,"ancestor_record_offset":32272,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":517,"record_offset":34008,"parent_record_offset":33968,"ancestor_record_offset":32272,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":517,"record_offset":32796,"parent_record_offset":32768,"ancestor_record_offset":32272,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":517,"record_offset":34624,"parent_record_offset":33968,"ancestor_record_offset":32272,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":34340,"parent_record_offset":34108,"ancestor_record_offset":32272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":517,"record_offset":33268,"parent_record_offset":33244,"ancestor_record_offset":32272,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":517,"record_offset":33244,"parent_record_offset":33220,"ancestor_record_offset":32272,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":517,"record_offset":33452,"parent_record_offset":33424,"ancestor_record_offset":32272,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":517,"record_offset":33552,"parent_record_offset":33524,"ancestor_record_offset":32272,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":517,"record_offset":33524,"parent_record_offset":33496,"ancestor_record_offset":32272,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":517,"record_offset":33196,"parent_record_offset":33172,"ancestor_record_offset":32272,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":517,"record_offset":33220,"parent_record_offset":33196,"ancestor_record_offset":32272,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":517,"record_offset":33580,"parent_record_offset":33552,"ancestor_record_offset":32272,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":517,"record_offset":33172,"parent_record_offset":33056,"ancestor_record_offset":32272,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":517,"record_offset":32588,"parent_record_offset":32564,"ancestor_record_offset":32272,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":517,"record_offset":32884,"parent_record_offset":32848,"ancestor_record_offset":32272,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":517,"record_offset":33024,"parent_record_offset":32992,"ancestor_record_offset":32272,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":517,"record_offset":32992,"parent_record_offset":32956,"ancestor_record_offset":32272,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":517,"record_offset":32932,"parent_record_offset":32908,"ancestor_record_offset":32272,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":517,"record_offset":32956,"parent_record_offset":32932,"ancestor_record_offset":32272,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":517,"record_offset":32908,"parent_record_offset":32884,"ancestor_record_offset":32272,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":517,"record_offset":33496,"parent_record_offset":32956,"ancestor_record_offset":32272,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":517,"record_offset":32564,"parent_record_offset":32532,"ancestor_record_offset":32272,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":517,"record_offset":32532,"parent_record_offset":32504,"ancestor_record_offset":32272,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":517,"record_offset":32688,"parent_record_offset":32656,"ancestor_record_offset":32272,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":517,"record_offset":32744,"parent_record_offset":32688,"ancestor_record_offset":32272,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":517,"record_offset":32656,"parent_record_offset":32628,"ancestor_record_offset":32272,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":517,"record_offset":32504,"parent_record_offset":32460,"ancestor_record_offset":32272,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":517,"record_offset":32768,"parent_record_offset":32744,"ancestor_record_offset":32272,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":517,"record_offset":33056,"parent_record_offset":33024,"ancestor_record_offset":32272,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":517,"record_offset":33340,"parent_record_offset":33056,"ancestor_record_offset":32272,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":517,"record_offset":33312,"parent_record_offset":33056,"ancestor_record_offset":32272,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":517,"record_offset":33116,"parent_record_offset":33056,"ancestor_record_offset":32272,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":517,"record_offset":33896,"parent_record_offset":33860,"ancestor_record_offset":32272,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":517,"record_offset":34732,"parent_record_offset":34700,"ancestor_record_offset":32272,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":517,"record_offset":35172,"parent_record_offset":34968,"ancestor_record_offset":32272,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":517,"record_offset":34036,"parent_record_offset":34008,"ancestor_record_offset":32272,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":517,"record_offset":34108,"parent_record_offset":33968,"ancestor_record_offset":32272,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":517,"record_offset":34312,"parent_record_offset":34108,"ancestor_record_offset":32272,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":517,"record_offset":34064,"parent_record_offset":34036,"ancestor_record_offset":32272,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":517,"record_offset":33968,"parent_record_offset":33932,"ancestor_record_offset":32272,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":517,"record_offset":33932,"parent_record_offset":33896,"ancestor_record_offset":32272,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 276
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1406C
// 首条位置: {"module":1655,"inlinee":82028}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 295
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1404B
// 首条位置: {"module":1655,"inlinee":81995}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 311
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1405A
// 首条位置: {"module":1655,"inlinee":82010}
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
// 完整结构及原始类型记录: main_pass.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: main_pass.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/main_pass.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: main_pass.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":5678848,"caller_aliases":{"alias_group_rva":5678848,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5682125,"position_among_mapped_render_targets":32,"mapped_render_target_count":74,"previous_callsite":5681057,"next_callsite":5682156,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/gpu_generate_draw_lists.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/generate_hzb.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5678848,"caller_aliases":{"alias_group_rva":5678848,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5683087,"position_among_mapped_render_targets":38,"mapped_render_target_count":74,"previous_callsite":5682785,"next_callsite":5683118,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/grass2.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/grass2.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5678848,"caller_aliases":{"alias_group_rva":5678848,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5686383,"position_among_mapped_render_targets":51,"mapped_render_target_count":74,"previous_callsite":5686328,"next_callsite":5686412,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/mouse_depth_value.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/half_res.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: main_pass.rs.enriched.json.gz
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
// 函数: country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass
// 信息 ID: b2029aa48350bbdba6a54df7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56c46f -> 0x9b1f60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass::closure$0
// 信息 ID: 5423fede113d717f4b9265d7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass::closure_env$0 <- rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass::closure_env$0 <- rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass::closure_env$0>; module=1655, inline_record=138416; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass::closure$1
// 信息 ID: 47a13c07a5272af2aa61bd02
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass::closure_env$1 <- rhapsody::rg::api::GraphApi::raster<2,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,2>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass::closure_env$1 <- rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<2,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,2>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] rhapsody::rg::api::GraphApi::raster<2,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,2>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass::closure_env$1>; module=1655, inline_record=63404; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass::closure$2
// 信息 ID: b9a887ee737d9194317d52fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass::closure_env$2 <- rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass::closure_env$2>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass::closure_env$2 <- rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass::closure_env$2>; module=1655, inline_record=111372; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::main_pass::main_pass
// 信息 ID: 5311d2d67ccc569fe9555e07
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56b3cd -> 0x9b2420 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0
// 信息 ID: 9089bc7bc859c734a7c1ade3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::main_pass::main_pass", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=517, inline_record=37544; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure$0
// 信息 ID: 75aad22d1a35cf44dfe55b47
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::main_pass::main_pass", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$0 <- rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$0 <- rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$0>; module=1655, inline_record=98040; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure$1
// 信息 ID: 60b1bf96a552e5e4e558d738
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::main_pass::main_pass", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$1 <- rhapsody::rg::api::GraphApi::raster<2,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,2>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$1 <- rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<2,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,2>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] rhapsody::rg::api::GraphApi::raster<2,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,2>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$1>; module=1655, inline_record=164204; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure$2
// 信息 ID: c2fd9c287613b80379378c42
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::main_pass::main_pass", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$2 <- rhapsody::rg::api::GraphApi::raster<2,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,2>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$2>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$2 <- rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<2,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,2>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] rhapsody::rg::api::GraphApi::raster<2,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,2>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$2>; module=1655, inline_record=119384; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure$3
// 信息 ID: d8c268c022277492f47c84c6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::main_pass::main_pass", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$3 <- rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$3>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$3 <- rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$3> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$3>; module=1655, inline_record=42076; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure$4
// 信息 ID: 3cd122352b53acd81ff81b6f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::main_pass::main_pass", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$4 <- rhapsody::rg::api::GraphApi::raster<2,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,2>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$4>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$4 <- rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<2,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,2>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$4> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] rhapsody::rg::api::GraphApi::raster<2,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,2>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$4>; module=1655, inline_record=66664; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure$5
// 信息 ID: 17154e0c4b1b7e12db0baad9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::main_pass::main_pass", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$5 <- rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$5>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$5 <- rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$5> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass::closure$0::closure_env$5>; module=1655, inline_record=114556; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2
// 信息 ID: 1ebc64850b7a269410465522
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56b78f -> 0x9b1960 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0
// 信息 ID: a1af5a5f49e01d13734affa1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=517, inline_record=30140; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure$0
// 信息 ID: 2e0d697e3eefff5e265fff44
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$0 <- rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$0 <- rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$0 <- rhapsody::rg::recorder::impl$8::new_in::render_impl<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$0>; module=1655, inline_record=65036; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure$1
// 信息 ID: 3b83b17a22409bec9a83c622
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$1 <- rhapsody::rg::api::GraphApi::raster<2,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,2>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$1 <- rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<2,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,2>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] rhapsody::rg::api::GraphApi::raster<2,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,2>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$1>; module=1655, inline_record=43708; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure$2
// 信息 ID: b33384854e68a248e5b7c079
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$2 <- rhapsody::rg::api::GraphApi::raster<2,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,2>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$2>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$2 <- rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<2,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,2>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] rhapsody::rg::api::GraphApi::raster<2,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,2>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$2>; module=1655, inline_record=116188; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure$3
// 信息 ID: 2cb90403732bf22a1ea64c46
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$3 <- rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$3>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$3 <- rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$3> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$3>; module=1655, inline_record=33244; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure$4
// 信息 ID: 95e10aedef6ccd7c654c45b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$4 <- rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$4>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$4 <- rhapsody::rg::node_builder::NodeBuilder::set_render_callback<rhapsody::rg::node_builder::draw_stream::impl$1::raster::closure_env$0<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$4> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] rhapsody::rg::api::GraphApi::raster<1,tuple$<array$<enum2$<rhapsody::rg::node_builder::framebuffer_binding::ColorAttachment>,1>,enum2$<rhapsody::rg::node_builder::framebuffer_binding::DepthAttachment> >,country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2::closure$0::closure_env$4>; module=1655, inline_record=145284; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
