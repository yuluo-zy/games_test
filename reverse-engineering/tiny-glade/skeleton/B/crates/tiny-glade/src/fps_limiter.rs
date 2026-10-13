// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tiny-glade/src/fps_limiter.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: d37ef552b91bb09e5c007ac8412df4088a7ab5c348b067069a035c134ec9a2bf
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: fps_limiter.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 92
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use once_cell; // namespace candidate
//   package identities: [{"name":"once_cell","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.21.3"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use spin_sleep; // namespace candidate
//   package identities: [{"name":"spin_sleep","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.1.1"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::fps_limiter::FpsLimiter::frame
// 记录形式: compiled_procedure; instances=1
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":92,"record_offset":29576,"rva":2558032,"va":5371267152}
// 编译实例 RVAs: 0x270850
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a2580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: fps_limiter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1894455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371267152,"raw_hex":"e8c917e3ff","size":5,"soft_fail":false,"target_va":5369374080,"va":5371268530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d63680
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: fps_limiter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11480127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371267152,"raw_hex":"e83f2caf00","size":5,"soft_fail":false,"target_va":5382747776,"va":5371267644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a1640
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: fps_limiter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40045018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371267152,"raw_hex":"e8da096302","size":5,"soft_fail":false,"target_va":5411313216,"va":5371268193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a43c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: fps_limiter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40056943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371267152,"raw_hex":"e86f386302","size":5,"soft_fail":false,"target_va":5411324864,"va":5371267916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428ce090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: fps_limiter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40227748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371267152,"raw_hex":"e8a4d36502","size":5,"soft_fail":false,"target_va":5411496080,"va":5371268327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1429234a0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: fps_limiter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40577350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371267152,"raw_hex":"e846296b02","size":5,"soft_fail":false,"target_va":5411845280,"va":5371267925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1429234b0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: fps_limiter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40577323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371267152,"raw_hex":"e82b296b02","size":5,"soft_fail":false,"target_va":5411845296,"va":5371267968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40273302","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371267152,"raw_hex":"e896856602","size":5,"soft_fail":false,"target_va":5411541584,"va":5371268277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40275867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371267152,"raw_hex":"e89b8f6602","size":5,"soft_fail":false,"target_va":5411543360,"va":5371267488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10117936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371267152,"raw_hex":"e830639a00","size":5,"soft_fail":false,"target_va":5381385536,"va":5371267595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10123129","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371267152,"raw_hex":"e879779a00","size":5,"soft_fail":false,"target_va":5381390640,"va":5371267506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10118422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371267152,"raw_hex":"e816659a00","size":5,"soft_fail":false,"target_va":5381386480,"va":5371268053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10117480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371267152,"raw_hex":"e868619a00","size":5,"soft_fail":false,"target_va":5381384704,"va":5371267219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40268834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371267152,"raw_hex":"e822746602","size":5,"soft_fail":false,"target_va":5411537104,"va":5371268265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::sleep
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11423260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371267152,"raw_hex":"e81c4eae00","size":5,"soft_fail":false,"target_va":5382691200,"va":5371267935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::now
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11480416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371267152,"raw_hex":"e8602daf00","size":5,"soft_fail":false,"target_va":5382747616,"va":5371267195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":92,"record_offset":32788,"parent_record_offset":29576,"ancestor_record_offset":29576,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":92,"record_offset":32760,"parent_record_offset":29576,"ancestor_record_offset":29576,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":92,"record_offset":30708,"parent_record_offset":29576,"ancestor_record_offset":29576,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] core::time::Duration::as_secs_f32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":92,"record_offset":32728,"parent_record_offset":29576,"ancestor_record_offset":29576,"inlinee":11237,"source_line":853,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] core::time::Duration::from_secs_f32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":92,"record_offset":32820,"parent_record_offset":29576,"ancestor_record_offset":29576,"inlinee":11238,"source_line":964,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":92,"record_offset":31760,"parent_record_offset":29576,"ancestor_record_offset":29576,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":92,"record_offset":29732,"parent_record_offset":29576,"ancestor_record_offset":29576,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] spin_sleep::native_sleep
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"spin_sleep","version":"1.1.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/spin_sleep-1.1.1/src/lib.rs
//     evidence={"module":92,"record_offset":33036,"parent_record_offset":29576,"ancestor_record_offset":29576,"inlinee":11242,"source_line":115,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":92,"record_offset":31104,"parent_record_offset":31076,"ancestor_record_offset":29576,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":92,"record_offset":31156,"parent_record_offset":31132,"ancestor_record_offset":29576,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":92,"record_offset":31132,"parent_record_offset":31104,"ancestor_record_offset":29576,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":92,"record_offset":31052,"parent_record_offset":31024,"ancestor_record_offset":29576,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":92,"record_offset":31076,"parent_record_offset":31052,"ancestor_record_offset":29576,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":92,"record_offset":31248,"parent_record_offset":31076,"ancestor_record_offset":29576,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":92,"record_offset":31360,"parent_record_offset":31336,"ancestor_record_offset":29576,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$72::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\num.rs
//     evidence={"module":92,"record_offset":32972,"parent_record_offset":32860,"ancestor_record_offset":29576,"inlinee":10908,"source_line":77,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_bits
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":92,"record_offset":32944,"parent_record_offset":32860,"ancestor_record_offset":29576,"inlinee":11240,"source_line":1096,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":92,"record_offset":31180,"parent_record_offset":31156,"ancestor_record_offset":29576,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":92,"record_offset":30572,"parent_record_offset":30540,"ancestor_record_offset":29576,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":92,"record_offset":30920,"parent_record_offset":30880,"ancestor_record_offset":29576,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":92,"record_offset":30656,"parent_record_offset":30628,"ancestor_record_offset":29576,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":92,"record_offset":31708,"parent_record_offset":30880,"ancestor_record_offset":29576,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":92,"record_offset":30752,"parent_record_offset":30708,"ancestor_record_offset":29576,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":92,"record_offset":30144,"parent_record_offset":30120,"ancestor_record_offset":29576,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":92,"record_offset":30120,"parent_record_offset":30096,"ancestor_record_offset":29576,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":92,"record_offset":30328,"parent_record_offset":30300,"ancestor_record_offset":29576,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":92,"record_offset":31204,"parent_record_offset":31180,"ancestor_record_offset":29576,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":92,"record_offset":30072,"parent_record_offset":30048,"ancestor_record_offset":29576,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":92,"record_offset":30096,"parent_record_offset":30072,"ancestor_record_offset":29576,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":92,"record_offset":30400,"parent_record_offset":30372,"ancestor_record_offset":29576,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":92,"record_offset":30048,"parent_record_offset":29936,"ancestor_record_offset":29576,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":92,"record_offset":30540,"parent_record_offset":30508,"ancestor_record_offset":29576,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":92,"record_offset":29764,"parent_record_offset":29732,"ancestor_record_offset":29576,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":92,"record_offset":29904,"parent_record_offset":29872,"ancestor_record_offset":29576,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":92,"record_offset":29872,"parent_record_offset":29836,"ancestor_record_offset":29576,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":92,"record_offset":29812,"parent_record_offset":29788,"ancestor_record_offset":29576,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":92,"record_offset":29836,"parent_record_offset":29812,"ancestor_record_offset":29576,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":92,"record_offset":29788,"parent_record_offset":29764,"ancestor_record_offset":29576,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":92,"record_offset":30372,"parent_record_offset":29836,"ancestor_record_offset":29576,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":92,"record_offset":30508,"parent_record_offset":30476,"ancestor_record_offset":29576,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":92,"record_offset":30604,"parent_record_offset":30508,"ancestor_record_offset":29576,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":92,"record_offset":30476,"parent_record_offset":30452,"ancestor_record_offset":29576,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":92,"record_offset":30452,"parent_record_offset":29732,"ancestor_record_offset":29576,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":92,"record_offset":30628,"parent_record_offset":30604,"ancestor_record_offset":29576,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":92,"record_offset":29936,"parent_record_offset":29904,"ancestor_record_offset":29576,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":92,"record_offset":30216,"parent_record_offset":29936,"ancestor_record_offset":29576,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":92,"record_offset":30188,"parent_record_offset":29936,"ancestor_record_offset":29576,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":92,"record_offset":29992,"parent_record_offset":29936,"ancestor_record_offset":29576,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":92,"record_offset":33216,"parent_record_offset":33192,"ancestor_record_offset":29576,"inlinee":6050,"source_line":1566,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":92,"record_offset":33240,"parent_record_offset":33216,"ancestor_record_offset":29576,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::try_from_secs_f32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":92,"record_offset":32860,"parent_record_offset":32820,"ancestor_record_offset":29576,"inlinee":11239,"source_line":1602,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::imp::OnceCell::is_initialized
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/imp_std.rs
//     evidence={"module":92,"record_offset":33192,"parent_record_offset":33168,"ancestor_record_offset":29576,"inlinee":6048,"source_line":49,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::Lazy::force
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":92,"record_offset":33092,"parent_record_offset":33068,"ancestor_record_offset":29576,"inlinee":6042,"source_line":1308,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":92,"record_offset":33168,"parent_record_offset":33140,"ancestor_record_offset":29576,"inlinee":6046,"source_line":963,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":92,"record_offset":33116,"parent_record_offset":33092,"ancestor_record_offset":29576,"inlinee":6044,"source_line":1115,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_try_init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":92,"record_offset":33140,"parent_record_offset":33116,"ancestor_record_offset":29576,"inlinee":6045,"source_line":1152,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::impl$12::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":92,"record_offset":33068,"parent_record_offset":33036,"ancestor_record_offset":29576,"inlinee":6040,"source_line":1375,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":92,"record_offset":30816,"parent_record_offset":30784,"ancestor_record_offset":29576,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":92,"record_offset":30784,"parent_record_offset":30752,"ancestor_record_offset":29576,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":92,"record_offset":31284,"parent_record_offset":31024,"ancestor_record_offset":29576,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":92,"record_offset":30948,"parent_record_offset":30920,"ancestor_record_offset":29576,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":92,"record_offset":31024,"parent_record_offset":30880,"ancestor_record_offset":29576,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":92,"record_offset":32260,"parent_record_offset":32000,"ancestor_record_offset":29576,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":92,"record_offset":30976,"parent_record_offset":30948,"ancestor_record_offset":29576,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":92,"record_offset":30880,"parent_record_offset":30848,"ancestor_record_offset":29576,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":92,"record_offset":30848,"parent_record_offset":30816,"ancestor_record_offset":29576,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00092.jsonl.gz"}
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
// 完整结构及原始类型记录: fps_limiter.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::fps_limiter::FpsLimiter
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 1
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: fps_limiter.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: fps_limiter.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tiny_glade::fps_limiter::FpsLimiter::frame
// 信息 ID: 92f027f2584453d2f59e3ee4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x4de7da -> 0x270850 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
