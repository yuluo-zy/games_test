// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/wall-constructor/src/construct_fences/process_segment.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 6e6d68775b229ab94e15ff902f9bfd5efed97e65d0c5cac8cd379e58607eec41
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: process_segment.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1413, 1452, 1454, 1455, 3027
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
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.20.5"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
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
// 首条位置: {"module":1454,"record_offset":2052,"rva":19106032,"va":5387815152}
// 编译实例 RVAs: 0x12388f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12734696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387815152,"raw_hex":"e818af3dff","size":5,"soft_fail":false,"target_va":5375081504,"va":5387816195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411fff30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-232612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387815152,"raw_hex":"e85c73fcff","size":5,"soft_fail":false,"target_va":5387583280,"va":5387815887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23725505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387815152,"raw_hex":"e8c1056a01","size":5,"soft_fail":false,"target_va":5411541584,"va":5387816074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387815152,"raw_hex":"e881fbffff","size":5,"soft_fail":false,"target_va":5387814080,"va":5387815226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t23728052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387815152,"raw_hex":"e8b40f6a01","size":5,"soft_fail":false,"target_va":5411543360,"va":5387815303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6430202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387815152,"raw_hex":"e806e29dff","size":5,"soft_fail":false,"target_va":5381385536,"va":5387815733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6425016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387815152,"raw_hex":"e848f69dff","size":5,"soft_fail":false,"target_va":5381390640,"va":5387815651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6429499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387815152,"raw_hex":"e8c5e49dff","size":5,"soft_fail":false,"target_va":5381386480,"va":5387815974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6430504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387815152,"raw_hex":"e8d8e09dff","size":5,"soft_fail":false,"target_va":5381384704,"va":5387815203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23824747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387815152,"raw_hex":"e86b896b01","size":5,"soft_fail":false,"target_va":5411640768,"va":5387816016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-251806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387815152,"raw_hex":"e86228fcff","size":5,"soft_fail":false,"target_va":5387564432,"va":5387816233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23721037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387815152,"raw_hex":"e84df46901","size":5,"soft_fail":false,"target_va":5411537104,"va":5387816062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1454,"record_offset":4756,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1454,"record_offset":4784,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1454,"record_offset":3760,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1454,"record_offset":2304,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1454,"record_offset":2772,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_fences::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_fences/mod.rs
//     evidence={"module":1454,"record_offset":4728,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":71065,"source_line":15,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1454,"record_offset":6012,"parent_record_offset":5988,"ancestor_record_offset":2052,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1454,"record_offset":5988,"parent_record_offset":5896,"ancestor_record_offset":2052,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1454,"record_offset":5924,"parent_record_offset":5896,"ancestor_record_offset":2052,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1454,"record_offset":5896,"parent_record_offset":5872,"ancestor_record_offset":2052,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1454,"record_offset":5872,"parent_record_offset":5848,"ancestor_record_offset":2052,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1454,"record_offset":4080,"parent_record_offset":4052,"ancestor_record_offset":2052,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1454,"record_offset":4132,"parent_record_offset":4108,"ancestor_record_offset":2052,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1454,"record_offset":4108,"parent_record_offset":4080,"ancestor_record_offset":2052,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1454,"record_offset":4028,"parent_record_offset":4000,"ancestor_record_offset":2052,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1454,"record_offset":4052,"parent_record_offset":4028,"ancestor_record_offset":2052,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1454,"record_offset":4224,"parent_record_offset":4052,"ancestor_record_offset":2052,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1454,"record_offset":4336,"parent_record_offset":4312,"ancestor_record_offset":2052,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1454,"record_offset":4156,"parent_record_offset":4132,"ancestor_record_offset":2052,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1454,"record_offset":5956,"parent_record_offset":5924,"ancestor_record_offset":2052,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":1454,"record_offset":2464,"parent_record_offset":2432,"ancestor_record_offset":2052,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1454,"record_offset":3896,"parent_record_offset":3856,"ancestor_record_offset":2052,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1454,"record_offset":2720,"parent_record_offset":2692,"ancestor_record_offset":2052,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1454,"record_offset":4684,"parent_record_offset":3856,"ancestor_record_offset":2052,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1454,"record_offset":4288,"parent_record_offset":4000,"ancestor_record_offset":2052,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1454,"record_offset":3188,"parent_record_offset":3164,"ancestor_record_offset":2052,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1454,"record_offset":3164,"parent_record_offset":3140,"ancestor_record_offset":2052,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1454,"record_offset":3372,"parent_record_offset":3344,"ancestor_record_offset":2052,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1454,"record_offset":4180,"parent_record_offset":4156,"ancestor_record_offset":2052,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1454,"record_offset":3116,"parent_record_offset":3092,"ancestor_record_offset":2052,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1454,"record_offset":3140,"parent_record_offset":3116,"ancestor_record_offset":2052,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1454,"record_offset":3444,"parent_record_offset":3416,"ancestor_record_offset":2052,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1454,"record_offset":3092,"parent_record_offset":2976,"ancestor_record_offset":2052,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1454,"record_offset":2432,"parent_record_offset":2404,"ancestor_record_offset":2052,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1454,"record_offset":2804,"parent_record_offset":2772,"ancestor_record_offset":2052,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1454,"record_offset":2944,"parent_record_offset":2912,"ancestor_record_offset":2052,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1454,"record_offset":2912,"parent_record_offset":2876,"ancestor_record_offset":2052,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1454,"record_offset":2852,"parent_record_offset":2828,"ancestor_record_offset":2052,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1454,"record_offset":2876,"parent_record_offset":2852,"ancestor_record_offset":2052,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1454,"record_offset":2828,"parent_record_offset":2804,"ancestor_record_offset":2052,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1454,"record_offset":3416,"parent_record_offset":2876,"ancestor_record_offset":2052,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1454,"record_offset":2404,"parent_record_offset":2372,"ancestor_record_offset":2052,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1454,"record_offset":2372,"parent_record_offset":2344,"ancestor_record_offset":2052,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1454,"record_offset":2572,"parent_record_offset":2536,"ancestor_record_offset":2052,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1454,"record_offset":2668,"parent_record_offset":2572,"ancestor_record_offset":2052,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1454,"record_offset":2536,"parent_record_offset":2508,"ancestor_record_offset":2052,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1454,"record_offset":2344,"parent_record_offset":2304,"ancestor_record_offset":2052,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1454,"record_offset":2692,"parent_record_offset":2668,"ancestor_record_offset":2052,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1454,"record_offset":2976,"parent_record_offset":2944,"ancestor_record_offset":2052,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1454,"record_offset":3260,"parent_record_offset":2976,"ancestor_record_offset":2052,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1454,"record_offset":3232,"parent_record_offset":2976,"ancestor_record_offset":2052,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1454,"record_offset":3036,"parent_record_offset":2976,"ancestor_record_offset":2052,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1454,"record_offset":3792,"parent_record_offset":3760,"ancestor_record_offset":2052,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1454,"record_offset":4848,"parent_record_offset":4816,"ancestor_record_offset":2052,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1454,"record_offset":5348,"parent_record_offset":5088,"ancestor_record_offset":2052,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1454,"record_offset":3924,"parent_record_offset":3896,"ancestor_record_offset":2052,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1454,"record_offset":4000,"parent_record_offset":3856,"ancestor_record_offset":2052,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1454,"record_offset":4260,"parent_record_offset":4000,"ancestor_record_offset":2052,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1454,"record_offset":3952,"parent_record_offset":3924,"ancestor_record_offset":2052,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1454,"record_offset":3856,"parent_record_offset":3824,"ancestor_record_offset":2052,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1454,"record_offset":3824,"parent_record_offset":3792,"ancestor_record_offset":2052,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_fences::process_segment::emit_fences::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11441
// 首条位置: {"module":1413,"inlinee":70721}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1413,"record_offset":3276,"parent_record_offset":2668,"ancestor_record_offset":2668,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1413,"record_offset":2824,"parent_record_offset":2668,"ancestor_record_offset":2668,"inlinee":7077,"source_line":1691,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1413,"record_offset":2724,"parent_record_offset":2668,"ancestor_record_offset":2668,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1413,"record_offset":3344,"parent_record_offset":3312,"ancestor_record_offset":2668,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1413,"record_offset":3392,"parent_record_offset":3368,"ancestor_record_offset":2668,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1413,"record_offset":3368,"parent_record_offset":3344,"ancestor_record_offset":2668,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1413,"record_offset":3312,"parent_record_offset":3276,"ancestor_record_offset":2668,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1413,"record_offset":3224,"parent_record_offset":3196,"ancestor_record_offset":2668,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1413,"record_offset":3160,"parent_record_offset":3132,"ancestor_record_offset":2668,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":1413,"record_offset":2980,"parent_record_offset":2944,"ancestor_record_offset":2668,"inlinee":5527,"source_line":45,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1413,"record_offset":2944,"parent_record_offset":2900,"ancestor_record_offset":2668,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1413,"record_offset":2788,"parent_record_offset":2752,"ancestor_record_offset":2668,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1413,"record_offset":3096,"parent_record_offset":2980,"ancestor_record_offset":2668,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1413,"record_offset":3060,"parent_record_offset":3016,"ancestor_record_offset":2668,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1413,"record_offset":3432,"parent_record_offset":3276,"ancestor_record_offset":2668,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1413,"record_offset":2752,"parent_record_offset":2724,"ancestor_record_offset":2668,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1413,"record_offset":3016,"parent_record_offset":2980,"ancestor_record_offset":2668,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1413,"record_offset":3196,"parent_record_offset":2900,"ancestor_record_offset":2668,"inlinee":10960,"source_line":194,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1413,"record_offset":3132,"parent_record_offset":2900,"ancestor_record_offset":2668,"inlinee":10957,"source_line":182,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_fences::process_segment::emit_fences::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_fences/process_segment.rs
//     evidence={"module":1413,"record_offset":2852,"parent_record_offset":2824,"ancestor_record_offset":2668,"inlinee":70723,"source_line":40,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::bounding_box_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1413,"record_offset":2876,"parent_record_offset":2852,"ancestor_record_offset":2668,"inlinee":30460,"source_line":1082,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::aabb::Aabb2::from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":1413,"record_offset":2900,"parent_record_offset":2876,"ancestor_record_offset":2668,"inlinee":30462,"source_line":92,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_fences::process_segment::emit_fences::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11443
// 首条位置: {"module":1413,"inlinee":70723}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::Curve::bounding_box_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1413,"record_offset":2876,"parent_record_offset":2852,"ancestor_record_offset":2852,"inlinee":30460,"source_line":1082,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1413,"record_offset":3224,"parent_record_offset":3196,"ancestor_record_offset":2852,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1413,"record_offset":3160,"parent_record_offset":3132,"ancestor_record_offset":2852,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":1413,"record_offset":2980,"parent_record_offset":2944,"ancestor_record_offset":2852,"inlinee":5527,"source_line":45,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1413,"record_offset":2944,"parent_record_offset":2900,"ancestor_record_offset":2852,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1413,"record_offset":3096,"parent_record_offset":2980,"ancestor_record_offset":2852,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1413,"record_offset":3060,"parent_record_offset":3016,"ancestor_record_offset":2852,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1413,"record_offset":3016,"parent_record_offset":2980,"ancestor_record_offset":2852,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1413,"record_offset":3196,"parent_record_offset":2900,"ancestor_record_offset":2852,"inlinee":10960,"source_line":194,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1413,"record_offset":3132,"parent_record_offset":2900,"ancestor_record_offset":2852,"inlinee":10957,"source_line":182,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::aabb::Aabb2::from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":1413,"record_offset":2900,"parent_record_offset":2876,"ancestor_record_offset":2852,"inlinee":30462,"source_line":92,"archive":"_pdb/B/modules/01413.jsonl.gz"}
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
// 首条位置: {"module":1454,"record_offset":6176,"rva":19107200,"va":5387816320}
// 编译实例 RVAs: 0x1238d80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400885d0
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18551320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387816320,"raw_hex":"e8e8ede4fe","size":5,"soft_fail":false,"target_va":5369267664,"va":5387818979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14014ba20
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17754193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387816320,"raw_hex":"e8af17f1fe","size":5,"soft_fail":false,"target_va":5370067488,"va":5387821676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12735098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387816320,"raw_hex":"e886ad3dff","size":5,"soft_fail":false,"target_va":5375081488,"va":5387816581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12735796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387816320,"raw_hex":"e8ccaa3dff","size":5,"soft_fail":false,"target_va":5375081504,"va":5387817295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5883410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387816320,"raw_hex":"e8ee39a6ff","size":5,"soft_fail":false,"target_va":5381934528,"va":5387817933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f8ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-262750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387816320,"raw_hex":"e8a2fdfbff","size":5,"soft_fail":false,"target_va":5387554512,"va":5387817257,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f9be0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-259185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387816320,"raw_hex":"e88f0bfcff","size":5,"soft_fail":false,"target_va":5387557856,"va":5387817036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14120c430
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-185319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387816320,"raw_hex":"e8192cfdff","size":5,"soft_fail":false,"target_va":5387633712,"va":5387819026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t24030174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387816320,"raw_hex":"e8deab6e01","size":5,"soft_fail":false,"target_va":5411847168,"va":5387816989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e60
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t24030540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387816320,"raw_hex":"e84cad6e01","size":5,"soft_fail":false,"target_va":5411847776,"va":5387817231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923ee0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t24028487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387816320,"raw_hex":"e847a56e01","size":5,"soft_fail":false,"target_va":5411847904,"va":5387819412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-11192247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387816320,"raw_hex":"e8493855ff","size":5,"soft_fail":false,"target_va":5376625744,"va":5387817986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23724354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387816320,"raw_hex":"e842016a01","size":5,"soft_fail":false,"target_va":5411540963,"va":5387816604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23964596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387816320,"raw_hex":"e8b4ab6d01","size":5,"soft_fail":false,"target_va":5411785562,"va":5387820961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23721308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387816320,"raw_hex":"e85cf56901","size":5,"soft_fail":false,"target_va":5411542296,"va":5387820983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23721833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387816320,"raw_hex":"e869f76901","size":5,"soft_fail":false,"target_va":5411542880,"va":5387821042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23722229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387816320,"raw_hex":"e8f5f86901","size":5,"soft_fail":false,"target_va":5411543104,"va":5387820870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_overflow_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23722369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387816320,"raw_hex":"e881f96901","size":5,"soft_fail":false,"target_va":5411543200,"va":5387820826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23722269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387816320,"raw_hex":"e81df96901","size":5,"soft_fail":false,"target_va":5411543120,"va":5387820846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::render::Affine3Packed::from_cols
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10076896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387816320,"raw_hex":"e8203d66ff","size":5,"soft_fail":false,"target_va":5377742080,"va":5387818971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] perchance::PerchanceContext::uniform_disc_vec2
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4114780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387816320,"raw_hex":"e85cc93e00","size":5,"soft_fail":false,"target_va":5391932592,"va":5387817807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] perchance::PerchanceContext::uniform_f32
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4114305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387816320,"raw_hex":"e881c73e00","size":5,"soft_fail":false,"target_va":5391932128,"va":5387817818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] perchance::PerchanceContext::usize_less_than
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4111962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387816320,"raw_hex":"e85abe3e00","size":5,"soft_fail":false,"target_va":5391931984,"va":5387820017,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t14314397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387816320,"raw_hex":"e89d6bda00","size":5,"soft_fail":false,"target_va":5402130912,"va":5387816510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::max_extent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t29196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387816320,"raw_hex":"e80c720000","size":5,"soft_fail":false,"target_va":5387846320,"va":5387817119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1454,"record_offset":7724,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1454,"record_offset":6584,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1454,"record_offset":14476,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":11953,"source_line":1063,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1454,"record_offset":7048,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1454,"record_offset":14532,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1454,"record_offset":8628,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1454,"record_offset":6964,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1454,"record_offset":7556,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1454,"record_offset":15616,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1454,"record_offset":16292,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":8865,"source_line":217,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1454,"record_offset":16108,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1454,"record_offset":14756,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1454,"record_offset":15860,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$104::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1454,"record_offset":16080,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":8505,"source_line":17,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1454,"record_offset":15676,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1454,"record_offset":13092,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1454,"record_offset":15772,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1454,"record_offset":13572,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$88::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1454,"record_offset":15388,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":13727,"source_line":1809,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1454,"record_offset":13716,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1454,"record_offset":8432,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] perchance::PerchanceContext::choose
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":1454,"record_offset":15644,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":71072,"source_line":203,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] perchance::PerchanceContext::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":1454,"record_offset":6528,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":71017,"source_line":144,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1454,"record_offset":8600,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":13632,"source_line":70,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1454,"record_offset":14448,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":11009,"source_line":48,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_fences::process_segment::place_fence_piece
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_fences/process_segment.rs
//     evidence={"module":1454,"record_offset":13208,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":71069,"source_line":307,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_fences::process_segment::process_segment::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_fences/process_segment.rs
//     evidence={"module":1454,"record_offset":9212,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":71067,"source_line":130,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1454,"record_offset":8300,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":24512,"source_line":264,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1454,"record_offset":8572,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":9593,"source_line":169,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1454,"record_offset":8368,"parent_record_offset":6176,"ancestor_record_offset":6176,"inlinee":13649,"source_line":233,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1454,"record_offset":6884,"parent_record_offset":6860,"ancestor_record_offset":6176,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1454,"record_offset":6908,"parent_record_offset":6884,"ancestor_record_offset":6176,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1454,"record_offset":8840,"parent_record_offset":8816,"ancestor_record_offset":6176,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1454,"record_offset":6860,"parent_record_offset":6684,"ancestor_record_offset":6176,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1454,"record_offset":8816,"parent_record_offset":8724,"ancestor_record_offset":6176,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1454,"record_offset":9804,"parent_record_offset":9772,"ancestor_record_offset":6176,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1454,"record_offset":6632,"parent_record_offset":6608,"ancestor_record_offset":6176,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1454,"record_offset":8752,"parent_record_offset":8724,"ancestor_record_offset":6176,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1454,"record_offset":8724,"parent_record_offset":8700,"ancestor_record_offset":6176,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1454,"record_offset":9852,"parent_record_offset":9828,"ancestor_record_offset":6176,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1454,"record_offset":9828,"parent_record_offset":9804,"ancestor_record_offset":6176,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1454,"record_offset":6684,"parent_record_offset":6656,"ancestor_record_offset":6176,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1454,"record_offset":6656,"parent_record_offset":6632,"ancestor_record_offset":6176,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1454,"record_offset":8700,"parent_record_offset":8676,"ancestor_record_offset":6176,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1454,"record_offset":6720,"parent_record_offset":6684,"ancestor_record_offset":6176,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1454,"record_offset":7820,"parent_record_offset":7788,"ancestor_record_offset":6176,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1454,"record_offset":7788,"parent_record_offset":7756,"ancestor_record_offset":6176,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1454,"record_offset":7756,"parent_record_offset":7724,"ancestor_record_offset":6176,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1454,"record_offset":9772,"parent_record_offset":9716,"ancestor_record_offset":6176,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1454,"record_offset":9716,"parent_record_offset":9212,"ancestor_record_offset":6176,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1454,"record_offset":6608,"parent_record_offset":6584,"ancestor_record_offset":6176,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1454,"record_offset":6744,"parent_record_offset":6720,"ancestor_record_offset":6176,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1454,"record_offset":6768,"parent_record_offset":6744,"ancestor_record_offset":6176,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1454,"record_offset":14500,"parent_record_offset":14476,"ancestor_record_offset":6176,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1454,"record_offset":8396,"parent_record_offset":8368,"ancestor_record_offset":6176,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1454,"record_offset":8252,"parent_record_offset":8228,"ancestor_record_offset":6176,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1454,"record_offset":7280,"parent_record_offset":7256,"ancestor_record_offset":6176,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1454,"record_offset":7328,"parent_record_offset":7304,"ancestor_record_offset":6176,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1454,"record_offset":14556,"parent_record_offset":14532,"ancestor_record_offset":6176,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1454,"record_offset":7352,"parent_record_offset":7328,"ancestor_record_offset":6176,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1454,"record_offset":6792,"parent_record_offset":6768,"ancestor_record_offset":6176,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1454,"record_offset":7684,"parent_record_offset":7648,"ancestor_record_offset":6176,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1454,"record_offset":6816,"parent_record_offset":6792,"ancestor_record_offset":6176,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1454,"record_offset":8784,"parent_record_offset":8752,"ancestor_record_offset":6176,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1454,"record_offset":8540,"parent_record_offset":8432,"ancestor_record_offset":6176,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=50
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1454,"record_offset":8484,"parent_record_offset":8432,"ancestor_record_offset":6176,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$327::neg
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1454,"record_offset":15416,"parent_record_offset":15388,"ancestor_record_offset":6176,"inlinee":11306,"source_line":694,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1454,"record_offset":15800,"parent_record_offset":15772,"ancestor_record_offset":6176,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1454,"record_offset":8456,"parent_record_offset":8432,"ancestor_record_offset":6176,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::into_slice_range
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1454,"record_offset":7620,"parent_record_offset":7584,"ancestor_record_offset":6176,"inlinee":4608,"source_line":454,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1454,"record_offset":7304,"parent_record_offset":7280,"ancestor_record_offset":6176,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1454,"record_offset":8228,"parent_record_offset":7820,"ancestor_record_offset":6176,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1454,"record_offset":8652,"parent_record_offset":8628,"ancestor_record_offset":6176,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1454,"record_offset":7012,"parent_record_offset":6988,"ancestor_record_offset":6176,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1454,"record_offset":7216,"parent_record_offset":7192,"ancestor_record_offset":6176,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1454,"record_offset":7140,"parent_record_offset":7104,"ancestor_record_offset":6176,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1454,"record_offset":9892,"parent_record_offset":9716,"ancestor_record_offset":6176,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1454,"record_offset":8328,"parent_record_offset":8300,"ancestor_record_offset":6176,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1454,"record_offset":7584,"parent_record_offset":7556,"ancestor_record_offset":6176,"inlinee":4606,"source_line":680,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1454,"record_offset":7648,"parent_record_offset":7584,"ancestor_record_offset":6176,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1454,"record_offset":6988,"parent_record_offset":6964,"ancestor_record_offset":6176,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1454,"record_offset":7192,"parent_record_offset":7168,"ancestor_record_offset":6176,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1454,"record_offset":7104,"parent_record_offset":7072,"ancestor_record_offset":6176,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1454,"record_offset":7168,"parent_record_offset":7104,"ancestor_record_offset":6176,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] glam::core::scalar::vector::impl$9::mul_scalar
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.20.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.20.5/src/core/scalar/vector.rs
//     evidence={"module":1454,"record_offset":9328,"parent_record_offset":9304,"ancestor_record_offset":6176,"inlinee":29997,"source_line":238,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1454,"record_offset":10352,"parent_record_offset":10320,"ancestor_record_offset":6176,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1454,"record_offset":10320,"parent_record_offset":10288,"ancestor_record_offset":6176,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1454,"record_offset":10288,"parent_record_offset":10264,"ancestor_record_offset":6176,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1454,"record_offset":10264,"parent_record_offset":9212,"ancestor_record_offset":6176,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1454,"record_offset":10176,"parent_record_offset":9212,"ancestor_record_offset":6176,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1454,"record_offset":9668,"parent_record_offset":9212,"ancestor_record_offset":6176,"inlinee":8865,"source_line":217,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1454,"record_offset":9564,"parent_record_offset":9532,"ancestor_record_offset":6176,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1454,"record_offset":9532,"parent_record_offset":9500,"ancestor_record_offset":6176,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1454,"record_offset":9500,"parent_record_offset":9472,"ancestor_record_offset":6176,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1454,"record_offset":9472,"parent_record_offset":9212,"ancestor_record_offset":6176,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1454,"record_offset":9940,"parent_record_offset":9212,"ancestor_record_offset":6176,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1454,"record_offset":10028,"parent_record_offset":9212,"ancestor_record_offset":6176,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1454,"record_offset":16132,"parent_record_offset":16108,"ancestor_record_offset":6176,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] glam::vec2::impl$25::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.20.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.20.5/src/vec.rs
//     evidence={"module":1454,"record_offset":9304,"parent_record_offset":9212,"ancestor_record_offset":6176,"inlinee":29995,"source_line":576,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1454,"record_offset":7256,"parent_record_offset":7072,"ancestor_record_offset":6176,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1454,"record_offset":7072,"parent_record_offset":7048,"ancestor_record_offset":6176,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] perchance::PerchanceContext::uniform_range_f32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":1454,"record_offset":10120,"parent_record_offset":9212,"ancestor_record_offset":6176,"inlinee":26762,"source_line":270,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] perchance::PerchanceContext::uniform_range_f32_raw
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":1454,"record_offset":10144,"parent_record_offset":10120,"ancestor_record_offset":6176,"inlinee":26763,"source_line":275,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] perchance::Rng::from_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":1454,"record_offset":6552,"parent_record_offset":6528,"ancestor_record_offset":6176,"inlinee":71019,"source_line":66,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powi
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1454,"record_offset":10484,"parent_record_offset":9212,"ancestor_record_offset":6176,"inlinee":14226,"source_line":314,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1454,"record_offset":9596,"parent_record_offset":9564,"ancestor_record_offset":6176,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_fences::construct_fences::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_fences/mod.rs
//     evidence={"module":1454,"record_offset":13536,"parent_record_offset":13208,"ancestor_record_offset":6176,"inlinee":71071,"source_line":47,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_fences::process_segment::process_segment::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 130
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1159B
// 首条位置: {"module":1454,"inlinee":71067}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1454,"record_offset":9716,"parent_record_offset":9212,"ancestor_record_offset":9212,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1454,"record_offset":10264,"parent_record_offset":9212,"ancestor_record_offset":9212,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1454,"record_offset":10176,"parent_record_offset":9212,"ancestor_record_offset":9212,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1454,"record_offset":9668,"parent_record_offset":9212,"ancestor_record_offset":9212,"inlinee":8865,"source_line":217,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1454,"record_offset":9472,"parent_record_offset":9212,"ancestor_record_offset":9212,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1454,"record_offset":9940,"parent_record_offset":9212,"ancestor_record_offset":9212,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1454,"record_offset":10028,"parent_record_offset":9212,"ancestor_record_offset":9212,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] glam::vec2::impl$25::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.20.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.20.5/src/vec.rs
//     evidence={"module":1454,"record_offset":9304,"parent_record_offset":9212,"ancestor_record_offset":9212,"inlinee":29995,"source_line":576,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] perchance::PerchanceContext::uniform_range_f32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":1454,"record_offset":10120,"parent_record_offset":9212,"ancestor_record_offset":9212,"inlinee":26762,"source_line":270,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1454,"record_offset":10484,"parent_record_offset":9212,"ancestor_record_offset":9212,"inlinee":14226,"source_line":314,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1454,"record_offset":9804,"parent_record_offset":9772,"ancestor_record_offset":9212,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1454,"record_offset":9852,"parent_record_offset":9828,"ancestor_record_offset":9212,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1454,"record_offset":9828,"parent_record_offset":9804,"ancestor_record_offset":9212,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1454,"record_offset":9772,"parent_record_offset":9716,"ancestor_record_offset":9212,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1454,"record_offset":10060,"parent_record_offset":10028,"ancestor_record_offset":9212,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1454,"record_offset":9352,"parent_record_offset":9328,"ancestor_record_offset":9212,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1454,"record_offset":10204,"parent_record_offset":10176,"ancestor_record_offset":9212,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1454,"record_offset":9892,"parent_record_offset":9716,"ancestor_record_offset":9212,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] glam::core::scalar::vector::impl$9::mul_scalar
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.20.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.20.5/src/core/scalar/vector.rs
//     evidence={"module":1454,"record_offset":9328,"parent_record_offset":9304,"ancestor_record_offset":9212,"inlinee":29997,"source_line":238,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1454,"record_offset":10352,"parent_record_offset":10320,"ancestor_record_offset":9212,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1454,"record_offset":10320,"parent_record_offset":10288,"ancestor_record_offset":9212,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1454,"record_offset":10288,"parent_record_offset":10264,"ancestor_record_offset":9212,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1454,"record_offset":9564,"parent_record_offset":9532,"ancestor_record_offset":9212,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1454,"record_offset":9532,"parent_record_offset":9500,"ancestor_record_offset":9212,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1454,"record_offset":9500,"parent_record_offset":9472,"ancestor_record_offset":9212,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] perchance::PerchanceContext::uniform_range_f32_raw
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":1454,"record_offset":10144,"parent_record_offset":10120,"ancestor_record_offset":9212,"inlinee":26763,"source_line":275,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1454,"record_offset":9596,"parent_record_offset":9564,"ancestor_record_offset":9212,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01454.jsonl.gz"}
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
// 首条位置: {"module":3027,"record_offset":272,"rva":33421792,"va":5402130912}
// 编译实例 RVAs: 0x1fdf9e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27049512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402130912,"raw_hex":"e8d84163fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5402130995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27050301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402130912,"raw_hex":"e8c33e63fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5402131800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14123ac50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14307003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402130912,"raw_hex":"e845b125ff","size":5,"soft_fail":false,"target_va":5387824208,"va":5402131206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9716833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402130912,"raw_hex":"e861449400","size":5,"soft_fail":false,"target_va":5411847808,"va":5402130970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9409256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402130912,"raw_hex":"e8e8928f00","size":5,"soft_fail":false,"target_va":5411540992,"va":5402131731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-21356126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402130912,"raw_hex":"e8a221bafe","size":5,"soft_fail":false,"target_va":5380775408,"va":5402131529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9653898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402130912,"raw_hex":"e88a4e9300","size":5,"soft_fail":false,"target_va":5411785562,"va":5402131659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9410577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402130912,"raw_hex":"e811988f00","size":5,"soft_fail":false,"target_va":5411542296,"va":5402131714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3027,"record_offset":504,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3027,"record_offset":652,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3027,"record_offset":736,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3027,"record_offset":920,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3027,"record_offset":2292,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$104::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3027,"record_offset":1664,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8505,"source_line":17,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3027,"record_offset":1740,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3027,"record_offset":476,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":14893,"source_line":675,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_direct] utils::cos_angle_between
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3027,"record_offset":1916,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":70918,"source_line":488,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3027,"record_offset":560,"parent_record_offset":536,"ancestor_record_offset":272,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3027,"record_offset":584,"parent_record_offset":560,"ancestor_record_offset":272,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3027,"record_offset":2480,"parent_record_offset":2456,"ancestor_record_offset":272,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3027,"record_offset":536,"parent_record_offset":504,"ancestor_record_offset":272,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3027,"record_offset":2456,"parent_record_offset":2364,"ancestor_record_offset":272,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3027,"record_offset":800,"parent_record_offset":776,"ancestor_record_offset":272,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3027,"record_offset":2392,"parent_record_offset":2364,"ancestor_record_offset":272,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3027,"record_offset":2364,"parent_record_offset":2340,"ancestor_record_offset":272,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3027,"record_offset":848,"parent_record_offset":824,"ancestor_record_offset":272,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3027,"record_offset":824,"parent_record_offset":800,"ancestor_record_offset":272,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3027,"record_offset":2340,"parent_record_offset":2316,"ancestor_record_offset":272,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3027,"record_offset":676,"parent_record_offset":652,"ancestor_record_offset":272,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3027,"record_offset":776,"parent_record_offset":736,"ancestor_record_offset":272,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3027,"record_offset":700,"parent_record_offset":676,"ancestor_record_offset":272,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::Clone::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\clone.rs
//     evidence={"module":3027,"record_offset":1624,"parent_record_offset":1596,"ancestor_record_offset":272,"inlinee":4549,"source_line":165,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3027,"record_offset":1088,"parent_record_offset":1060,"ancestor_record_offset":272,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3027,"record_offset":1564,"parent_record_offset":1540,"ancestor_record_offset":272,"inlinee":4545,"source_line":36,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":3027,"record_offset":1008,"parent_record_offset":960,"ancestor_record_offset":272,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":3027,"record_offset":1116,"parent_record_offset":1060,"ancestor_record_offset":272,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":3027,"record_offset":1060,"parent_record_offset":1036,"ancestor_record_offset":272,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":3027,"record_offset":1036,"parent_record_offset":1008,"ancestor_record_offset":272,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3027,"record_offset":1540,"parent_record_offset":960,"ancestor_record_offset":272,"inlinee":4543,"source_line":468,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":3027,"record_offset":1304,"parent_record_offset":1280,"ancestor_record_offset":272,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3027,"record_offset":1140,"parent_record_offset":1116,"ancestor_record_offset":272,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3027,"record_offset":2424,"parent_record_offset":2392,"ancestor_record_offset":272,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3027,"record_offset":1768,"parent_record_offset":1740,"ancestor_record_offset":272,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3027,"record_offset":1208,"parent_record_offset":1180,"ancestor_record_offset":272,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3027,"record_offset":1180,"parent_record_offset":1008,"ancestor_record_offset":272,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3027,"record_offset":1596,"parent_record_offset":960,"ancestor_record_offset":272,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3027,"record_offset":2316,"parent_record_offset":2292,"ancestor_record_offset":272,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3027,"record_offset":1332,"parent_record_offset":1304,"ancestor_record_offset":272,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3027,"record_offset":608,"parent_record_offset":584,"ancestor_record_offset":272,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3027,"record_offset":888,"parent_record_offset":736,"ancestor_record_offset":272,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3027,"record_offset":1940,"parent_record_offset":1916,"ancestor_record_offset":272,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3027,"record_offset":1968,"parent_record_offset":1916,"ancestor_record_offset":272,"inlinee":25585,"source_line":406,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$30::left_shift_push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":3027,"record_offset":1280,"parent_record_offset":960,"ancestor_record_offset":272,"inlinee":25584,"source_line":388,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":3027,"record_offset":960,"parent_record_offset":920,"ancestor_record_offset":272,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3027,"record_offset":2080,"parent_record_offset":1916,"ancestor_record_offset":272,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_fences/process_segment.rs
//     evidence={"module":3027,"record_offset":1236,"parent_record_offset":1208,"ancestor_record_offset":272,"inlinee":71047,"source_line":286,"archive":"_pdb/B/modules/03027.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 286
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11587
// 首条位置: {"module":1452,"inlinee":71047}
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
// IPI IDs: 0x1159D
// 首条位置: {"module":1454,"inlinee":71069}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1454,"record_offset":13252,"parent_record_offset":13208,"ancestor_record_offset":13208,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_fences::construct_fences::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_fences/mod.rs
//     evidence={"module":1454,"record_offset":13536,"parent_record_offset":13208,"ancestor_record_offset":13208,"inlinee":71071,"source_line":47,"archive":"_pdb/B/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1454,"record_offset":13288,"parent_record_offset":13252,"ancestor_record_offset":13208,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01454.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_fences::process_segment::emit_fences::closure$1
// 信息 ID: 38b0ab07be9ca99c4c687237
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_fences::process_segment::emit_fences", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$1 <- utils::threshold_range::threshold_range<system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$0<system_wall_constructor::construct_fences::construct_fences::closure_env$0>,system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$1<system_wall_constructor::construct_fences::construct_fences::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] utils::threshold_range::threshold_range<system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$0<system_wall_constructor::construct_fences::construct_fences::closure_env$0>,system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$1<system_wall_constructor::construct_fences::construct_fences::closure_env$0> >; module=1413, inline_record=2668; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_fences::process_segment::emit_fences::closure$1::closure$0
// 信息 ID: 17c919cc22c6612079bf1fe0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_fences::process_segment::emit_fences", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::threshold_range::threshold_range<system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$0<system_wall_constructor::construct_fences::construct_fences::closure_env$0>,system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$1<system_wall_constructor::construct_fences::construct_fences::closure_env$0> >; module=1413, inline_record=2852; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_fences::process_segment::emit_fences<system_wall_constructor::construct_fences::construct_fences::closure_env$0>
// 信息 ID: 611b67b65725b71bfefc3d60
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::construct_fences::process_segment::emit_fences -> ["system_wall_constructor::construct_fences::construct_fences::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_fences::construct_fences @ 0x765ddc -> 0x12388f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_fences::process_segment::place_fence_piece
// 信息 ID: fa6a8577c55a5ffe10010948
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_fences::process_segment::process_segment<system_wall_constructor::construct_fences::construct_fences::closure_env$0>; module=1454, inline_record=13208; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_fences::process_segment::process_segment<system_wall_constructor::construct_fences::construct_fences::closure_env$0>; module=1454, inline_record=16604; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_fences::process_segment::process_segment::closure$0
// 信息 ID: 4186b470220814cf304ed9d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_fences::process_segment::process_segment", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_fences::process_segment::process_segment<system_wall_constructor::construct_fences::construct_fences::closure_env$0>; module=1454, inline_record=9212; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_fences::process_segment::process_segment<system_wall_constructor::construct_fences::construct_fences::closure_env$0>; module=1454, inline_record=11024; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_fences::process_segment::process_segment<system_wall_constructor::construct_fences::construct_fences::closure_env$0>
// 信息 ID: 6fc5ece6ac753a9382f0de5f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::construct_fences::process_segment::process_segment -> ["system_wall_constructor::construct_fences::construct_fences::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::threshold_range::threshold_range<system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$0<system_wall_constructor::construct_fences::construct_fences::closure_env$0>,system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$1<system_wall_constructor::construct_fences::construct_fences::closure_env$0> > @ 0x120019d -> 0x1238d80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle
// 信息 ID: 3426706d38320bb33379c693
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_fences::process_segment::process_segment<system_wall_constructor::construct_fences::construct_fences::closure_env$0> @ 0x1238e3e -> 0x1fdf9e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle::closure$0
// 信息 ID: 3f403effa90558d3b1c4f6e5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle::closure_env$0 <- itertools::tuple_impl::impl$30::collect_from_iter_no_buf<ref$<glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<ref$<glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] itertools::tuple_impl::impl$30::collect_from_iter_no_buf<ref$<glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<ref$<glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle::closure_env$0> > > >; module=1455, inline_record=17608; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$30::collect_from_iter_no_buf<ref$<glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<ref$<glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle::closure_env$0> > > >; module=1455, inline_record=18052; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$30::collect_from_iter_no_buf<ref$<glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<ref$<glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle::closure_env$0> > > >; module=1455, inline_record=18600; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$30::collect_from_iter_no_buf<ref$<glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<ref$<glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle::closure_env$0> > > >; module=1455, inline_record=18836; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle; module=3027, inline_record=1236; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
