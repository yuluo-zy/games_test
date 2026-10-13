// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/fps_limiter.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: d37ef552b91bb09e5c007ac8412df4088a7ab5c348b067069a035c134ec9a2bf
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: fps_limiter.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 52
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
// 首条位置: {"module":52,"record_offset":3588,"rva":1845728,"va":5370554848}
// 编译实例 RVAs: 0x1c29e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::time::Instant>::now
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14093132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370554848,"raw_hex":"e84c0bd700","size":5,"soft_fail":false,"target_va":5384648032,"va":5370554895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140139520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: fps_limiter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-563803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370554848,"raw_hex":"e8a565f7ff","size":5,"soft_fail":false,"target_va":5369992480,"va":5370556278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f33320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: fps_limiter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14092153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370554848,"raw_hex":"e87907d700","size":5,"soft_fail":false,"target_va":5384647456,"va":5370555298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d70990
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: fps_limiter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t45800362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370554848,"raw_hex":"e8aadbba02","size":5,"soft_fail":false,"target_va":5416356240,"va":5370555873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d71650
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: fps_limiter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t45803568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370554848,"raw_hex":"e830e8ba02","size":5,"soft_fail":false,"target_va":5416359504,"va":5370555931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1b2e0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: fps_limiter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46499144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370554848,"raw_hex":"e84885c502","size":5,"soft_fail":false,"target_va":5417054944,"va":5370555795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1b2f0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: fps_limiter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46499127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370554848,"raw_hex":"e83785c502","size":5,"soft_fail":false,"target_va":5417054960,"va":5370555828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t46065540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370554848,"raw_hex":"e884e7be02","size":5,"soft_fail":false,"target_va":5416621408,"va":5370555863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t46066196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370554848,"raw_hex":"e814eabe02","size":5,"soft_fail":false,"target_va":5416622240,"va":5370556039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t46065378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370554848,"raw_hex":"e8e2e6be02","size":5,"soft_fail":false,"target_va":5416621376,"va":5370555993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13408540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370554848,"raw_hex":"e81c99cc00","size":5,"soft_fail":false,"target_va":5383963792,"va":5370555247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13417051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370554848,"raw_hex":"e85bbacc00","size":5,"soft_fail":false,"target_va":5383972224,"va":5370555168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13409393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370554848,"raw_hex":"e8719ccc00","size":5,"soft_fail":false,"target_va":5383964912,"va":5370555514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13407887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370554848,"raw_hex":"e88f96cc00","size":5,"soft_fail":false,"target_va":5383962816,"va":5370554924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::functions::sleep
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t14172348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370554848,"raw_hex":"e8bc40d800","size":5,"soft_fail":false,"target_va":5384728160,"va":5370555807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t46063440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370554848,"raw_hex":"e850dfbe02","size":5,"soft_fail":false,"target_va":5416619360,"va":5370555915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":52,"record_offset":5620,"parent_record_offset":3588,"ancestor_record_offset":3588,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":52,"record_offset":5592,"parent_record_offset":3588,"ancestor_record_offset":3588,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":52,"record_offset":5880,"parent_record_offset":3588,"ancestor_record_offset":3588,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] core::time::Duration::as_secs_f32
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":52,"record_offset":5560,"parent_record_offset":3588,"ancestor_record_offset":3588,"inlinee":7360,"source_line":885,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] core::time::Duration::from_secs_f32
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":52,"record_offset":5652,"parent_record_offset":3588,"ancestor_record_offset":3588,"inlinee":9256,"source_line":996,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":52,"record_offset":4752,"parent_record_offset":3588,"ancestor_record_offset":3588,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":52,"record_offset":3744,"parent_record_offset":3588,"ancestor_record_offset":3588,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_direct] spin_sleep::native_sleep
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"spin_sleep","version":"1.1.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/spin_sleep-1.1.1/src/lib.rs
//     evidence={"module":52,"record_offset":6764,"parent_record_offset":3588,"ancestor_record_offset":3588,"inlinee":9262,"source_line":115,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":52,"record_offset":5080,"parent_record_offset":5052,"ancestor_record_offset":3588,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":52,"record_offset":5108,"parent_record_offset":5080,"ancestor_record_offset":3588,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":52,"record_offset":5028,"parent_record_offset":5000,"ancestor_record_offset":3588,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":52,"record_offset":5052,"parent_record_offset":5028,"ancestor_record_offset":3588,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":52,"record_offset":5168,"parent_record_offset":5052,"ancestor_record_offset":3588,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":52,"record_offset":5280,"parent_record_offset":5256,"ancestor_record_offset":3588,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$72::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\num.rs
//     evidence={"module":52,"record_offset":5816,"parent_record_offset":5688,"ancestor_record_offset":3588,"inlinee":9260,"source_line":79,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_bits
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":52,"record_offset":5788,"parent_record_offset":5688,"ancestor_record_offset":3588,"inlinee":9258,"source_line":1181,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":52,"record_offset":5132,"parent_record_offset":5108,"ancestor_record_offset":3588,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":52,"record_offset":4900,"parent_record_offset":4860,"ancestor_record_offset":3588,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":52,"record_offset":4700,"parent_record_offset":4672,"ancestor_record_offset":3588,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":52,"record_offset":5516,"parent_record_offset":4860,"ancestor_record_offset":3588,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":52,"record_offset":5232,"parent_record_offset":5000,"ancestor_record_offset":3588,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":52,"record_offset":4160,"parent_record_offset":4136,"ancestor_record_offset":3588,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":52,"record_offset":4136,"parent_record_offset":4112,"ancestor_record_offset":3588,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":52,"record_offset":4344,"parent_record_offset":4316,"ancestor_record_offset":3588,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":52,"record_offset":4444,"parent_record_offset":4416,"ancestor_record_offset":3588,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":52,"record_offset":4416,"parent_record_offset":4388,"ancestor_record_offset":3588,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":52,"record_offset":4088,"parent_record_offset":4064,"ancestor_record_offset":3588,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":52,"record_offset":4112,"parent_record_offset":4088,"ancestor_record_offset":3588,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":52,"record_offset":4472,"parent_record_offset":4444,"ancestor_record_offset":3588,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":52,"record_offset":4064,"parent_record_offset":3952,"ancestor_record_offset":3588,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":52,"record_offset":4620,"parent_record_offset":4592,"ancestor_record_offset":3588,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":52,"record_offset":3780,"parent_record_offset":3744,"ancestor_record_offset":3588,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":52,"record_offset":3920,"parent_record_offset":3888,"ancestor_record_offset":3588,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":52,"record_offset":3888,"parent_record_offset":3852,"ancestor_record_offset":3588,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":52,"record_offset":3828,"parent_record_offset":3804,"ancestor_record_offset":3588,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":52,"record_offset":3852,"parent_record_offset":3828,"ancestor_record_offset":3588,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":52,"record_offset":3804,"parent_record_offset":3780,"ancestor_record_offset":3588,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":52,"record_offset":4388,"parent_record_offset":3852,"ancestor_record_offset":3588,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":52,"record_offset":4592,"parent_record_offset":4560,"ancestor_record_offset":3588,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":52,"record_offset":4648,"parent_record_offset":4592,"ancestor_record_offset":3588,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":52,"record_offset":4560,"parent_record_offset":4532,"ancestor_record_offset":3588,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":52,"record_offset":4532,"parent_record_offset":3744,"ancestor_record_offset":3588,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":52,"record_offset":4672,"parent_record_offset":4648,"ancestor_record_offset":3588,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":52,"record_offset":3952,"parent_record_offset":3920,"ancestor_record_offset":3588,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":52,"record_offset":4232,"parent_record_offset":3952,"ancestor_record_offset":3588,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":52,"record_offset":4204,"parent_record_offset":3952,"ancestor_record_offset":3588,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":52,"record_offset":4008,"parent_record_offset":3952,"ancestor_record_offset":3588,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":52,"record_offset":6968,"parent_record_offset":6944,"ancestor_record_offset":3588,"inlinee":4833,"source_line":1700,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":52,"record_offset":6992,"parent_record_offset":6968,"ancestor_record_offset":3588,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::try_from_secs_f32
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":52,"record_offset":5688,"parent_record_offset":5652,"ancestor_record_offset":3588,"inlinee":9257,"source_line":1696,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::imp::OnceCell::is_initialized
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/imp_std.rs
//     evidence={"module":52,"record_offset":6944,"parent_record_offset":6920,"ancestor_record_offset":3588,"inlinee":6531,"source_line":49,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::Lazy::force
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":52,"record_offset":6832,"parent_record_offset":6804,"ancestor_record_offset":3588,"inlinee":6525,"source_line":1308,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":52,"record_offset":6920,"parent_record_offset":6888,"ancestor_record_offset":3588,"inlinee":6529,"source_line":963,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":52,"record_offset":6860,"parent_record_offset":6832,"ancestor_record_offset":3588,"inlinee":6527,"source_line":1115,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_try_init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":52,"record_offset":6888,"parent_record_offset":6860,"ancestor_record_offset":3588,"inlinee":6528,"source_line":1152,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::impl$12::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":52,"record_offset":6804,"parent_record_offset":6764,"ancestor_record_offset":3588,"inlinee":6523,"source_line":1375,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":52,"record_offset":4788,"parent_record_offset":4752,"ancestor_record_offset":3588,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":52,"record_offset":5948,"parent_record_offset":5916,"ancestor_record_offset":3588,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":52,"record_offset":6400,"parent_record_offset":6196,"ancestor_record_offset":3588,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":52,"record_offset":4928,"parent_record_offset":4900,"ancestor_record_offset":3588,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":52,"record_offset":5000,"parent_record_offset":4860,"ancestor_record_offset":3588,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":52,"record_offset":5204,"parent_record_offset":5000,"ancestor_record_offset":3588,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":52,"record_offset":4956,"parent_record_offset":4928,"ancestor_record_offset":3588,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":52,"record_offset":4860,"parent_record_offset":4824,"ancestor_record_offset":3588,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00052.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":52,"record_offset":4824,"parent_record_offset":4788,"ancestor_record_offset":3588,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00052.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tiny_glade::fps_limiter::FpsLimiter::frame
// 信息 ID: 8257b49fe12f69b1179fdb7b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x580639 -> 0x1c29e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
