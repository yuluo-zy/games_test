// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/wall-constructor/src/construct_fences/process_segment.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 3a332285f331c056141664545be4b0ccdf6b20868940d4048fabdbb485d61cd2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: process_segment.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1471, 1504, 1505, 1525, 3161
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.20.5"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use perchance; // namespace candidate
//   package identities: [{"name":"perchance","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_fences::process_segment::emit_fences<system_wall_constructor::construct_fences::construct_fences::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1505,"record_offset":280,"rva":21446416,"va":5390155536}
// 编译实例 RVAs: 0x1473f10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14477816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390155536,"raw_hex":"e8081623ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5390156515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141419eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-369320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390155536,"raw_hex":"e8585dfaff","size":5,"soft_fail":false,"target_va":5389786800,"va":5390156115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26465167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390155536,"raw_hex":"e88fd39301","size":5,"soft_fail":false,"target_va":5416621408,"va":5390156236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390155536,"raw_hex":"e87d1b0000","size":5,"soft_fail":false,"target_va":5390162656,"va":5390155614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26465052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390155536,"raw_hex":"e81cd39301","size":5,"soft_fail":false,"target_va":5416621376,"va":5390156319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6192173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390155536,"raw_hex":"e8d383a1ff","size":5,"soft_fail":false,"target_va":5383963792,"va":5390155960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6183669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390155536,"raw_hex":"e80ba5a1ff","size":5,"soft_fail":false,"target_va":5383972224,"va":5390155888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6191286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390155536,"raw_hex":"e84a87a1ff","size":5,"soft_fail":false,"target_va":5383964912,"va":5390156193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6192776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390155536,"raw_hex":"e87881a1ff","size":5,"soft_fail":false,"target_va":5383962816,"va":5390155587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26540757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390155536,"raw_hex":"e8d5fa9401","size":5,"soft_fail":false,"target_va":5416697008,"va":5390156246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-385422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390155536,"raw_hex":"e8721efaff","size":5,"soft_fail":false,"target_va":5389771136,"va":5390156553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26463071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390155536,"raw_hex":"e85fcb9301","size":5,"soft_fail":false,"target_va":5416619360,"va":5390156284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1505,"record_offset":2764,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1505,"record_offset":2792,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1505,"record_offset":1928,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1505,"record_offset":532,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1505,"record_offset":916,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_fences::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_fences/mod.rs
//     evidence={"module":1505,"record_offset":2736,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":79916,"source_line":15,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1505,"record_offset":3848,"parent_record_offset":3824,"ancestor_record_offset":280,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1505,"record_offset":3872,"parent_record_offset":3848,"ancestor_record_offset":280,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1505,"record_offset":3896,"parent_record_offset":3872,"ancestor_record_offset":280,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1505,"record_offset":3824,"parent_record_offset":3732,"ancestor_record_offset":280,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1505,"record_offset":3760,"parent_record_offset":3732,"ancestor_record_offset":280,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1505,"record_offset":3732,"parent_record_offset":3708,"ancestor_record_offset":280,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1505,"record_offset":3708,"parent_record_offset":3684,"ancestor_record_offset":280,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1505,"record_offset":2256,"parent_record_offset":2228,"ancestor_record_offset":280,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1505,"record_offset":2284,"parent_record_offset":2256,"ancestor_record_offset":280,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1505,"record_offset":2204,"parent_record_offset":2176,"ancestor_record_offset":280,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1505,"record_offset":2228,"parent_record_offset":2204,"ancestor_record_offset":280,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1505,"record_offset":2344,"parent_record_offset":2228,"ancestor_record_offset":280,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1505,"record_offset":2456,"parent_record_offset":2432,"ancestor_record_offset":280,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1505,"record_offset":2308,"parent_record_offset":2284,"ancestor_record_offset":280,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1505,"record_offset":3792,"parent_record_offset":3760,"ancestor_record_offset":280,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1505,"record_offset":2076,"parent_record_offset":2036,"ancestor_record_offset":280,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1505,"record_offset":864,"parent_record_offset":836,"ancestor_record_offset":280,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1505,"record_offset":2692,"parent_record_offset":2036,"ancestor_record_offset":280,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1505,"record_offset":2408,"parent_record_offset":2176,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1505,"record_offset":1336,"parent_record_offset":1312,"ancestor_record_offset":280,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1505,"record_offset":1312,"parent_record_offset":1288,"ancestor_record_offset":280,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1505,"record_offset":1520,"parent_record_offset":1492,"ancestor_record_offset":280,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1505,"record_offset":1620,"parent_record_offset":1592,"ancestor_record_offset":280,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1505,"record_offset":1592,"parent_record_offset":1564,"ancestor_record_offset":280,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1505,"record_offset":1264,"parent_record_offset":1240,"ancestor_record_offset":280,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1505,"record_offset":1288,"parent_record_offset":1264,"ancestor_record_offset":280,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1505,"record_offset":1648,"parent_record_offset":1620,"ancestor_record_offset":280,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1505,"record_offset":1240,"parent_record_offset":1124,"ancestor_record_offset":280,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1505,"record_offset":660,"parent_record_offset":636,"ancestor_record_offset":280,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1505,"record_offset":952,"parent_record_offset":916,"ancestor_record_offset":280,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1505,"record_offset":1092,"parent_record_offset":1060,"ancestor_record_offset":280,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1505,"record_offset":1060,"parent_record_offset":1024,"ancestor_record_offset":280,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1505,"record_offset":1000,"parent_record_offset":976,"ancestor_record_offset":280,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1505,"record_offset":1024,"parent_record_offset":1000,"ancestor_record_offset":280,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1505,"record_offset":976,"parent_record_offset":952,"ancestor_record_offset":280,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1505,"record_offset":1564,"parent_record_offset":1024,"ancestor_record_offset":280,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1505,"record_offset":636,"parent_record_offset":604,"ancestor_record_offset":280,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1505,"record_offset":604,"parent_record_offset":576,"ancestor_record_offset":280,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1505,"record_offset":760,"parent_record_offset":728,"ancestor_record_offset":280,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1505,"record_offset":812,"parent_record_offset":760,"ancestor_record_offset":280,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1505,"record_offset":728,"parent_record_offset":700,"ancestor_record_offset":280,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1505,"record_offset":576,"parent_record_offset":532,"ancestor_record_offset":280,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1505,"record_offset":836,"parent_record_offset":812,"ancestor_record_offset":280,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1505,"record_offset":1124,"parent_record_offset":1092,"ancestor_record_offset":280,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1505,"record_offset":1408,"parent_record_offset":1124,"ancestor_record_offset":280,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1505,"record_offset":1380,"parent_record_offset":1124,"ancestor_record_offset":280,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1505,"record_offset":1184,"parent_record_offset":1124,"ancestor_record_offset":280,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1505,"record_offset":1964,"parent_record_offset":1928,"ancestor_record_offset":280,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1505,"record_offset":2856,"parent_record_offset":2824,"ancestor_record_offset":280,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1505,"record_offset":3296,"parent_record_offset":3092,"ancestor_record_offset":280,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1505,"record_offset":2104,"parent_record_offset":2076,"ancestor_record_offset":280,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1505,"record_offset":2176,"parent_record_offset":2036,"ancestor_record_offset":280,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1505,"record_offset":2380,"parent_record_offset":2176,"ancestor_record_offset":280,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1505,"record_offset":2132,"parent_record_offset":2104,"ancestor_record_offset":280,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1505,"record_offset":2036,"parent_record_offset":2000,"ancestor_record_offset":280,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1505,"record_offset":2000,"parent_record_offset":1964,"ancestor_record_offset":280,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_fences::process_segment::emit_fences::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x136A2
// 首条位置: {"module":1471,"inlinee":79522}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1471,"record_offset":1328,"parent_record_offset":732,"ancestor_record_offset":732,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1471,"record_offset":916,"parent_record_offset":732,"ancestor_record_offset":732,"inlinee":4101,"source_line":1805,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1471,"record_offset":788,"parent_record_offset":732,"ancestor_record_offset":732,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1471,"record_offset":1420,"parent_record_offset":1388,"ancestor_record_offset":732,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1471,"record_offset":1468,"parent_record_offset":1444,"ancestor_record_offset":732,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1471,"record_offset":1444,"parent_record_offset":1420,"ancestor_record_offset":732,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1471,"record_offset":1388,"parent_record_offset":1352,"ancestor_record_offset":732,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1471,"record_offset":1352,"parent_record_offset":1328,"ancestor_record_offset":732,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":1471,"record_offset":1084,"parent_record_offset":1044,"ancestor_record_offset":732,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1471,"record_offset":1044,"parent_record_offset":992,"ancestor_record_offset":732,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1471,"record_offset":852,"parent_record_offset":816,"ancestor_record_offset":732,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1471,"record_offset":1232,"parent_record_offset":1084,"ancestor_record_offset":732,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1471,"record_offset":1200,"parent_record_offset":1124,"ancestor_record_offset":732,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1471,"record_offset":1168,"parent_record_offset":1124,"ancestor_record_offset":732,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1471,"record_offset":1508,"parent_record_offset":1352,"ancestor_record_offset":732,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1471,"record_offset":880,"parent_record_offset":816,"ancestor_record_offset":732,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1471,"record_offset":816,"parent_record_offset":788,"ancestor_record_offset":732,"inlinee":6157,"source_line":435,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1471,"record_offset":1124,"parent_record_offset":1084,"ancestor_record_offset":732,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1471,"record_offset":1276,"parent_record_offset":992,"ancestor_record_offset":732,"inlinee":8899,"source_line":204,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_fences::process_segment::emit_fences::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_fences/process_segment.rs
//     evidence={"module":1471,"record_offset":944,"parent_record_offset":916,"ancestor_record_offset":732,"inlinee":79524,"source_line":40,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::bounding_box_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1471,"record_offset":968,"parent_record_offset":944,"ancestor_record_offset":732,"inlinee":36143,"source_line":1082,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::aabb::Aabb2::from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":1471,"record_offset":992,"parent_record_offset":968,"ancestor_record_offset":732,"inlinee":35826,"source_line":92,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_fences::process_segment::emit_fences::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x136A4
// 首条位置: {"module":1471,"inlinee":79524}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::Curve::bounding_box_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1471,"record_offset":968,"parent_record_offset":944,"ancestor_record_offset":944,"inlinee":36143,"source_line":1082,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":1471,"record_offset":1084,"parent_record_offset":1044,"ancestor_record_offset":944,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1471,"record_offset":1044,"parent_record_offset":992,"ancestor_record_offset":944,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1471,"record_offset":1232,"parent_record_offset":1084,"ancestor_record_offset":944,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1471,"record_offset":1200,"parent_record_offset":1124,"ancestor_record_offset":944,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1471,"record_offset":1168,"parent_record_offset":1124,"ancestor_record_offset":944,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1471,"record_offset":1124,"parent_record_offset":1084,"ancestor_record_offset":944,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1471,"record_offset":1276,"parent_record_offset":992,"ancestor_record_offset":944,"inlinee":8899,"source_line":204,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::aabb::Aabb2::from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":1471,"record_offset":992,"parent_record_offset":968,"ancestor_record_offset":944,"inlinee":35826,"source_line":92,"archive":"_pdb/C/modules/01471.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_fences::process_segment::process_segment<system_wall_constructor::construct_fences::construct_fences::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1505,"record_offset":4068,"rva":21447520,"va":5390156640}
// 编译实例 RVAs: 0x1474360
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-14478238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390156640,"raw_hex":"e8621423ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5390156921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t-14478931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390156640,"raw_hex":"e8ad1123ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5390157630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21434716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390156640,"raw_hex":"e8a4eeb8fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5390156903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081c20
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20921940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390156640,"raw_hex":"e8acc1c0fe","size":5,"soft_fail":false,"target_va":5369240608,"va":5390162543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404a3e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16582531","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390156640,"raw_hex":"e87df802ff","size":5,"soft_fail":false,"target_va":5373574768,"va":5390157294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=2
//     target_aliases=20 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12032376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390156640,"raw_hex":"e8886648ff","size":5,"soft_fail":false,"target_va":5378126944,"va":5390159315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c6a170
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8432760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390156640,"raw_hex":"e888537fff","size":5,"soft_fail":false,"target_va":5381726576,"va":5390159331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6833327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390156640,"raw_hex":"e851bb97ff","size":5,"soft_fail":false,"target_va":5383325024,"va":5390158346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141403450
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-465443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390156640,"raw_hex":"e8dde5f8ff","size":5,"soft_fail":false,"target_va":5389694032,"va":5390159470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141406130
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-452005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390156640,"raw_hex":"e85b1af9ff","size":5,"soft_fail":false,"target_va":5389705520,"va":5390157520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26899800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390156640,"raw_hex":"e858759a01","size":5,"soft_fail":false,"target_va":5417057056,"va":5390157251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26900168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390156640,"raw_hex":"e8c8769a01","size":5,"soft_fail":false,"target_va":5417057664,"va":5390157491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26897837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390156640,"raw_hex":"e8ad6d9a01","size":5,"soft_fail":false,"target_va":5417057792,"va":5390159950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26222386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390156640,"raw_hex":"e8321f9001","size":5,"soft_fail":false,"target_va":5416380784,"va":5390158393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26463910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390156640,"raw_hex":"e8a6ce9301","size":5,"soft_fail":false,"target_va":5416620794,"va":5390156879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26811422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390156640,"raw_hex":"e81e1c9901","size":5,"soft_fail":false,"target_va":5416973319,"va":5390161892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26459842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390156640,"raw_hex":"e8c2be9301","size":5,"soft_fail":false,"target_va":5416621762,"va":5390161915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26459850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390156640,"raw_hex":"e8cabe9301","size":5,"soft_fail":false,"target_va":5416621632,"va":5390161777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26460601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390156640,"raw_hex":"e8b9c19301","size":5,"soft_fail":false,"target_va":5416622320,"va":5390161714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] perchance::PerchanceContext::uniform_disc_vec2
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7177834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390156640,"raw_hex":"e86a866d00","size":5,"soft_fail":false,"target_va":5397335984,"va":5390158145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] perchance::PerchanceContext::usize_less_than
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7174936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390156640,"raw_hex":"e8187b6d00","size":5,"soft_fail":false,"target_va":5397335840,"va":5390160899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t17255956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390156640,"raw_hex":"e8144e0701","size":5,"soft_fail":false,"target_va":5407412800,"va":5390156839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::max_extent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6855575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390156640,"raw_hex":"e8696497ff","size":5,"soft_fail":false,"target_va":5383301808,"va":5390157378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1505,"record_offset":5464,"parent_record_offset":4068,"ancestor_record_offset":4068,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1505,"record_offset":4480,"parent_record_offset":4068,"ancestor_record_offset":4068,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$59::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1505,"record_offset":13580,"parent_record_offset":4068,"ancestor_record_offset":4068,"inlinee":37485,"source_line":2016,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1505,"record_offset":4984,"parent_record_offset":4068,"ancestor_record_offset":4068,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1505,"record_offset":13608,"parent_record_offset":4068,"ancestor_record_offset":4068,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1505,"record_offset":6500,"parent_record_offset":4068,"ancestor_record_offset":4068,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1505,"record_offset":4900,"parent_record_offset":4068,"ancestor_record_offset":4068,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1505,"record_offset":5364,"parent_record_offset":4068,"ancestor_record_offset":4068,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1505,"record_offset":14640,"parent_record_offset":4068,"ancestor_record_offset":4068,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1505,"record_offset":15536,"parent_record_offset":4068,"ancestor_record_offset":4068,"inlinee":9830,"source_line":262,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1505,"record_offset":13088,"parent_record_offset":4068,"ancestor_record_offset":4068,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1505,"record_offset":15260,"parent_record_offset":4068,"ancestor_record_offset":4068,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$105::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1505,"record_offset":15508,"parent_record_offset":4068,"ancestor_record_offset":4068,"inlinee":11117,"source_line":20,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1505,"record_offset":15056,"parent_record_offset":4068,"ancestor_record_offset":4068,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1505,"record_offset":11684,"parent_record_offset":4068,"ancestor_record_offset":4068,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1505,"record_offset":15144,"parent_record_offset":4068,"ancestor_record_offset":4068,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1505,"record_offset":11772,"parent_record_offset":4068,"ancestor_record_offset":4068,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$88::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1505,"record_offset":14552,"parent_record_offset":4068,"ancestor_record_offset":4068,"inlinee":11877,"source_line":2063,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1505,"record_offset":12068,"parent_record_offset":4068,"ancestor_record_offset":4068,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1505,"record_offset":6172,"parent_record_offset":4068,"ancestor_record_offset":4068,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] perchance::PerchanceContext::choose
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":1505,"record_offset":15024,"parent_record_offset":4068,"ancestor_record_offset":4068,"inlinee":79925,"source_line":203,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] perchance::PerchanceContext::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":1505,"record_offset":4424,"parent_record_offset":4068,"ancestor_record_offset":4068,"inlinee":79917,"source_line":144,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] perchance::PerchanceContext::uniform_f32
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":1505,"record_offset":12420,"parent_record_offset":4068,"ancestor_record_offset":4068,"inlinee":31959,"source_line":250,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1505,"record_offset":6340,"parent_record_offset":4068,"ancestor_record_offset":4068,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1505,"record_offset":13524,"parent_record_offset":4068,"ancestor_record_offset":4068,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_fences::process_segment::place_fence_piece
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_fences/process_segment.rs
//     evidence={"module":1505,"record_offset":11828,"parent_record_offset":4068,"ancestor_record_offset":4068,"inlinee":79922,"source_line":307,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_fences::process_segment::process_segment::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_fences/process_segment.rs
//     evidence={"module":1505,"record_offset":7196,"parent_record_offset":4068,"ancestor_record_offset":4068,"inlinee":79920,"source_line":130,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1505,"record_offset":6040,"parent_record_offset":4068,"ancestor_record_offset":4068,"inlinee":26085,"source_line":264,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_resample
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1505,"record_offset":6396,"parent_record_offset":4068,"ancestor_record_offset":4068,"inlinee":79445,"source_line":324,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1505,"record_offset":6312,"parent_record_offset":4068,"ancestor_record_offset":4068,"inlinee":9818,"source_line":169,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1505,"record_offset":6108,"parent_record_offset":4068,"ancestor_record_offset":4068,"inlinee":11673,"source_line":233,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1505,"record_offset":4792,"parent_record_offset":4768,"ancestor_record_offset":4068,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1505,"record_offset":4816,"parent_record_offset":4792,"ancestor_record_offset":4068,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1505,"record_offset":6712,"parent_record_offset":6688,"ancestor_record_offset":4068,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1505,"record_offset":6736,"parent_record_offset":6712,"ancestor_record_offset":4068,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1505,"record_offset":4840,"parent_record_offset":4816,"ancestor_record_offset":4068,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1505,"record_offset":6760,"parent_record_offset":6736,"ancestor_record_offset":4068,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1505,"record_offset":4768,"parent_record_offset":4584,"ancestor_record_offset":4068,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1505,"record_offset":6688,"parent_record_offset":6596,"ancestor_record_offset":4068,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1505,"record_offset":7860,"parent_record_offset":7828,"ancestor_record_offset":4068,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1505,"record_offset":4528,"parent_record_offset":4504,"ancestor_record_offset":4068,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1505,"record_offset":6624,"parent_record_offset":6596,"ancestor_record_offset":4068,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1505,"record_offset":6596,"parent_record_offset":6572,"ancestor_record_offset":4068,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1505,"record_offset":7908,"parent_record_offset":7884,"ancestor_record_offset":4068,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1505,"record_offset":7884,"parent_record_offset":7860,"ancestor_record_offset":4068,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1505,"record_offset":4584,"parent_record_offset":4552,"ancestor_record_offset":4068,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1505,"record_offset":4552,"parent_record_offset":4528,"ancestor_record_offset":4068,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1505,"record_offset":6572,"parent_record_offset":6548,"ancestor_record_offset":4068,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1505,"record_offset":4624,"parent_record_offset":4584,"ancestor_record_offset":4068,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1505,"record_offset":5500,"parent_record_offset":5464,"ancestor_record_offset":4068,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1505,"record_offset":5536,"parent_record_offset":5500,"ancestor_record_offset":4068,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1505,"record_offset":7828,"parent_record_offset":7772,"ancestor_record_offset":4068,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1505,"record_offset":7728,"parent_record_offset":7196,"ancestor_record_offset":4068,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1505,"record_offset":7772,"parent_record_offset":7728,"ancestor_record_offset":4068,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1505,"record_offset":4504,"parent_record_offset":4480,"ancestor_record_offset":4068,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1505,"record_offset":4648,"parent_record_offset":4624,"ancestor_record_offset":4068,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1505,"record_offset":13656,"parent_record_offset":13632,"ancestor_record_offset":4068,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1505,"record_offset":11920,"parent_record_offset":11828,"ancestor_record_offset":4068,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1505,"record_offset":6136,"parent_record_offset":6108,"ancestor_record_offset":4068,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1505,"record_offset":6364,"parent_record_offset":6340,"ancestor_record_offset":4068,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1505,"record_offset":13548,"parent_record_offset":13524,"ancestor_record_offset":4068,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::powi
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1505,"record_offset":8816,"parent_record_offset":8792,"ancestor_record_offset":4068,"inlinee":17053,"source_line":2010,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1505,"record_offset":7604,"parent_record_offset":7580,"ancestor_record_offset":4068,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_right
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1505,"record_offset":8400,"parent_record_offset":8376,"ancestor_record_offset":4068,"inlinee":12398,"source_line":2047,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1505,"record_offset":4728,"parent_record_offset":4672,"ancestor_record_offset":4068,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1505,"record_offset":5152,"parent_record_offset":5128,"ancestor_record_offset":4068,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1505,"record_offset":5200,"parent_record_offset":5176,"ancestor_record_offset":4068,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1505,"record_offset":13632,"parent_record_offset":13608,"ancestor_record_offset":4068,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1505,"record_offset":5224,"parent_record_offset":5200,"ancestor_record_offset":4068,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$10::wrapping_mul
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1505,"record_offset":8348,"parent_record_offset":8312,"ancestor_record_offset":4068,"inlinee":6064,"source_line":2532,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1505,"record_offset":4672,"parent_record_offset":4648,"ancestor_record_offset":4068,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1505,"record_offset":5428,"parent_record_offset":5392,"ancestor_record_offset":4068,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1505,"record_offset":4700,"parent_record_offset":4672,"ancestor_record_offset":4068,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1505,"record_offset":6656,"parent_record_offset":6624,"ancestor_record_offset":4068,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_right
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1505,"record_offset":8376,"parent_record_offset":8312,"ancestor_record_offset":4068,"inlinee":12397,"source_line":414,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1505,"record_offset":6280,"parent_record_offset":6172,"ancestor_record_offset":4068,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=38
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1505,"record_offset":6224,"parent_record_offset":6172,"ancestor_record_offset":4068,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$327::neg
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1505,"record_offset":14580,"parent_record_offset":14552,"ancestor_record_offset":4068,"inlinee":8939,"source_line":720,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1505,"record_offset":15172,"parent_record_offset":15144,"ancestor_record_offset":4068,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1505,"record_offset":6196,"parent_record_offset":6172,"ancestor_record_offset":4068,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1505,"record_offset":5176,"parent_record_offset":5152,"ancestor_record_offset":4068,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1505,"record_offset":5972,"parent_record_offset":5536,"ancestor_record_offset":4068,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1505,"record_offset":5996,"parent_record_offset":5972,"ancestor_record_offset":4068,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1505,"record_offset":6524,"parent_record_offset":6500,"ancestor_record_offset":4068,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1505,"record_offset":4948,"parent_record_offset":4924,"ancestor_record_offset":4068,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1505,"record_offset":5096,"parent_record_offset":5040,"ancestor_record_offset":4068,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1505,"record_offset":5068,"parent_record_offset":5040,"ancestor_record_offset":4068,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1505,"record_offset":7948,"parent_record_offset":7772,"ancestor_record_offset":4068,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1505,"record_offset":6068,"parent_record_offset":6040,"ancestor_record_offset":4068,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1505,"record_offset":5392,"parent_record_offset":5364,"ancestor_record_offset":4068,"inlinee":11671,"source_line":696,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1505,"record_offset":4924,"parent_record_offset":4900,"ancestor_record_offset":4068,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1505,"record_offset":5040,"parent_record_offset":5008,"ancestor_record_offset":4068,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] glam::core::scalar::vector::impl$9::mul_scalar
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.20.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.20.5/src/core/scalar/vector.rs
//     evidence={"module":1505,"record_offset":7312,"parent_record_offset":7288,"ancestor_record_offset":4068,"inlinee":33024,"source_line":238,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1505,"record_offset":8632,"parent_record_offset":8600,"ancestor_record_offset":4068,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1505,"record_offset":8600,"parent_record_offset":8568,"ancestor_record_offset":4068,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1505,"record_offset":8568,"parent_record_offset":8544,"ancestor_record_offset":4068,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1505,"record_offset":8544,"parent_record_offset":7196,"ancestor_record_offset":4068,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1505,"record_offset":8456,"parent_record_offset":7196,"ancestor_record_offset":4068,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1505,"record_offset":7680,"parent_record_offset":7196,"ancestor_record_offset":4068,"inlinee":9830,"source_line":262,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1505,"record_offset":7548,"parent_record_offset":7516,"ancestor_record_offset":4068,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1505,"record_offset":7516,"parent_record_offset":7484,"ancestor_record_offset":4068,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1505,"record_offset":7484,"parent_record_offset":7456,"ancestor_record_offset":4068,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1505,"record_offset":7456,"parent_record_offset":7196,"ancestor_record_offset":4068,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1505,"record_offset":8000,"parent_record_offset":7196,"ancestor_record_offset":4068,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1505,"record_offset":8088,"parent_record_offset":7196,"ancestor_record_offset":4068,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] glam::vec2::impl$26::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.20.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.20.5/src/vec.rs
//     evidence={"module":1505,"record_offset":7288,"parent_record_offset":7196,"ancestor_record_offset":4068,"inlinee":33022,"source_line":576,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1505,"record_offset":5128,"parent_record_offset":5008,"ancestor_record_offset":4068,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1505,"record_offset":5008,"parent_record_offset":4984,"ancestor_record_offset":4068,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] perchance::PerchanceContext::get_u32
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":1505,"record_offset":8252,"parent_record_offset":8228,"ancestor_record_offset":4068,"inlinee":31960,"source_line":154,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] perchance::PerchanceContext::uniform_f32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":1505,"record_offset":8228,"parent_record_offset":8204,"ancestor_record_offset":4068,"inlinee":31959,"source_line":250,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] perchance::PerchanceContext::uniform_range_f32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":1505,"record_offset":8180,"parent_record_offset":7196,"ancestor_record_offset":4068,"inlinee":31957,"source_line":270,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] perchance::PerchanceContext::uniform_range_f32_raw
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":1505,"record_offset":8204,"parent_record_offset":8180,"ancestor_record_offset":4068,"inlinee":31958,"source_line":275,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] perchance::Rng::from_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":1505,"record_offset":4448,"parent_record_offset":4424,"ancestor_record_offset":4068,"inlinee":79918,"source_line":66,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] perchance::Rng::next_u32
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":1505,"record_offset":8288,"parent_record_offset":8252,"ancestor_record_offset":4068,"inlinee":31962,"source_line":72,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] perchance::Rng::next_u64
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":1505,"record_offset":8312,"parent_record_offset":8288,"ancestor_record_offset":4068,"inlinee":31963,"source_line":77,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powi
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1505,"record_offset":8792,"parent_record_offset":7196,"ancestor_record_offset":4068,"inlinee":17052,"source_line":323,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1505,"record_offset":7580,"parent_record_offset":7548,"ancestor_record_offset":4068,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_fences::construct_fences::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_fences/mod.rs
//     evidence={"module":1505,"record_offset":11948,"parent_record_offset":11828,"ancestor_record_offset":4068,"inlinee":79924,"source_line":47,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_resample_remap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1505,"record_offset":6424,"parent_record_offset":6396,"ancestor_record_offset":4068,"inlinee":79438,"source_line":346,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_fences::process_segment::process_segment::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 130
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13830
// 首条位置: {"module":1505,"inlinee":79920}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1505,"record_offset":7728,"parent_record_offset":7196,"ancestor_record_offset":7196,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1505,"record_offset":8544,"parent_record_offset":7196,"ancestor_record_offset":7196,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1505,"record_offset":8456,"parent_record_offset":7196,"ancestor_record_offset":7196,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1505,"record_offset":7680,"parent_record_offset":7196,"ancestor_record_offset":7196,"inlinee":9830,"source_line":262,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1505,"record_offset":7456,"parent_record_offset":7196,"ancestor_record_offset":7196,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1505,"record_offset":8000,"parent_record_offset":7196,"ancestor_record_offset":7196,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1505,"record_offset":8088,"parent_record_offset":7196,"ancestor_record_offset":7196,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] glam::vec2::impl$26::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.20.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.20.5/src/vec.rs
//     evidence={"module":1505,"record_offset":7288,"parent_record_offset":7196,"ancestor_record_offset":7196,"inlinee":33022,"source_line":576,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] perchance::PerchanceContext::uniform_range_f32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":1505,"record_offset":8180,"parent_record_offset":7196,"ancestor_record_offset":7196,"inlinee":31957,"source_line":270,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1505,"record_offset":8792,"parent_record_offset":7196,"ancestor_record_offset":7196,"inlinee":17052,"source_line":323,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1505,"record_offset":7860,"parent_record_offset":7828,"ancestor_record_offset":7196,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1505,"record_offset":7908,"parent_record_offset":7884,"ancestor_record_offset":7196,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1505,"record_offset":7884,"parent_record_offset":7860,"ancestor_record_offset":7196,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1505,"record_offset":7828,"parent_record_offset":7772,"ancestor_record_offset":7196,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1505,"record_offset":7772,"parent_record_offset":7728,"ancestor_record_offset":7196,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::powi
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1505,"record_offset":8816,"parent_record_offset":8792,"ancestor_record_offset":7196,"inlinee":17053,"source_line":2010,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1505,"record_offset":7604,"parent_record_offset":7580,"ancestor_record_offset":7196,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_right
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1505,"record_offset":8400,"parent_record_offset":8376,"ancestor_record_offset":7196,"inlinee":12398,"source_line":2047,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$10::wrapping_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1505,"record_offset":8348,"parent_record_offset":8312,"ancestor_record_offset":7196,"inlinee":6064,"source_line":2532,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_right
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1505,"record_offset":8376,"parent_record_offset":8312,"ancestor_record_offset":7196,"inlinee":12397,"source_line":414,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1505,"record_offset":8120,"parent_record_offset":8088,"ancestor_record_offset":7196,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1505,"record_offset":7336,"parent_record_offset":7312,"ancestor_record_offset":7196,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1505,"record_offset":8484,"parent_record_offset":8456,"ancestor_record_offset":7196,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1505,"record_offset":7948,"parent_record_offset":7772,"ancestor_record_offset":7196,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] glam::core::scalar::vector::impl$9::mul_scalar
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.20.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.20.5/src/core/scalar/vector.rs
//     evidence={"module":1505,"record_offset":7312,"parent_record_offset":7288,"ancestor_record_offset":7196,"inlinee":33024,"source_line":238,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1505,"record_offset":8632,"parent_record_offset":8600,"ancestor_record_offset":7196,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1505,"record_offset":8600,"parent_record_offset":8568,"ancestor_record_offset":7196,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1505,"record_offset":8568,"parent_record_offset":8544,"ancestor_record_offset":7196,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1505,"record_offset":7548,"parent_record_offset":7516,"ancestor_record_offset":7196,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1505,"record_offset":7516,"parent_record_offset":7484,"ancestor_record_offset":7196,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1505,"record_offset":7484,"parent_record_offset":7456,"ancestor_record_offset":7196,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] perchance::PerchanceContext::get_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":1505,"record_offset":8252,"parent_record_offset":8228,"ancestor_record_offset":7196,"inlinee":31960,"source_line":154,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] perchance::PerchanceContext::uniform_f32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":1505,"record_offset":8228,"parent_record_offset":8204,"ancestor_record_offset":7196,"inlinee":31959,"source_line":250,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] perchance::PerchanceContext::uniform_range_f32_raw
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":1505,"record_offset":8204,"parent_record_offset":8180,"ancestor_record_offset":7196,"inlinee":31958,"source_line":275,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] perchance::Rng::next_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":1505,"record_offset":8288,"parent_record_offset":8252,"ancestor_record_offset":7196,"inlinee":31962,"source_line":72,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] perchance::Rng::next_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":1505,"record_offset":8312,"parent_record_offset":8288,"ancestor_record_offset":7196,"inlinee":31963,"source_line":77,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1505,"record_offset":7580,"parent_record_offset":7548,"ancestor_record_offset":7196,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle
// 记录形式: compiled_procedure; instances=1
// 源码行: 282
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3161,"record_offset":280,"rva":38703680,"va":5407412800}
// 编译实例 RVAs: 0x24e9240
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-31734195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407412800,"raw_hex":"e84dc61bfe","size":5,"soft_fail":false,"target_va":5375678688,"va":5407412878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-31734973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407412800,"raw_hex":"e843c31bfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5407413672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38690676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407412800,"raw_hex":"e88ca0b1fd","size":5,"soft_fail":false,"target_va":5368722192,"va":5407412863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414816b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17202363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407412800,"raw_hex":"e84583f9fe","size":5,"soft_fail":false,"target_va":5390210736,"va":5407413094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bda0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9644836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407412800,"raw_hex":"e8242b9300","size":5,"soft_fail":false,"target_va":5417057696,"va":5407412855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9207177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407412800,"raw_hex":"e8897d8c00","size":5,"soft_fail":false,"target_va":5416620769,"va":5407413587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t8952565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407412800,"raw_hex":"e8f59a8800","size":5,"soft_fail":false,"target_va":5416365968,"va":5407413398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9559861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407412800,"raw_hex":"e835df9100","size":5,"soft_fail":false,"target_va":5416973319,"va":5407413453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9208157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407412800,"raw_hex":"e85d818c00","size":5,"soft_fail":false,"target_va":5416621762,"va":5407413600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":3161,"record_offset":692,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":3161,"record_offset":512,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3161,"record_offset":776,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3161,"record_offset":992,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3161,"record_offset":2208,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$105::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3161,"record_offset":1600,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11117,"source_line":20,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3161,"record_offset":1676,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3161,"record_offset":484,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11241,"source_line":723,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_direct] utils::cos_angle_between
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3161,"record_offset":1764,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":117790,"source_line":488,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3161,"record_offset":596,"parent_record_offset":572,"ancestor_record_offset":280,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3161,"record_offset":620,"parent_record_offset":596,"ancestor_record_offset":280,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3161,"record_offset":2396,"parent_record_offset":2372,"ancestor_record_offset":280,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3161,"record_offset":2420,"parent_record_offset":2396,"ancestor_record_offset":280,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3161,"record_offset":644,"parent_record_offset":620,"ancestor_record_offset":280,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3161,"record_offset":2444,"parent_record_offset":2420,"ancestor_record_offset":280,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3161,"record_offset":572,"parent_record_offset":540,"ancestor_record_offset":280,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3161,"record_offset":2372,"parent_record_offset":2280,"ancestor_record_offset":280,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":3161,"record_offset":540,"parent_record_offset":512,"ancestor_record_offset":280,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3161,"record_offset":868,"parent_record_offset":844,"ancestor_record_offset":280,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3161,"record_offset":2308,"parent_record_offset":2280,"ancestor_record_offset":280,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3161,"record_offset":2280,"parent_record_offset":2256,"ancestor_record_offset":280,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3161,"record_offset":916,"parent_record_offset":892,"ancestor_record_offset":280,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3161,"record_offset":892,"parent_record_offset":868,"ancestor_record_offset":280,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3161,"record_offset":2256,"parent_record_offset":2232,"ancestor_record_offset":280,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3161,"record_offset":716,"parent_record_offset":692,"ancestor_record_offset":280,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3161,"record_offset":844,"parent_record_offset":804,"ancestor_record_offset":280,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3161,"record_offset":740,"parent_record_offset":716,"ancestor_record_offset":280,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3161,"record_offset":804,"parent_record_offset":776,"ancestor_record_offset":280,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::Clone::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\clone.rs
//     evidence={"module":3161,"record_offset":1560,"parent_record_offset":1532,"ancestor_record_offset":280,"inlinee":6914,"source_line":236,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3161,"record_offset":1164,"parent_record_offset":1136,"ancestor_record_offset":280,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3161,"record_offset":1904,"parent_record_offset":1880,"ancestor_record_offset":280,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":3161,"record_offset":1500,"parent_record_offset":1476,"ancestor_record_offset":280,"inlinee":8137,"source_line":36,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":3161,"record_offset":1084,"parent_record_offset":1032,"ancestor_record_offset":280,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":3161,"record_offset":1192,"parent_record_offset":1136,"ancestor_record_offset":280,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":3161,"record_offset":1136,"parent_record_offset":1112,"ancestor_record_offset":280,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":3161,"record_offset":1112,"parent_record_offset":1084,"ancestor_record_offset":280,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3161,"record_offset":1476,"parent_record_offset":1032,"ancestor_record_offset":280,"inlinee":8135,"source_line":507,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":3161,"record_offset":1380,"parent_record_offset":1356,"ancestor_record_offset":280,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3161,"record_offset":1216,"parent_record_offset":1192,"ancestor_record_offset":280,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3161,"record_offset":2340,"parent_record_offset":2308,"ancestor_record_offset":280,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3161,"record_offset":1704,"parent_record_offset":1676,"ancestor_record_offset":280,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":3161,"record_offset":1284,"parent_record_offset":1256,"ancestor_record_offset":280,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3161,"record_offset":1256,"parent_record_offset":1084,"ancestor_record_offset":280,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3161,"record_offset":1532,"parent_record_offset":1032,"ancestor_record_offset":280,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3161,"record_offset":2232,"parent_record_offset":2208,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3161,"record_offset":956,"parent_record_offset":804,"ancestor_record_offset":280,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3161,"record_offset":1792,"parent_record_offset":1764,"ancestor_record_offset":280,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3161,"record_offset":1824,"parent_record_offset":1764,"ancestor_record_offset":280,"inlinee":26151,"source_line":471,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$30::left_shift_push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":3161,"record_offset":1356,"parent_record_offset":1032,"ancestor_record_offset":280,"inlinee":27237,"source_line":388,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":3161,"record_offset":1032,"parent_record_offset":992,"ancestor_record_offset":280,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3161,"record_offset":1880,"parent_record_offset":1764,"ancestor_record_offset":280,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_fences/process_segment.rs
//     evidence={"module":3161,"record_offset":1312,"parent_record_offset":1284,"ancestor_record_offset":280,"inlinee":79900,"source_line":286,"archive":"_pdb/C/modules/03161.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 286
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1381C
// 首条位置: {"module":1504,"inlinee":79900}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_fences::process_segment::place_fence_piece
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 307
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13832
// 首条位置: {"module":1505,"inlinee":79922}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1505,"record_offset":11920,"parent_record_offset":11828,"ancestor_record_offset":11828,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1505,"record_offset":11864,"parent_record_offset":11828,"ancestor_record_offset":11828,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_fences::construct_fences::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_fences/mod.rs
//     evidence={"module":1505,"record_offset":11948,"parent_record_offset":11828,"ancestor_record_offset":11828,"inlinee":79924,"source_line":47,"archive":"_pdb/C/modules/01505.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1505,"record_offset":11888,"parent_record_offset":11864,"ancestor_record_offset":11828,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01505.jsonl.gz"}
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
// 完整结构及原始类型记录: process_segment.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::construct_fences::process_segment::process_segment::FencePost
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<system_wall_constructor::construct_fences::process_segment::process_segment::FencePost,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["system_wall_constructor::construct_fences::process_segment::process_segment::FencePost", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: process_segment.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: process_segment.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_fences::process_segment::emit_fences::closure$1
// 信息 ID: 24e6892898efc04a8085c0a3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_fences::process_segment::emit_fences", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$1 <- utils::threshold_range::threshold_range<system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$0<system_wall_constructor::construct_fences::construct_fences::closure_env$0>,system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$1<system_wall_constructor::construct_fences::construct_fences::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] utils::threshold_range::threshold_range<system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$0<system_wall_constructor::construct_fences::construct_fences::closure_env$0>,system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$1<system_wall_constructor::construct_fences::construct_fences::closure_env$0> >; module=1471, inline_record=732; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_fences::process_segment::emit_fences::closure$1::closure$0
// 信息 ID: 6ef6d98667155ebf1baf98f9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_fences::process_segment::emit_fences", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::threshold_range::threshold_range<system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$0<system_wall_constructor::construct_fences::construct_fences::closure_env$0>,system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$1<system_wall_constructor::construct_fences::construct_fences::closure_env$0> >; module=1471, inline_record=944; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_fences::process_segment::emit_fences<system_wall_constructor::construct_fences::construct_fences::closure_env$0>
// 信息 ID: ccb882a758e29577f9c912f0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::construct_fences::process_segment::emit_fences -> ["system_wall_constructor::construct_fences::construct_fences::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_fences::construct_fences @ 0x80dc7c -> 0x1473f10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_fences::process_segment::place_fence_piece
// 信息 ID: f39ba58d14cf08e353e599c9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_fences::process_segment::process_segment<system_wall_constructor::construct_fences::construct_fences::closure_env$0>; module=1505, inline_record=11828; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_fences::process_segment::process_segment<system_wall_constructor::construct_fences::construct_fences::closure_env$0>; module=1505, inline_record=15848; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_fences::process_segment::process_segment::closure$0
// 信息 ID: 06c6baf887e68398dc74ecb5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_fences::process_segment::process_segment", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_fences::process_segment::process_segment<system_wall_constructor::construct_fences::construct_fences::closure_env$0>; module=1505, inline_record=7196; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_fences::process_segment::process_segment<system_wall_constructor::construct_fences::construct_fences::closure_env$0>; module=1505, inline_record=9232; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_fences::process_segment::process_segment<system_wall_constructor::construct_fences::construct_fences::closure_env$0>
// 信息 ID: 9b48a027f13f0755a5140b0a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::construct_fences::process_segment::process_segment -> ["system_wall_constructor::construct_fences::construct_fences::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::threshold_range::threshold_range<system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$0<system_wall_constructor::construct_fences::construct_fences::closure_env$0>,system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$1<system_wall_constructor::construct_fences::construct_fences::closure_env$0> > @ 0x141a142 -> 0x1474360 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle
// 信息 ID: 5c6a25c9114734804cc37ac7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_fences::process_segment::process_segment<system_wall_constructor::construct_fences::construct_fences::closure_env$0> @ 0x1474427 -> 0x24e9240 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle::closure$0
// 信息 ID: 4d418d72b7347755d4cbfca9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle::closure_env$0 <- itertools::tuple_impl::impl$30::collect_from_iter_no_buf<ref$<glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<ref$<glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] itertools::tuple_impl::impl$30::collect_from_iter_no_buf<ref$<glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<ref$<glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle::closure_env$0> > > >; module=1525, inline_record=11424; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$30::collect_from_iter_no_buf<ref$<glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<ref$<glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle::closure_env$0> > > >; module=1525, inline_record=11732; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$30::collect_from_iter_no_buf<ref$<glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<ref$<glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle::closure_env$0> > > >; module=1525, inline_record=12176; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$30::collect_from_iter_no_buf<ref$<glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<ref$<glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle::closure_env$0> > > >; module=1525, inline_record=12588; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle; module=3161, inline_record=1312; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
