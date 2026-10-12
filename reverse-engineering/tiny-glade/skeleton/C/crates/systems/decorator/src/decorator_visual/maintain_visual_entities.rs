// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/decorator/src/decorator_visual/maintain_visual_entities.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: ba23a50e083bc41251c818d95f53c3b5d5614a00db5c267e8bf1460216bafb41
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: maintain_visual_entities.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 474
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::maintain_visual_entities::clear_out_visual_entities
// 记录形式: compiled_procedure; instances=1
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":474,"record_offset":772,"rva":9231728,"va":5377940848}
// 编译实例 RVAs: 0x8cdd70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-2262838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377940848,"raw_hex":"e8ca78ddff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377941537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414aa440
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: maintain_visual_entities.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12435989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377940848,"raw_hex":"e815c2bd00","size":5,"soft_fail":false,"target_va":5390378048,"va":5377942054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::despawn
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6476986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377940848,"raw_hex":"e8bad46200","size":5,"soft_fail":false,"target_va":5384418976,"va":5377941985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38679242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377940848,"raw_hex":"e8ca324e02","size":5,"soft_fail":false,"target_va":5416621408,"va":5377942161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377940848,"raw_hex":"e8ab060000","size":5,"soft_fail":false,"target_va":5377942640,"va":5377940928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38679127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377940848,"raw_hex":"e857324e02","size":5,"soft_fail":false,"target_va":5416621376,"va":5377942244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6022512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377940848,"raw_hex":"e870e55b00","size":5,"soft_fail":false,"target_va":5383963792,"va":5377941275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6031019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377940848,"raw_hex":"e8ab065c00","size":5,"soft_fail":false,"target_va":5383972224,"va":5377941200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6022796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377940848,"raw_hex":"e88ce65b00","size":5,"soft_fail":false,"target_va":5383964912,"va":5377942111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6021912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377940848,"raw_hex":"e818e35b00","size":5,"soft_fail":false,"target_va":5383962816,"va":5377940899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38775520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377940848,"raw_hex":"e8e0aa4f02","size":5,"soft_fail":false,"target_va":5416717696,"va":5377942171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12424775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377940848,"raw_hex":"e84796bd00","size":5,"soft_fail":false,"target_va":5390367248,"va":5377942468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38677146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377940848,"raw_hex":"e89a2a4e02","size":5,"soft_fail":false,"target_va":5416619360,"va":5377942209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":474,"record_offset":3180,"parent_record_offset":772,"ancestor_record_offset":772,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":474,"record_offset":7200,"parent_record_offset":772,"ancestor_record_offset":772,"inlinee":10329,"source_line":442,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":474,"record_offset":3804,"parent_record_offset":772,"ancestor_record_offset":772,"inlinee":10940,"source_line":63,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":474,"record_offset":3772,"parent_record_offset":772,"ancestor_record_offset":772,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":474,"record_offset":7228,"parent_record_offset":772,"ancestor_record_offset":772,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":474,"record_offset":2372,"parent_record_offset":772,"ancestor_record_offset":772,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":474,"record_offset":976,"parent_record_offset":772,"ancestor_record_offset":772,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":474,"record_offset":1360,"parent_record_offset":772,"ancestor_record_offset":772,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual_state::DecoratorTransientVisualState::drain
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual_state.rs
//     evidence={"module":474,"record_offset":3240,"parent_record_offset":772,"ancestor_record_offset":772,"inlinee":27647,"source_line":46,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":474,"record_offset":4572,"parent_record_offset":4548,"ancestor_record_offset":772,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":474,"record_offset":4596,"parent_record_offset":4572,"ancestor_record_offset":772,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":474,"record_offset":4620,"parent_record_offset":4596,"ancestor_record_offset":772,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":474,"record_offset":4548,"parent_record_offset":4460,"ancestor_record_offset":772,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":474,"record_offset":4488,"parent_record_offset":4460,"ancestor_record_offset":772,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":474,"record_offset":4460,"parent_record_offset":4436,"ancestor_record_offset":772,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":474,"record_offset":4436,"parent_record_offset":4412,"ancestor_record_offset":772,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":474,"record_offset":6356,"parent_record_offset":6332,"ancestor_record_offset":772,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":474,"record_offset":4340,"parent_record_offset":4316,"ancestor_record_offset":772,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":474,"record_offset":4388,"parent_record_offset":4364,"ancestor_record_offset":772,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":474,"record_offset":4048,"parent_record_offset":4020,"ancestor_record_offset":772,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":474,"record_offset":3208,"parent_record_offset":3180,"ancestor_record_offset":772,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":474,"record_offset":2700,"parent_record_offset":2672,"ancestor_record_offset":772,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":474,"record_offset":2728,"parent_record_offset":2700,"ancestor_record_offset":772,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":474,"record_offset":2648,"parent_record_offset":2620,"ancestor_record_offset":772,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":474,"record_offset":2672,"parent_record_offset":2648,"ancestor_record_offset":772,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":474,"record_offset":2788,"parent_record_offset":2672,"ancestor_record_offset":772,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":474,"record_offset":2900,"parent_record_offset":2876,"ancestor_record_offset":772,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":474,"record_offset":3568,"parent_record_offset":3540,"ancestor_record_offset":772,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":474,"record_offset":3620,"parent_record_offset":3596,"ancestor_record_offset":772,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":474,"record_offset":3904,"parent_record_offset":3836,"ancestor_record_offset":772,"inlinee":10749,"source_line":752,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$20::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":474,"record_offset":3836,"parent_record_offset":3804,"ancestor_record_offset":772,"inlinee":10747,"source_line":518,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":474,"record_offset":5136,"parent_record_offset":5100,"ancestor_record_offset":772,"inlinee":10753,"source_line":380,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":474,"record_offset":5100,"parent_record_offset":3836,"ancestor_record_offset":772,"inlinee":10751,"source_line":49,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":474,"record_offset":4020,"parent_record_offset":3992,"ancestor_record_offset":772,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":474,"record_offset":2752,"parent_record_offset":2728,"ancestor_record_offset":772,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":474,"record_offset":4516,"parent_record_offset":4488,"ancestor_record_offset":772,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":474,"record_offset":5820,"parent_record_offset":5796,"ancestor_record_offset":772,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":474,"record_offset":5796,"parent_record_offset":5772,"ancestor_record_offset":772,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":474,"record_offset":2520,"parent_record_offset":2480,"ancestor_record_offset":772,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":474,"record_offset":6308,"parent_record_offset":6280,"ancestor_record_offset":772,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":474,"record_offset":3960,"parent_record_offset":3904,"ancestor_record_offset":772,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":474,"record_offset":4224,"parent_record_offset":4020,"ancestor_record_offset":772,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$48::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":474,"record_offset":6732,"parent_record_offset":6436,"ancestor_record_offset":772,"inlinee":10158,"source_line":2779,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":474,"record_offset":1308,"parent_record_offset":1280,"ancestor_record_offset":772,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":474,"record_offset":3136,"parent_record_offset":2480,"ancestor_record_offset":772,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":474,"record_offset":2852,"parent_record_offset":2620,"ancestor_record_offset":772,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":474,"record_offset":6380,"parent_record_offset":6356,"ancestor_record_offset":772,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":474,"record_offset":5552,"parent_record_offset":5528,"ancestor_record_offset":772,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":474,"record_offset":4124,"parent_record_offset":4048,"ancestor_record_offset":772,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":474,"record_offset":1780,"parent_record_offset":1756,"ancestor_record_offset":772,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":474,"record_offset":4156,"parent_record_offset":4048,"ancestor_record_offset":772,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":474,"record_offset":1756,"parent_record_offset":1732,"ancestor_record_offset":772,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":474,"record_offset":1964,"parent_record_offset":1936,"ancestor_record_offset":772,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":474,"record_offset":4184,"parent_record_offset":4156,"ancestor_record_offset":772,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":474,"record_offset":2064,"parent_record_offset":2036,"ancestor_record_offset":772,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":474,"record_offset":2036,"parent_record_offset":2008,"ancestor_record_offset":772,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":474,"record_offset":1708,"parent_record_offset":1684,"ancestor_record_offset":772,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":474,"record_offset":1732,"parent_record_offset":1708,"ancestor_record_offset":772,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":474,"record_offset":2092,"parent_record_offset":2064,"ancestor_record_offset":772,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":474,"record_offset":1684,"parent_record_offset":1568,"ancestor_record_offset":772,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":474,"record_offset":1104,"parent_record_offset":1080,"ancestor_record_offset":772,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":474,"record_offset":1396,"parent_record_offset":1360,"ancestor_record_offset":772,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":474,"record_offset":1536,"parent_record_offset":1504,"ancestor_record_offset":772,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":474,"record_offset":1504,"parent_record_offset":1468,"ancestor_record_offset":772,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":474,"record_offset":1444,"parent_record_offset":1420,"ancestor_record_offset":772,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":474,"record_offset":1468,"parent_record_offset":1444,"ancestor_record_offset":772,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":474,"record_offset":1420,"parent_record_offset":1396,"ancestor_record_offset":772,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":474,"record_offset":2008,"parent_record_offset":1468,"ancestor_record_offset":772,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":474,"record_offset":1080,"parent_record_offset":1048,"ancestor_record_offset":772,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":474,"record_offset":1048,"parent_record_offset":1020,"ancestor_record_offset":772,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":474,"record_offset":1204,"parent_record_offset":1172,"ancestor_record_offset":772,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":474,"record_offset":1256,"parent_record_offset":1204,"ancestor_record_offset":772,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":474,"record_offset":1172,"parent_record_offset":1144,"ancestor_record_offset":772,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":474,"record_offset":1020,"parent_record_offset":976,"ancestor_record_offset":772,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":474,"record_offset":1280,"parent_record_offset":1256,"ancestor_record_offset":772,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":474,"record_offset":1568,"parent_record_offset":1536,"ancestor_record_offset":772,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":474,"record_offset":1852,"parent_record_offset":1568,"ancestor_record_offset":772,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":474,"record_offset":1824,"parent_record_offset":1568,"ancestor_record_offset":772,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":474,"record_offset":1628,"parent_record_offset":1568,"ancestor_record_offset":772,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":474,"record_offset":5436,"parent_record_offset":5408,"ancestor_record_offset":772,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":474,"record_offset":5772,"parent_record_offset":5748,"ancestor_record_offset":772,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":474,"record_offset":5860,"parent_record_offset":5720,"ancestor_record_offset":772,"inlinee":4664,"source_line":37,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":474,"record_offset":5408,"parent_record_offset":5372,"ancestor_record_offset":772,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":474,"record_offset":3540,"parent_record_offset":3476,"ancestor_record_offset":772,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":474,"record_offset":3596,"parent_record_offset":3540,"ancestor_record_offset":772,"inlinee":7442,"source_line":101,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::drain
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":474,"record_offset":3296,"parent_record_offset":3268,"ancestor_record_offset":772,"inlinee":11322,"source_line":888,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$70::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":474,"record_offset":5280,"parent_record_offset":5240,"ancestor_record_offset":772,"inlinee":11038,"source_line":3485,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":474,"record_offset":5528,"parent_record_offset":5372,"ancestor_record_offset":772,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":474,"record_offset":3476,"parent_record_offset":3384,"ancestor_record_offset":772,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":474,"record_offset":5372,"parent_record_offset":5344,"ancestor_record_offset":772,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::drain
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":474,"record_offset":3324,"parent_record_offset":3296,"ancestor_record_offset":772,"inlinee":11323,"source_line":1442,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::drain_iter_from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":474,"record_offset":3696,"parent_record_offset":3324,"ancestor_record_offset":772,"inlinee":11324,"source_line":1457,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":474,"record_offset":3356,"parent_record_offset":3324,"ancestor_record_offset":772,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":474,"record_offset":3448,"parent_record_offset":3384,"ancestor_record_offset":772,"inlinee":8402,"source_line":2675,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":474,"record_offset":3420,"parent_record_offset":3384,"ancestor_record_offset":772,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":474,"record_offset":3384,"parent_record_offset":3356,"ancestor_record_offset":772,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":474,"record_offset":5344,"parent_record_offset":5320,"ancestor_record_offset":772,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$49::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":474,"record_offset":5320,"parent_record_offset":5280,"ancestor_record_offset":772,"inlinee":11040,"source_line":4154,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":474,"record_offset":2408,"parent_record_offset":2372,"ancestor_record_offset":772,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":474,"record_offset":7292,"parent_record_offset":7260,"ancestor_record_offset":772,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":474,"record_offset":7732,"parent_record_offset":7528,"ancestor_record_offset":772,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":474,"record_offset":2548,"parent_record_offset":2520,"ancestor_record_offset":772,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":474,"record_offset":2620,"parent_record_offset":2480,"ancestor_record_offset":772,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":474,"record_offset":2824,"parent_record_offset":2620,"ancestor_record_offset":772,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::drain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":474,"record_offset":3268,"parent_record_offset":3240,"ancestor_record_offset":772,"inlinee":11321,"source_line":738,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$67::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":474,"record_offset":5240,"parent_record_offset":5208,"ancestor_record_offset":772,"inlinee":11036,"source_line":2397,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":474,"record_offset":2576,"parent_record_offset":2548,"ancestor_record_offset":772,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":474,"record_offset":2480,"parent_record_offset":2444,"ancestor_record_offset":772,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":474,"record_offset":2444,"parent_record_offset":2408,"ancestor_record_offset":772,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual_state::impl$3::drain::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual_state.rs
//     evidence={"module":474,"record_offset":6332,"parent_record_offset":6308,"ancestor_record_offset":772,"inlinee":27649,"source_line":49,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::maintain_visual_entities::clear_out_wall_holes
// 记录形式: compiled_procedure; instances=1
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":474,"record_offset":272,"rva":9231664,"va":5377940784}
// 编译实例 RVAs: 0x8cdd30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] country_core::resources::walls::wall_holes::WallHoles::clear_for_next_frame
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t3671505","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5377940784,"raw_hex":"e9d1053800","size":5,"soft_fail":false,"target_va":5381612320,"va":5377940810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":474,"record_offset":440,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":474,"record_offset":464,"parent_record_offset":440,"ancestor_record_offset":272,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::maintain_visual_entities::clear_out_window_cuts
// 记录形式: compiled_procedure; instances=1
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":474,"record_offset":548,"rva":9231696,"va":5377940816}
// 编译实例 RVAs: 0x8cdd50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] country_core::resources::walls::window_cuts::WindowCutRecords::clear_for_next_frame
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t2545119","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5377940816,"raw_hex":"e9dfd52600","size":5,"soft_fail":false,"target_va":5380485952,"va":5377940828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":474,"record_offset":716,"parent_record_offset":548,"ancestor_record_offset":548,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":474,"record_offset":740,"parent_record_offset":716,"ancestor_record_offset":548,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00474.jsonl.gz"}
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
// 完整结构及原始类型记录: maintain_visual_entities.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: maintain_visual_entities.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: maintain_visual_entities.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::PrevWallHoles>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::PrevWallHoles>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::PrevWallHoles>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::window_cuts::WindowCutRecords>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::window_cuts::WindowCutRecords>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::window_cuts::WindowCutRecords>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::maintain_visual_entities::clear_out_visual_entities
// 信息 ID: 87e1b040fe630cd86f84c3cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] c02953a16fade6322fbb9ac7: void (*)(bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::system::commands::Commands)> > @ 0x287afe -> 0x8cdd70 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::system::commands::Commands)> @ 0x67d481 -> 0x8cdd70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::maintain_visual_entities::clear_out_wall_holes
// 信息 ID: 65d83a3f7a10030e8d0773d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] ce40887b725066b6674dbc04: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::PrevWallHoles>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::PrevWallHoles>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::PrevWallHoles>)> > @ 0x285cf7 -> 0x8cdd30 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::PrevWallHoles>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::PrevWallHoles>)> @ 0x67d65a -> 0x8cdd30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::maintain_visual_entities::clear_out_window_cuts
// 信息 ID: e8068469b77f97347e670932
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 8cdf8b86ba36cd175fb5a46d: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::window_cuts::WindowCutRecords>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::window_cuts::WindowCutRecords>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::window_cuts::WindowCutRecords>)> > @ 0x2cb0e8 -> 0x8cdd50 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::window_cuts::WindowCutRecords>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::window_cuts::WindowCutRecords>)> @ 0x67d7c0 -> 0x8cdd50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
