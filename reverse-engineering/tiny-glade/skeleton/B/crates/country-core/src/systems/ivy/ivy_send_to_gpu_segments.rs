// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/ivy/ivy_send_to_gpu_segments.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: d0c6e8770bccb972e95d5fb792abdec102445290ee5e54032eb112f1ac381dc4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ivy_send_to_gpu_segments.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 559, 1585, 1734
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments
// 记录形式: compiled_procedure; instances=1
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":559,"record_offset":69000,"rva":11121888,"va":5379831008}
// 编译实例 RVAs: 0xa9b4e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: ivy_send_to_gpu_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4750551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379831008,"raw_hex":"e82983b7ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379832050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407b1810
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: ivy_send_to_gpu_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3056581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379831008,"raw_hex":"e83b5cd1ff","size":5,"soft_fail":false,"target_va":5376776208,"va":5379832784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140969720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_send_to_gpu_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1253566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379831008,"raw_hex":"e842dfecff","size":5,"soft_fail":false,"target_va":5378578208,"va":5379831769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140beefa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_send_to_gpu_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1390304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379831008,"raw_hex":"e8e0361500","size":5,"soft_fail":false,"target_va":5381222304,"va":5379831995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141309c30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_send_to_gpu_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8840289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379831008,"raw_hex":"e861e48600","size":5,"soft_fail":false,"target_va":5388672048,"va":5379831754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ivy_send_to_gpu_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31837804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379831008,"raw_hex":"e86ccee501","size":5,"soft_fail":false,"target_va":5411669968,"va":5379832159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ivy_send_to_gpu_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32015302","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379831008,"raw_hex":"e8c683e801","size":5,"soft_fail":false,"target_va":5411847152,"va":5379831845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31709362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379831008,"raw_hex":"e8b2d8e301","size":5,"soft_fail":false,"target_va":5411541584,"va":5379832217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31650170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379831008,"raw_hex":"e87af1e201","size":5,"soft_fail":false,"target_va":5411482433,"va":5379832258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31710477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379831008,"raw_hex":"e80ddde301","size":5,"soft_fail":false,"target_va":5411542880,"va":5379832398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-19515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379831008,"raw_hex":"e8c5b3ffff","size":5,"soft_fail":false,"target_va":5379811584,"va":5379831094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t31712191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379831008,"raw_hex":"e8bfe3e301","size":5,"soft_fail":false,"target_va":5411543360,"va":5379831164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1553913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379831008,"raw_hex":"e8f9b51700","size":5,"soft_fail":false,"target_va":5381385536,"va":5379831618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1559102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379831008,"raw_hex":"e83eca1700","size":5,"soft_fail":false,"target_va":5381390640,"va":5379831533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1554358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379831008,"raw_hex":"e8b6b71700","size":5,"soft_fail":false,"target_va":5381386480,"va":5379832117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1553641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379831008,"raw_hex":"e8e9b41700","size":5,"soft_fail":false,"target_va":5381384704,"va":5379831058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8992732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379831008,"raw_hex":"e8dc378900","size":5,"soft_fail":false,"target_va":5388825296,"va":5379832559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31704894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379831008,"raw_hex":"e83ec7e301","size":5,"soft_fail":false,"target_va":5411537104,"va":5379832205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5379831008,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5379831865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":559,"record_offset":72088,"parent_record_offset":69000,"ancestor_record_offset":69000,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":72732,"parent_record_offset":69000,"ancestor_record_offset":69000,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":559,"record_offset":74008,"parent_record_offset":69000,"ancestor_record_offset":69000,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":559,"record_offset":72184,"parent_record_offset":69000,"ancestor_record_offset":69000,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::InstanceBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":559,"record_offset":72424,"parent_record_offset":69000,"ancestor_record_offset":69000,"inlinee":25075,"source_line":357,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::ivy::ivy_storage::IvyStorage::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":559,"record_offset":71612,"parent_record_offset":69000,"ancestor_record_offset":69000,"inlinee":26834,"source_line":180,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":559,"record_offset":70628,"parent_record_offset":69000,"ancestor_record_offset":69000,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":559,"record_offset":69196,"parent_record_offset":69000,"ancestor_record_offset":69000,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":559,"record_offset":69652,"parent_record_offset":69000,"ancestor_record_offset":69000,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":72920,"parent_record_offset":72896,"ancestor_record_offset":69000,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":72896,"parent_record_offset":72804,"ancestor_record_offset":69000,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":72832,"parent_record_offset":72804,"ancestor_record_offset":69000,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":72804,"parent_record_offset":72780,"ancestor_record_offset":69000,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":72780,"parent_record_offset":72756,"ancestor_record_offset":69000,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":72116,"parent_record_offset":72088,"ancestor_record_offset":69000,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":559,"record_offset":70964,"parent_record_offset":70936,"ancestor_record_offset":69000,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":559,"record_offset":71016,"parent_record_offset":70992,"ancestor_record_offset":69000,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":559,"record_offset":70992,"parent_record_offset":70964,"ancestor_record_offset":69000,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":559,"record_offset":70912,"parent_record_offset":70884,"ancestor_record_offset":69000,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":559,"record_offset":70936,"parent_record_offset":70912,"ancestor_record_offset":69000,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":559,"record_offset":71108,"parent_record_offset":70936,"ancestor_record_offset":69000,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":559,"record_offset":71220,"parent_record_offset":71196,"ancestor_record_offset":69000,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":559,"record_offset":72252,"parent_record_offset":72212,"ancestor_record_offset":69000,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":559,"record_offset":71936,"parent_record_offset":71908,"ancestor_record_offset":69000,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":559,"record_offset":71996,"parent_record_offset":71964,"ancestor_record_offset":69000,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":559,"record_offset":72144,"parent_record_offset":72116,"ancestor_record_offset":69000,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":559,"record_offset":71040,"parent_record_offset":71016,"ancestor_record_offset":69000,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":559,"record_offset":72596,"parent_record_offset":72572,"ancestor_record_offset":69000,"inlinee":8266,"source_line":338,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":559,"record_offset":72864,"parent_record_offset":72832,"ancestor_record_offset":69000,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":559,"record_offset":69352,"parent_record_offset":69320,"ancestor_record_offset":69000,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":559,"record_offset":70780,"parent_record_offset":70736,"ancestor_record_offset":69000,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":559,"record_offset":69600,"parent_record_offset":69572,"ancestor_record_offset":69000,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":559,"record_offset":71568,"parent_record_offset":70736,"ancestor_record_offset":69000,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":71172,"parent_record_offset":70884,"ancestor_record_offset":69000,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":70064,"parent_record_offset":70040,"ancestor_record_offset":69000,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":70040,"parent_record_offset":70016,"ancestor_record_offset":69000,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":70248,"parent_record_offset":70220,"ancestor_record_offset":69000,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":71064,"parent_record_offset":71040,"ancestor_record_offset":69000,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":559,"record_offset":72628,"parent_record_offset":72524,"ancestor_record_offset":69000,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":559,"record_offset":72336,"parent_record_offset":72308,"ancestor_record_offset":69000,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":559,"record_offset":72364,"parent_record_offset":72336,"ancestor_record_offset":69000,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":559,"record_offset":72464,"parent_record_offset":72424,"ancestor_record_offset":69000,"inlinee":4614,"source_line":134,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":69992,"parent_record_offset":69968,"ancestor_record_offset":69000,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":70016,"parent_record_offset":69992,"ancestor_record_offset":69000,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":70320,"parent_record_offset":70292,"ancestor_record_offset":69000,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":69968,"parent_record_offset":69852,"ancestor_record_offset":69000,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":559,"record_offset":69320,"parent_record_offset":69292,"ancestor_record_offset":69000,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":559,"record_offset":69680,"parent_record_offset":69652,"ancestor_record_offset":69000,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":559,"record_offset":69820,"parent_record_offset":69788,"ancestor_record_offset":69000,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":559,"record_offset":69788,"parent_record_offset":69752,"ancestor_record_offset":69000,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":559,"record_offset":69728,"parent_record_offset":69704,"ancestor_record_offset":69000,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":559,"record_offset":69752,"parent_record_offset":69728,"ancestor_record_offset":69000,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":559,"record_offset":69704,"parent_record_offset":69680,"ancestor_record_offset":69000,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":559,"record_offset":70292,"parent_record_offset":69752,"ancestor_record_offset":69000,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":559,"record_offset":69292,"parent_record_offset":69260,"ancestor_record_offset":69000,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":559,"record_offset":69260,"parent_record_offset":69232,"ancestor_record_offset":69000,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":559,"record_offset":69452,"parent_record_offset":69420,"ancestor_record_offset":69000,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":559,"record_offset":69548,"parent_record_offset":69452,"ancestor_record_offset":69000,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":559,"record_offset":69420,"parent_record_offset":69396,"ancestor_record_offset":69000,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":559,"record_offset":72280,"parent_record_offset":72252,"ancestor_record_offset":69000,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":559,"record_offset":69232,"parent_record_offset":69196,"ancestor_record_offset":69000,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":559,"record_offset":69572,"parent_record_offset":69548,"ancestor_record_offset":69000,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":559,"record_offset":69852,"parent_record_offset":69820,"ancestor_record_offset":69000,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":559,"record_offset":70136,"parent_record_offset":69852,"ancestor_record_offset":69000,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":559,"record_offset":70108,"parent_record_offset":69852,"ancestor_record_offset":69000,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":559,"record_offset":69912,"parent_record_offset":69852,"ancestor_record_offset":69000,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":72672,"parent_record_offset":72424,"ancestor_record_offset":69000,"inlinee":7965,"source_line":2725,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":72696,"parent_record_offset":72672,"ancestor_record_offset":69000,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":559,"record_offset":72212,"parent_record_offset":72184,"ancestor_record_offset":69000,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":559,"record_offset":72492,"parent_record_offset":72424,"ancestor_record_offset":69000,"inlinee":11642,"source_line":156,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":559,"record_offset":72524,"parent_record_offset":72492,"ancestor_record_offset":69000,"inlinee":11643,"source_line":208,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":559,"record_offset":71908,"parent_record_offset":71844,"ancestor_record_offset":69000,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":559,"record_offset":71964,"parent_record_offset":71908,"ancestor_record_offset":69000,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":559,"record_offset":71696,"parent_record_offset":71668,"ancestor_record_offset":69000,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":71844,"parent_record_offset":71752,"ancestor_record_offset":69000,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":71724,"parent_record_offset":71696,"ancestor_record_offset":69000,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":71816,"parent_record_offset":71752,"ancestor_record_offset":69000,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":71788,"parent_record_offset":71752,"ancestor_record_offset":69000,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":71752,"parent_record_offset":71724,"ancestor_record_offset":69000,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":559,"record_offset":70664,"parent_record_offset":70628,"ancestor_record_offset":69000,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":559,"record_offset":73032,"parent_record_offset":73000,"ancestor_record_offset":69000,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":559,"record_offset":73532,"parent_record_offset":73272,"ancestor_record_offset":69000,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":559,"record_offset":70808,"parent_record_offset":70780,"ancestor_record_offset":69000,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":559,"record_offset":70884,"parent_record_offset":70736,"ancestor_record_offset":69000,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":559,"record_offset":71144,"parent_record_offset":70884,"ancestor_record_offset":69000,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":559,"record_offset":72572,"parent_record_offset":72524,"ancestor_record_offset":69000,"inlinee":8265,"source_line":8,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":559,"record_offset":71668,"parent_record_offset":71640,"ancestor_record_offset":69000,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":559,"record_offset":71640,"parent_record_offset":71612,"ancestor_record_offset":69000,"inlinee":4948,"source_line":440,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":559,"record_offset":70836,"parent_record_offset":70808,"ancestor_record_offset":69000,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":559,"record_offset":70736,"parent_record_offset":70700,"ancestor_record_offset":69000,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":559,"record_offset":70700,"parent_record_offset":70664,"ancestor_record_offset":69000,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11AD7
// 首条位置: {"module":1585,"inlinee":72407}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1734,"record_offset":125544,"parent_record_offset":124260,"ancestor_record_offset":124260,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1585,"record_offset":413992,"parent_record_offset":412700,"ancestor_record_offset":412700,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1585,"record_offset":413872,"parent_record_offset":412700,"ancestor_record_offset":412700,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::ivy::ivy_storage::Ivy::point_count
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":1585,"record_offset":413964,"parent_record_offset":412700,"ancestor_record_offset":412700,"inlinee":26766,"source_line":57,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":1585,"record_offset":412748,"parent_record_offset":412700,"ancestor_record_offset":412700,"inlinee":26470,"source_line":39,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1585,"record_offset":413512,"parent_record_offset":413488,"ancestor_record_offset":412700,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1585,"record_offset":413020,"parent_record_offset":412996,"ancestor_record_offset":412700,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1585,"record_offset":413104,"parent_record_offset":413080,"ancestor_record_offset":412700,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1585,"record_offset":413928,"parent_record_offset":413896,"ancestor_record_offset":412700,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1585,"record_offset":413044,"parent_record_offset":413020,"ancestor_record_offset":412700,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1585,"record_offset":413580,"parent_record_offset":412940,"ancestor_record_offset":412700,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1734,"record_offset":125568,"parent_record_offset":125544,"ancestor_record_offset":124260,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1585,"record_offset":413252,"parent_record_offset":413224,"ancestor_record_offset":412700,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1585,"record_offset":413224,"parent_record_offset":413196,"ancestor_record_offset":412700,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1585,"record_offset":413372,"parent_record_offset":413348,"ancestor_record_offset":412700,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1585,"record_offset":413536,"parent_record_offset":413512,"ancestor_record_offset":412700,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ivy/ivy_send_to_gpu_segments.rs
//     evidence={"module":1585,"record_offset":413896,"parent_record_offset":413872,"ancestor_record_offset":412700,"inlinee":72408,"source_line":53,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1585,"record_offset":413164,"parent_record_offset":413136,"ancestor_record_offset":412700,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1585,"record_offset":413196,"parent_record_offset":413164,"ancestor_record_offset":412700,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1585,"record_offset":413136,"parent_record_offset":412940,"ancestor_record_offset":412700,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1585,"record_offset":412996,"parent_record_offset":412940,"ancestor_record_offset":412700,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1585,"record_offset":413608,"parent_record_offset":412940,"ancestor_record_offset":412700,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1585,"record_offset":413080,"parent_record_offset":412940,"ancestor_record_offset":412700,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1585,"record_offset":413760,"parent_record_offset":412940,"ancestor_record_offset":412700,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1585,"record_offset":413488,"parent_record_offset":413464,"ancestor_record_offset":412700,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1585,"record_offset":412820,"parent_record_offset":412792,"ancestor_record_offset":412700,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1585,"record_offset":412848,"parent_record_offset":412820,"ancestor_record_offset":412700,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1585,"record_offset":413464,"parent_record_offset":413296,"ancestor_record_offset":412700,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1585,"record_offset":413800,"parent_record_offset":412848,"ancestor_record_offset":412700,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1585,"record_offset":413404,"parent_record_offset":413320,"ancestor_record_offset":412700,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1585,"record_offset":413348,"parent_record_offset":413320,"ancestor_record_offset":412700,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1585,"record_offset":413700,"parent_record_offset":412940,"ancestor_record_offset":412700,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1585,"record_offset":413320,"parent_record_offset":413296,"ancestor_record_offset":412700,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1585,"record_offset":412912,"parent_record_offset":412884,"ancestor_record_offset":412700,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1585,"record_offset":412884,"parent_record_offset":412848,"ancestor_record_offset":412700,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1585,"record_offset":412940,"parent_record_offset":412912,"ancestor_record_offset":412700,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1585,"record_offset":413732,"parent_record_offset":412940,"ancestor_record_offset":412700,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1585,"record_offset":413296,"parent_record_offset":412940,"ancestor_record_offset":412700,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1585,"record_offset":412792,"parent_record_offset":412748,"ancestor_record_offset":412700,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1585,"record_offset":413840,"parent_record_offset":412748,"ancestor_record_offset":412700,"inlinee":26471,"source_line":165,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11AD8
// 首条位置: {"module":1585,"inlinee":72408}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1585,"record_offset":413928,"parent_record_offset":413896,"ancestor_record_offset":413896,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11AD5
// 首条位置: {"module":1585,"inlinee":72405}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::collections::vec_deque::impl$12::index
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1585,"record_offset":408544,"parent_record_offset":408500,"ancestor_record_offset":408500,"inlinee":11302,"source_line":2924,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::to_array
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1585,"record_offset":409100,"parent_record_offset":408500,"ancestor_record_offset":408500,"inlinee":44691,"source_line":121,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1585,"record_offset":408668,"parent_record_offset":408640,"ancestor_record_offset":408500,"inlinee":4735,"source_line":769,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1585,"record_offset":408572,"parent_record_offset":408544,"ancestor_record_offset":408500,"inlinee":4910,"source_line":685,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1585,"record_offset":408780,"parent_record_offset":408572,"ancestor_record_offset":408500,"inlinee":4911,"source_line":161,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1585,"record_offset":408612,"parent_record_offset":408572,"ancestor_record_offset":408500,"inlinee":4738,"source_line":216,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1585,"record_offset":408640,"parent_record_offset":408612,"ancestor_record_offset":408500,"inlinee":4739,"source_line":211,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1585,"record_offset":408712,"parent_record_offset":408668,"ancestor_record_offset":408500,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1585,"record_offset":408816,"parent_record_offset":408780,"ancestor_record_offset":408500,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1585,"record_offset":408736,"parent_record_offset":408712,"ancestor_record_offset":408500,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1585,"record_offset":408864,"parent_record_offset":408840,"ancestor_record_offset":408500,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1585,"record_offset":408840,"parent_record_offset":408816,"ancestor_record_offset":408500,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1585,"record_offset":409020,"parent_record_offset":408996,"ancestor_record_offset":408500,"inlinee":4914,"source_line":1980,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1585,"record_offset":411132,"parent_record_offset":411108,"ancestor_record_offset":411080,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1585,"record_offset":409064,"parent_record_offset":408940,"ancestor_record_offset":408500,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01585.jsonl.gz"}
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
// 完整结构及原始类型记录: ivy_send_to_gpu_segments.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::read_access_buffer", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<usize,country_core::resources::ivy::ivy_storage::Ivy>,core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0::closure_env$1>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo", "core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<usize,country_core::resources::ivy::ivy_storage::Ivy>,core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0::closure_env$1>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::InstanceBuffer<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo>", "constructor": "country_core::render::ssbo::InstanceBuffer", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::InstanceBuffer<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::InstanceBuffer<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer::new<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer::new", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo>", "constructor": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ivy_send_to_gpu_segments.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-camera/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ivy_send_to_gpu_segments.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 25; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// [文件级接口候选，具体函数归属 unknown] array$<f32,3> (*)(glam::f32::sse2::vec3a::Vec3A)
// 候选参数类型: ["glam::f32::sse2::vec3a::Vec3A"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_storage::IvyStorage>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_storage::IvyStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments
// 信息 ID: be91e62d92e2c22a04fb19dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] a6cc3567e30d18dfceb2432a: void (*)(bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)> > > @ 0x14d1dae -> 0xa9b4e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)> > @ 0x17cf8ec -> 0xa9b4e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0
// 信息 ID: 36b4450c48d2323c40c1b167
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<usize,country_core::resources::ivy::ivy_storage::Ivy>,core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0::closure_env$1>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure_env$0 <- core::ops::function::impls::impl$4::call_once<tuple$<ref$<country_core::resources::ivy::ivy_storage::Ivy> >,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<usize,country_core::resources::ivy::ivy_storage::Ivy>,core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0::closure_env$1>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure_env$0> >; module=1585, inline_record=412700; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<ref$<country_core::resources::ivy::ivy_storage::Ivy> >,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure_env$0>; module=1734, inline_record=124260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0::closure$0
// 信息 ID: ab72948bb392125bbaa40f5c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<usize,country_core::resources::ivy::ivy_storage::Ivy>,core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0::closure_env$1>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure_env$0> >; module=1585, inline_record=413896; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<ref$<country_core::resources::ivy::ivy_storage::Ivy> >,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure_env$0>; module=1734, inline_record=125424; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0::closure$1
// 信息 ID: bbd0b4908262e7e1ed126dae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<usize,country_core::resources::ivy::ivy_storage::Ivy>,core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0::closure_env$1>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<usize,country_core::resources::ivy::ivy_storage::Ivy>,core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0::closure_env$1>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure_env$0> >; module=1585, inline_record=408500; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<usize,country_core::resources::ivy::ivy_storage::Ivy>,core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0::closure_env$1>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure_env$0> >; module=1585, inline_record=410308; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<usize,country_core::resources::ivy::ivy_storage::Ivy>,core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0::closure_env$1>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure_env$0> >; module=1585, inline_record=411080; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<usize,country_core::resources::ivy::ivy_storage::Ivy>,core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0::closure_env$1>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure_env$0> >; module=1585, inline_record=414944; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<usize,country_core::resources::ivy::ivy_storage::Ivy>,core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0::closure_env$1>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure_env$0> >; module=1585, inline_record=415472; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<usize,country_core::resources::ivy::ivy_storage::Ivy>,core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0::closure_env$1>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure_env$0> >; module=1585, inline_record=415948; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
