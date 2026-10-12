// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/panic_reporter/mod.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: a679b57ff9d51e46d15f4dce112922c6ec44ed022fdea04627c6d867be924aff
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 35, 228
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use backtrace; // namespace candidate
//   package identities: [{"name":"backtrace","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.3.71"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use paths; // namespace candidate
//   package identities: [{"name":"paths","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::setup_panic_hook
// 记录形式: compiled_procedure; instances=1
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":35,"record_offset":1188,"rva":1593264,"va":5370302384}
// 编译实例 RVAs: 0x184fb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5376245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370302384,"raw_hex":"e8f5085200","size":5,"soft_fail":false,"target_va":5375678688,"va":5370302438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1580236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370302384,"raw_hex":"e834e3e7ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5370302423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400926c0
//     owner=unresolved_namespace; count=1
//     target_aliases=66 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-993657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370302384,"raw_hex":"e887d6f0ff","size":5,"soft_fail":false,"target_va":5369308864,"va":5370302516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t46318277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370302384,"raw_hex":"e8c5c2c202","size":5,"soft_fail":false,"target_va":5416620769,"va":5370302487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::take_hook
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t14417127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370302384,"raw_hex":"e8e7fcdb00","size":5,"soft_fail":false,"target_va":5384719536,"va":5370302404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] std::panicking::set_hook
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tjmp\t14416643","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5370302384,"raw_hex":"e903fbdb00","size":5,"soft_fail":false,"target_va":5384719120,"va":5370302472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":35,"record_offset":1316,"parent_record_offset":1188,"ancestor_record_offset":1188,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":35,"record_offset":1408,"parent_record_offset":1384,"ancestor_record_offset":1188,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":35,"record_offset":1432,"parent_record_offset":1408,"ancestor_record_offset":1188,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":35,"record_offset":1456,"parent_record_offset":1432,"ancestor_record_offset":1188,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":35,"record_offset":1384,"parent_record_offset":1356,"ancestor_record_offset":1188,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":35,"record_offset":1356,"parent_record_offset":1316,"ancestor_record_offset":1188,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::setup_panic_hook::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":35,"record_offset":1508,"rva":1593424,"va":5370302544}
// 编译实例 RVAs: 0x185050
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14009e6f0
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-944972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370302544,"raw_hex":"e8b494f1ff","size":5,"soft_fail":false,"target_va":5369358064,"va":5370303031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140517ac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3746349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370302544,"raw_hex":"e82d2a3900","size":5,"soft_fail":false,"target_va":5374048960,"va":5370302606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140523d10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3795919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370302544,"raw_hex":"e8cfeb3900","size":5,"soft_fail":false,"target_va":5374098704,"va":5370302780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370302544,"raw_hex":"e862fdffff","size":5,"soft_fail":false,"target_va":5370302256,"va":5370302921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] backtrace::capture::Backtrace::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"backtrace","version":"0.3.71","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13687165","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370302544,"raw_hex":"e87dd9d000","size":5,"soft_fail":false,"target_va":5383989840,"va":5370302670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::functions::sleep
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t14425532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370302544,"raw_hex":"e8bc1ddc00","size":5,"soft_fail":false,"target_va":5384728160,"va":5370302623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::crash_service::request_and_wait
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3725508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370302544,"raw_hex":"e8c4d83800","size":5,"soft_fail":false,"target_va":5374028480,"va":5370302967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::sentry_reporter::panic_info_to_message
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t615778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370302544,"raw_hex":"e862650900","size":5,"soft_fail":false,"target_va":5370918688,"va":5370302905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$33::call
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":35,"record_offset":2400,"parent_record_offset":1508,"ancestor_record_offset":1508,"inlinee":8038,"source_line":2253,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::panic_reporter::panic_handler
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":35,"record_offset":1652,"parent_record_offset":1508,"ancestor_record_offset":1508,"inlinee":8033,"source_line":41,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":35,"record_offset":2144,"parent_record_offset":1652,"ancestor_record_offset":1508,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":35,"record_offset":2168,"parent_record_offset":2144,"ancestor_record_offset":1508,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":35,"record_offset":2000,"parent_record_offset":1976,"ancestor_record_offset":1508,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":35,"record_offset":1808,"parent_record_offset":1652,"ancestor_record_offset":1508,"inlinee":4686,"source_line":788,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":35,"record_offset":1748,"parent_record_offset":1724,"ancestor_record_offset":1508,"inlinee":5580,"source_line":2842,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":35,"record_offset":2084,"parent_record_offset":2060,"ancestor_record_offset":1508,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":35,"record_offset":1772,"parent_record_offset":1748,"ancestor_record_offset":1508,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":35,"record_offset":2024,"parent_record_offset":2000,"ancestor_record_offset":1508,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":35,"record_offset":1832,"parent_record_offset":1808,"ancestor_record_offset":1508,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":35,"record_offset":2060,"parent_record_offset":1652,"ancestor_record_offset":1508,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] std::process::abort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\process.rs
//     evidence={"module":35,"record_offset":2340,"parent_record_offset":1652,"ancestor_record_offset":1508,"inlinee":4946,"source_line":2532,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::abort_internal
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\pal\windows\mod.rs
//     evidence={"module":35,"record_offset":2364,"parent_record_offset":2340,"ancestor_record_offset":1508,"inlinee":4948,"source_line":266,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":35,"record_offset":1888,"parent_record_offset":1864,"ancestor_record_offset":1508,"inlinee":6616,"source_line":606,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":35,"record_offset":1936,"parent_record_offset":1912,"ancestor_record_offset":1508,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":35,"record_offset":1912,"parent_record_offset":1888,"ancestor_record_offset":1508,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::crash_service::crash_in_progress
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/crash_service.rs
//     evidence={"module":35,"record_offset":1724,"parent_record_offset":1652,"ancestor_record_offset":1508,"inlinee":8035,"source_line":182,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::crash_service::note_crash_started
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/crash_service.rs
//     evidence={"module":35,"record_offset":1976,"parent_record_offset":1652,"ancestor_record_offset":1508,"inlinee":8039,"source_line":169,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::crash_service::park_secondary_panic
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/crash_service.rs
//     evidence={"module":35,"record_offset":1864,"parent_record_offset":1652,"ancestor_record_offset":1508,"inlinee":8036,"source_line":190,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_handler
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1F61
// 首条位置: {"module":35,"inlinee":8033}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":35,"record_offset":2144,"parent_record_offset":1652,"ancestor_record_offset":1652,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicBool::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":35,"record_offset":1808,"parent_record_offset":1652,"ancestor_record_offset":1652,"inlinee":4686,"source_line":788,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":35,"record_offset":2060,"parent_record_offset":1652,"ancestor_record_offset":1652,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_direct] std::process::abort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\process.rs
//     evidence={"module":35,"record_offset":2340,"parent_record_offset":1652,"ancestor_record_offset":1652,"inlinee":4946,"source_line":2532,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::panic_reporter::crash_service::crash_in_progress
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/crash_service.rs
//     evidence={"module":35,"record_offset":1724,"parent_record_offset":1652,"ancestor_record_offset":1652,"inlinee":8035,"source_line":182,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::panic_reporter::crash_service::note_crash_started
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/crash_service.rs
//     evidence={"module":35,"record_offset":1976,"parent_record_offset":1652,"ancestor_record_offset":1652,"inlinee":8039,"source_line":169,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::panic_reporter::crash_service::park_secondary_panic
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/crash_service.rs
//     evidence={"module":35,"record_offset":1864,"parent_record_offset":1652,"ancestor_record_offset":1652,"inlinee":8036,"source_line":190,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":35,"record_offset":2168,"parent_record_offset":2144,"ancestor_record_offset":1652,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":35,"record_offset":2000,"parent_record_offset":1976,"ancestor_record_offset":1652,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":35,"record_offset":1748,"parent_record_offset":1724,"ancestor_record_offset":1652,"inlinee":5580,"source_line":2842,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":35,"record_offset":2084,"parent_record_offset":2060,"ancestor_record_offset":1652,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":35,"record_offset":1772,"parent_record_offset":1748,"ancestor_record_offset":1652,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":35,"record_offset":2024,"parent_record_offset":2000,"ancestor_record_offset":1652,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":35,"record_offset":1832,"parent_record_offset":1808,"ancestor_record_offset":1652,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::abort_internal
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\pal\windows\mod.rs
//     evidence={"module":35,"record_offset":2364,"parent_record_offset":2340,"ancestor_record_offset":1652,"inlinee":4948,"source_line":266,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":35,"record_offset":1888,"parent_record_offset":1864,"ancestor_record_offset":1652,"inlinee":6616,"source_line":606,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":35,"record_offset":1936,"parent_record_offset":1912,"ancestor_record_offset":1652,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":35,"record_offset":1912,"parent_record_offset":1888,"ancestor_record_offset":1652,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash
// 记录形式: compiled_procedure; instances=1
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":35,"record_offset":2432,"rva":1593968,"va":5370303088}
// 编译实例 RVAs: 0x185270
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t14492142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370303088,"raw_hex":"e8ee21dd00","size":5,"soft_fail":false,"target_va":5384795984,"va":5370303837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::wtf8::Wtf8>::to_owned
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14490095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370303088,"raw_hex":"e8ef19dd00","size":5,"soft_fail":false,"target_va":5384793264,"va":5370303164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14355657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370303088,"raw_hex":"e8c90cdb00","size":5,"soft_fail":false,"target_va":5384659264,"va":5370303602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::DirBuilder>::_create
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14386048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370303088,"raw_hex":"e88083db00","size":5,"soft_fail":false,"target_va":5384689616,"va":5370303563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14352783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370303088,"raw_hex":"e88f01db00","size":5,"soft_fail":false,"target_va":5384656240,"va":5370303452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5374937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370303088,"raw_hex":"e8d9035200","size":5,"soft_fail":false,"target_va":5375678688,"va":5370303746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=12
//     evidence={"assembly":"\tcallq\t5375404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370303088,"raw_hex":"e8ac055200","size":5,"soft_fail":false,"target_va":5375678704,"va":5370303295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1581544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370303088,"raw_hex":"e818dee7ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5370303731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007eec0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1078610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370303088,"raw_hex":"e8ae8aefff","size":5,"soft_fail":false,"target_va":5369228992,"va":5370307597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-969396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370303088,"raw_hex":"e84c35f1ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5370308239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b2470
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-867876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370303088,"raw_hex":"e8dcc1f2ff","size":5,"soft_fail":false,"target_va":5369439344,"va":5370307215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140103f10
//     owner=unresolved_namespace; count=9
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-533479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370303088,"raw_hex":"e819dcf7ff","size":5,"soft_fail":false,"target_va":5369773840,"va":5370307314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14018be80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t22422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370303088,"raw_hex":"e896570000","size":5,"soft_fail":false,"target_va":5370330752,"va":5370308325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e2850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4573690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370303088,"raw_hex":"e8fac94500","size":5,"soft_fail":false,"target_va":5374879824,"va":5370306129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d678c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46015460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370303088,"raw_hex":"e8e423be02","size":5,"soft_fail":false,"target_va":5416319168,"va":5370303703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46753067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370303088,"raw_hex":"e82b65c902","size":5,"soft_fail":false,"target_va":5417057056,"va":5370303984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t14491500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370303088,"raw_hex":"e86c1fdd00","size":5,"soft_fail":false,"target_va":5384794864,"va":5370303359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t46313861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370303088,"raw_hex":"e885b1c202","size":5,"soft_fail":false,"target_va":5416620794,"va":5370306928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=16
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t14320841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370303088,"raw_hex":"e8c984da00","size":5,"soft_fail":false,"target_va":5384624080,"va":5370303234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<std::path::PathBuf,enum2$<core::option::Option<country_core::resources::session::ActiveSession> > >,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > > >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t23244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370303088,"raw_hex":"e8cc5a0000","size":5,"soft_fail":false,"target_va":5370330640,"va":5370307391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::backup_folder_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9961280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370303088,"raw_hex":"e840ff9700","size":5,"soft_fail":false,"target_va":5380267776,"va":5370306491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::save_folder_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9960832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370303088,"raw_hex":"e880fd9700","size":5,"soft_fail":false,"target_va":5380267312,"va":5370306475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::find_standard_app_root_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t12835695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370303088,"raw_hex":"e86fdbc300","size":5,"soft_fail":false,"target_va":5383138832,"va":5370303132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::stdio::_eprint
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t14417270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370303088,"raw_hex":"e876fddb00","size":5,"soft_fail":false,"target_va":5384721472,"va":5370304197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::panic_dump::dump_gpu_crash_dump_file
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t79030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370303088,"raw_hex":"e8b6340100","size":5,"soft_fail":false,"target_va":5370384752,"va":5370305717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::panic_dump::dump_inputs
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t77550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370303088,"raw_hex":"e8ee2e0100","size":5,"soft_fail":false,"target_va":5370381904,"va":5370304349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::panic_dump::dump_log_file
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t78184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370303088,"raw_hex":"e868310100","size":5,"soft_fail":false,"target_va":5370382976,"va":5370304787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::panic_dump::dump_profile_file
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t78076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370303088,"raw_hex":"e8fc300100","size":5,"soft_fail":false,"target_va":5370383488,"va":5370305407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::panic_dump::dump_save
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t81211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370303088,"raw_hex":"e83b3d0100","size":5,"soft_fail":false,"target_va":5370385264,"va":5370304048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::panic_dump::dump_settings_file
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t79010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370303088,"raw_hex":"e8a2340100","size":5,"soft_fail":false,"target_va":5370384112,"va":5370305097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::panic_screenshot::dump_screenshot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2565480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370303088,"raw_hex":"e868252700","size":5,"soft_fail":false,"target_va":5372869552,"va":5370304067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::file::date_time_string_for_filename
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t12996748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370303088,"raw_hex":"e88c50c600","size":5,"soft_fail":false,"target_va":5383300064,"va":5370303311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":35,"record_offset":3176,"parent_record_offset":2432,"ancestor_record_offset":2432,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":35,"record_offset":4864,"parent_record_offset":2432,"ancestor_record_offset":2432,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":35,"record_offset":4656,"parent_record_offset":2432,"ancestor_record_offset":2432,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":35,"record_offset":3544,"parent_record_offset":2432,"ancestor_record_offset":2432,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":35,"record_offset":7080,"parent_record_offset":2432,"ancestor_record_offset":2432,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":35,"record_offset":3260,"parent_record_offset":2432,"ancestor_record_offset":2432,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":35,"record_offset":7052,"parent_record_offset":2432,"ancestor_record_offset":2432,"inlinee":8050,"source_line":768,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":35,"record_offset":7644,"parent_record_offset":2432,"ancestor_record_offset":2432,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":35,"record_offset":5348,"parent_record_offset":2432,"ancestor_record_offset":2432,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":35,"record_offset":2712,"parent_record_offset":2432,"ancestor_record_offset":2432,"inlinee":8040,"source_line":1590,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":35,"record_offset":4544,"parent_record_offset":2432,"ancestor_record_offset":2432,"inlinee":7539,"source_line":3123,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":35,"record_offset":3572,"parent_record_offset":2432,"ancestor_record_offset":2432,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$37::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":35,"record_offset":2628,"parent_record_offset":2432,"ancestor_record_offset":2432,"inlinee":7154,"source_line":1959,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::panic_reporter::crash_service::is_shutting_down
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/crash_service.rs
//     evidence={"module":35,"record_offset":4768,"parent_record_offset":2432,"ancestor_record_offset":2432,"inlinee":8041,"source_line":203,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":35,"record_offset":5168,"parent_record_offset":5144,"ancestor_record_offset":2432,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":35,"record_offset":5192,"parent_record_offset":5168,"ancestor_record_offset":2432,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":35,"record_offset":3052,"parent_record_offset":3028,"ancestor_record_offset":2432,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":35,"record_offset":3076,"parent_record_offset":3052,"ancestor_record_offset":2432,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":35,"record_offset":5216,"parent_record_offset":5192,"ancestor_record_offset":2432,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":35,"record_offset":3100,"parent_record_offset":3076,"ancestor_record_offset":2432,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":35,"record_offset":5144,"parent_record_offset":5116,"ancestor_record_offset":2432,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":35,"record_offset":3028,"parent_record_offset":2972,"ancestor_record_offset":2432,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":35,"record_offset":5452,"parent_record_offset":5416,"ancestor_record_offset":2432,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":35,"record_offset":3224,"parent_record_offset":3200,"ancestor_record_offset":2432,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":35,"record_offset":5056,"parent_record_offset":5028,"ancestor_record_offset":2432,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":35,"record_offset":3000,"parent_record_offset":2972,"ancestor_record_offset":2432,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":35,"record_offset":2972,"parent_record_offset":2948,"ancestor_record_offset":2432,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":35,"record_offset":5116,"parent_record_offset":5084,"ancestor_record_offset":2432,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":35,"record_offset":5084,"parent_record_offset":5056,"ancestor_record_offset":2432,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":35,"record_offset":2948,"parent_record_offset":2924,"ancestor_record_offset":2432,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":35,"record_offset":4936,"parent_record_offset":4892,"ancestor_record_offset":2432,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":35,"record_offset":4964,"parent_record_offset":4936,"ancestor_record_offset":2432,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":35,"record_offset":4992,"parent_record_offset":4964,"ancestor_record_offset":2432,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":35,"record_offset":4892,"parent_record_offset":4864,"ancestor_record_offset":2432,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":35,"record_offset":7540,"parent_record_offset":7516,"ancestor_record_offset":2432,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":35,"record_offset":7236,"parent_record_offset":7188,"ancestor_record_offset":2432,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":35,"record_offset":5028,"parent_record_offset":4992,"ancestor_record_offset":2432,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":35,"record_offset":7116,"parent_record_offset":7080,"ancestor_record_offset":2432,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":35,"record_offset":7516,"parent_record_offset":7188,"ancestor_record_offset":2432,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":35,"record_offset":7152,"parent_record_offset":7116,"ancestor_record_offset":2432,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":35,"record_offset":7188,"parent_record_offset":7152,"ancestor_record_offset":2432,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":35,"record_offset":4728,"parent_record_offset":4688,"ancestor_record_offset":2432,"inlinee":6277,"source_line":245,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":35,"record_offset":4688,"parent_record_offset":4656,"ancestor_record_offset":2432,"inlinee":6326,"source_line":23,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":35,"record_offset":4596,"parent_record_offset":4544,"ancestor_record_offset":2432,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":35,"record_offset":5536,"parent_record_offset":5416,"ancestor_record_offset":2432,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":35,"record_offset":7564,"parent_record_offset":7540,"ancestor_record_offset":2432,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":35,"record_offset":7588,"parent_record_offset":7564,"ancestor_record_offset":2432,"inlinee":8052,"source_line":841,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":35,"record_offset":8460,"parent_record_offset":8416,"ancestor_record_offset":2432,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":35,"record_offset":3200,"parent_record_offset":3176,"ancestor_record_offset":2432,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":35,"record_offset":5272,"parent_record_offset":4992,"ancestor_record_offset":2432,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":35,"record_offset":5296,"parent_record_offset":5272,"ancestor_record_offset":2432,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=70
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":35,"record_offset":2748,"parent_record_offset":2712,"ancestor_record_offset":2432,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":35,"record_offset":4796,"parent_record_offset":4768,"ancestor_record_offset":2432,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":35,"record_offset":4824,"parent_record_offset":4796,"ancestor_record_offset":2432,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":35,"record_offset":2652,"parent_record_offset":2628,"ancestor_record_offset":2432,"inlinee":5109,"source_line":986,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":35,"record_offset":4624,"parent_record_offset":4544,"ancestor_record_offset":2432,"inlinee":7542,"source_line":3441,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":35,"record_offset":4568,"parent_record_offset":4544,"ancestor_record_offset":2432,"inlinee":7541,"source_line":3417,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":35,"record_offset":2676,"parent_record_offset":2652,"ancestor_record_offset":2432,"inlinee":5111,"source_line":265,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::recover_game_state_after_crash::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":35,"record_offset":5388,"parent_record_offset":5348,"ancestor_record_offset":2432,"inlinee":8043,"source_line":96,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::recover_game_state_after_crash::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":35,"record_offset":5672,"parent_record_offset":5632,"ancestor_record_offset":2432,"inlinee":8045,"source_line":97,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::recover_game_state_after_crash::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":35,"record_offset":5956,"parent_record_offset":5916,"ancestor_record_offset":2432,"inlinee":8046,"source_line":101,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::recover_game_state_after_crash::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":35,"record_offset":6240,"parent_record_offset":6200,"ancestor_record_offset":2432,"inlinee":8047,"source_line":104,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::recover_game_state_after_crash::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":35,"record_offset":6524,"parent_record_offset":6484,"ancestor_record_offset":2432,"inlinee":8048,"source_line":107,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::recover_game_state_after_crash::closure$5
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":35,"record_offset":6808,"parent_record_offset":6768,"ancestor_record_offset":2432,"inlinee":8049,"source_line":110,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::recover_game_state_after_crash::closure$8
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":35,"record_offset":8160,"parent_record_offset":8112,"ancestor_record_offset":2432,"inlinee":8053,"source_line":128,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::recover_game_state_after_crash::closure$9
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":35,"record_offset":8416,"parent_record_offset":8364,"ancestor_record_offset":2432,"inlinee":8054,"source_line":130,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::recover_game_state_after_crash::closure$9::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":35,"record_offset":8500,"parent_record_offset":8460,"ancestor_record_offset":2432,"inlinee":8056,"source_line":132,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::recover_game_state_after_crash::logging_map_error
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":35,"record_offset":5416,"parent_record_offset":5388,"ancestor_record_offset":2432,"inlinee":8044,"source_line":74,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash::logging_map_error
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 74
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1F6C
// 首条位置: {"module":35,"inlinee":8044}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":35,"record_offset":5452,"parent_record_offset":5416,"ancestor_record_offset":5416,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":35,"record_offset":5536,"parent_record_offset":5416,"ancestor_record_offset":5416,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":35,"record_offset":5564,"parent_record_offset":5416,"ancestor_record_offset":5416,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":35,"record_offset":5500,"parent_record_offset":5476,"ancestor_record_offset":5416,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":35,"record_offset":5476,"parent_record_offset":5452,"ancestor_record_offset":5416,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 96
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1F6B
// 首条位置: {"module":35,"inlinee":8043}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] tiny_glade::panic_reporter::recover_game_state_after_crash::logging_map_error
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":35,"record_offset":5416,"parent_record_offset":5388,"ancestor_record_offset":5388,"inlinee":8044,"source_line":74,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":35,"record_offset":5452,"parent_record_offset":5416,"ancestor_record_offset":5388,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":35,"record_offset":5500,"parent_record_offset":5476,"ancestor_record_offset":5388,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":35,"record_offset":5536,"parent_record_offset":5416,"ancestor_record_offset":5388,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":35,"record_offset":5476,"parent_record_offset":5452,"ancestor_record_offset":5388,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":35,"record_offset":5564,"parent_record_offset":5416,"ancestor_record_offset":5388,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1F6D
// 首条位置: {"module":35,"inlinee":8045}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] tiny_glade::panic_reporter::recover_game_state_after_crash::logging_map_error
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":35,"record_offset":5700,"parent_record_offset":5672,"ancestor_record_offset":5672,"inlinee":8044,"source_line":74,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":35,"record_offset":5736,"parent_record_offset":5700,"ancestor_record_offset":5672,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":35,"record_offset":5784,"parent_record_offset":5760,"ancestor_record_offset":5672,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":35,"record_offset":5820,"parent_record_offset":5700,"ancestor_record_offset":5672,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":35,"record_offset":5760,"parent_record_offset":5736,"ancestor_record_offset":5672,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":35,"record_offset":5848,"parent_record_offset":5700,"ancestor_record_offset":5672,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1F6E
// 首条位置: {"module":35,"inlinee":8046}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] tiny_glade::panic_reporter::recover_game_state_after_crash::logging_map_error
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":35,"record_offset":5984,"parent_record_offset":5956,"ancestor_record_offset":5956,"inlinee":8044,"source_line":74,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":35,"record_offset":6020,"parent_record_offset":5984,"ancestor_record_offset":5956,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":35,"record_offset":6068,"parent_record_offset":6044,"ancestor_record_offset":5956,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":35,"record_offset":6104,"parent_record_offset":5984,"ancestor_record_offset":5956,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":35,"record_offset":6044,"parent_record_offset":6020,"ancestor_record_offset":5956,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":35,"record_offset":6132,"parent_record_offset":5984,"ancestor_record_offset":5956,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1F6F
// 首条位置: {"module":35,"inlinee":8047}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] tiny_glade::panic_reporter::recover_game_state_after_crash::logging_map_error
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":35,"record_offset":6268,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":8044,"source_line":74,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":35,"record_offset":6304,"parent_record_offset":6268,"ancestor_record_offset":6240,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":35,"record_offset":6352,"parent_record_offset":6328,"ancestor_record_offset":6240,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":35,"record_offset":6388,"parent_record_offset":6268,"ancestor_record_offset":6240,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":35,"record_offset":6328,"parent_record_offset":6304,"ancestor_record_offset":6240,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":35,"record_offset":6416,"parent_record_offset":6268,"ancestor_record_offset":6240,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 107
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1F70
// 首条位置: {"module":35,"inlinee":8048}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] tiny_glade::panic_reporter::recover_game_state_after_crash::logging_map_error
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":35,"record_offset":6552,"parent_record_offset":6524,"ancestor_record_offset":6524,"inlinee":8044,"source_line":74,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":35,"record_offset":6588,"parent_record_offset":6552,"ancestor_record_offset":6524,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":35,"record_offset":6636,"parent_record_offset":6612,"ancestor_record_offset":6524,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":35,"record_offset":6672,"parent_record_offset":6552,"ancestor_record_offset":6524,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":35,"record_offset":6612,"parent_record_offset":6588,"ancestor_record_offset":6524,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":35,"record_offset":6700,"parent_record_offset":6552,"ancestor_record_offset":6524,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 110
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1F71
// 首条位置: {"module":35,"inlinee":8049}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] tiny_glade::panic_reporter::recover_game_state_after_crash::logging_map_error
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":35,"record_offset":6836,"parent_record_offset":6808,"ancestor_record_offset":6808,"inlinee":8044,"source_line":74,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":35,"record_offset":6872,"parent_record_offset":6836,"ancestor_record_offset":6808,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":35,"record_offset":6920,"parent_record_offset":6896,"ancestor_record_offset":6808,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":35,"record_offset":6956,"parent_record_offset":6836,"ancestor_record_offset":6808,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":35,"record_offset":6896,"parent_record_offset":6872,"ancestor_record_offset":6808,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":35,"record_offset":6984,"parent_record_offset":6836,"ancestor_record_offset":6808,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 119
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x42BC
// 首条位置: {"module":228,"inlinee":17084}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":228,"record_offset":53024,"parent_record_offset":52440,"ancestor_record_offset":52440,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":228,"record_offset":52492,"parent_record_offset":52440,"ancestor_record_offset":52440,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":228,"record_offset":53108,"parent_record_offset":52440,"ancestor_record_offset":52440,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":228,"record_offset":53136,"parent_record_offset":52440,"ancestor_record_offset":52440,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":52836,"parent_record_offset":52812,"ancestor_record_offset":52440,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":52860,"parent_record_offset":52836,"ancestor_record_offset":52440,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":53336,"parent_record_offset":53312,"ancestor_record_offset":52440,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":53360,"parent_record_offset":53336,"ancestor_record_offset":52440,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":52884,"parent_record_offset":52860,"ancestor_record_offset":52440,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":53384,"parent_record_offset":53360,"ancestor_record_offset":52440,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":52812,"parent_record_offset":52784,"ancestor_record_offset":52440,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":53312,"parent_record_offset":53256,"ancestor_record_offset":52440,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":228,"record_offset":53072,"parent_record_offset":53048,"ancestor_record_offset":52440,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":228,"record_offset":52724,"parent_record_offset":52696,"ancestor_record_offset":52440,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":228,"record_offset":53284,"parent_record_offset":53256,"ancestor_record_offset":52440,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":228,"record_offset":53256,"parent_record_offset":53232,"ancestor_record_offset":52440,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":228,"record_offset":52784,"parent_record_offset":52752,"ancestor_record_offset":52440,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":228,"record_offset":52752,"parent_record_offset":52724,"ancestor_record_offset":52440,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":228,"record_offset":53232,"parent_record_offset":53208,"ancestor_record_offset":52440,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":228,"record_offset":52604,"parent_record_offset":52576,"ancestor_record_offset":52440,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":228,"record_offset":52632,"parent_record_offset":52604,"ancestor_record_offset":52440,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":228,"record_offset":52660,"parent_record_offset":52632,"ancestor_record_offset":52440,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":228,"record_offset":52576,"parent_record_offset":52548,"ancestor_record_offset":52440,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":228,"record_offset":52548,"parent_record_offset":52520,"ancestor_record_offset":52440,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":228,"record_offset":52520,"parent_record_offset":52492,"ancestor_record_offset":52440,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":228,"record_offset":52696,"parent_record_offset":52660,"ancestor_record_offset":52440,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":228,"record_offset":53048,"parent_record_offset":53024,"ancestor_record_offset":52440,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":228,"record_offset":52940,"parent_record_offset":52660,"ancestor_record_offset":52440,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":228,"record_offset":52964,"parent_record_offset":52940,"ancestor_record_offset":52440,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":228,"record_offset":53160,"parent_record_offset":53136,"ancestor_record_offset":52440,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$8
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 128
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1F75
// 首条位置: {"module":35,"inlinee":8053}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":35,"record_offset":8188,"parent_record_offset":8160,"ancestor_record_offset":8160,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":35,"record_offset":8272,"parent_record_offset":8160,"ancestor_record_offset":8160,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":35,"record_offset":8300,"parent_record_offset":8160,"ancestor_record_offset":8160,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":35,"record_offset":8236,"parent_record_offset":8212,"ancestor_record_offset":8160,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":35,"record_offset":8212,"parent_record_offset":8188,"ancestor_record_offset":8160,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$9
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 130
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1F76
// 首条位置: {"module":35,"inlinee":8054}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":35,"record_offset":8460,"parent_record_offset":8416,"ancestor_record_offset":8416,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":35,"record_offset":8952,"parent_record_offset":8416,"ancestor_record_offset":8416,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":35,"record_offset":8796,"parent_record_offset":8772,"ancestor_record_offset":8416,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":35,"record_offset":8820,"parent_record_offset":8796,"ancestor_record_offset":8416,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":35,"record_offset":8844,"parent_record_offset":8820,"ancestor_record_offset":8416,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":35,"record_offset":8772,"parent_record_offset":8716,"ancestor_record_offset":8416,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":35,"record_offset":8744,"parent_record_offset":8716,"ancestor_record_offset":8416,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":35,"record_offset":8716,"parent_record_offset":8692,"ancestor_record_offset":8416,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":35,"record_offset":8692,"parent_record_offset":8668,"ancestor_record_offset":8416,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":35,"record_offset":8548,"parent_record_offset":8500,"ancestor_record_offset":8416,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::recover_game_state_after_crash::closure$9::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":35,"record_offset":8500,"parent_record_offset":8460,"ancestor_record_offset":8416,"inlinee":8056,"source_line":132,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$9::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1F78
// 首条位置: {"module":35,"inlinee":8056}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":35,"record_offset":8548,"parent_record_offset":8500,"ancestor_record_offset":8500,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":35,"record_offset":8796,"parent_record_offset":8772,"ancestor_record_offset":8500,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":35,"record_offset":8820,"parent_record_offset":8796,"ancestor_record_offset":8500,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":35,"record_offset":8844,"parent_record_offset":8820,"ancestor_record_offset":8500,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":35,"record_offset":8772,"parent_record_offset":8716,"ancestor_record_offset":8500,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":35,"record_offset":8744,"parent_record_offset":8716,"ancestor_record_offset":8500,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":35,"record_offset":8716,"parent_record_offset":8692,"ancestor_record_offset":8500,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":35,"record_offset":8692,"parent_record_offset":8668,"ancestor_record_offset":8500,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":35,"record_offset":8572,"parent_record_offset":8548,"ancestor_record_offset":8500,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00035.jsonl.gz"}
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
// 完整结构及原始类型记录: mod.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mod.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mod.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] alloc::string::String (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >,ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >)
// 候选参数类型: ["ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >", "ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >"]
// [文件级接口候选，具体函数归属 unknown] sentry_types::protocol::v7::Breadcrumb (*)(ref$<sentry_types::protocol::v7::Breadcrumb>)
// 候选参数类型: ["ref$<sentry_types::protocol::v7::Breadcrumb>"]

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_handler
// 信息 ID: d7892cd8e6e30623479fbd4f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::setup_panic_hook::closure$0; module=35, inline_record=1652; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash
// 信息 ID: afbf9c45ec65168c8adf9100
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::panic_reporter::crash_service::start::closure_env$2,tuple$<> > @ 0x5e8e74 -> 0x185270 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$0
// 信息 ID: 43dd5fec9cb8b786b626a981
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::recover_game_state_after_crash", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=35, inline_record=5388; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$1
// 信息 ID: 2cb603571b51f30c961a49d2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::panic_reporter::recover_game_state_after_crash", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=35, inline_record=5672; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$2
// 信息 ID: d3efd4cea62b2fc79c31a862
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::panic_reporter::recover_game_state_after_crash", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=35, inline_record=5956; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$3
// 信息 ID: e75eb9ad2cc97000c9567705
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tiny_glade::panic_reporter::recover_game_state_after_crash", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=35, inline_record=6240; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$4
// 信息 ID: f6f38a7c2f4cfa473ff08c3f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::panic_reporter::recover_game_state_after_crash", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=35, inline_record=6524; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$5
// 信息 ID: 54bbf53c3e3b933fc9a0c6ba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "tiny_glade::panic_reporter::recover_game_state_after_crash", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=35, inline_record=6808; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$6
// 信息 ID: b763c3e7bf463debd5f79d67
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "tiny_glade::panic_reporter::recover_game_state_after_crash", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::recover_game_state_after_crash::closure_env$6 <- core::iter::adapters::map::impl$2::fold<sentry_types::protocol::v7::Breadcrumb,core::option::IntoIter<ref$<anyhow::Error> >,tiny_glade::panic_reporter::recover_game_state_after_crash::closure_env$6,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<sentry_types::protocol::v7::Breadcrumb,alloc::vec::impl$21::extend_trusted::closure_env$0<sentry_types::protocol::v7::Breadcrumb,alloc::alloc::Global,core::iter::adapters::map::Map<core::option::IntoIter<ref$<anyhow::Error> >,tiny_glade::panic_reporter::recover_game_state_after_crash::closure_env$6> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::recover_game_state_after_crash::closure_env$6 <- core::iter::adapters::map::impl$2::fold<sentry_types::protocol::v7::Breadcrumb,core::option::IntoIter<ref$<anyhow::Error> >,tiny_glade::panic_reporter::recover_game_state_after_crash::closure_env$6,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<sentry_types::protocol::v7::Breadcrumb,alloc::vec::impl$21::extend_trusted::closure_env$0<sentry_types::protocol::v7::Breadcrumb,alloc::alloc::Global,core::iter::adapters::map::Map<core::option::IntoIter<ref$<anyhow::Error> >,tiny_glade::panic_reporter::recover_game_state_after_crash::closure_env$6> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<sentry_types::protocol::v7::Breadcrumb,core::option::IntoIter<ref$<anyhow::Error> >,tiny_glade::panic_reporter::recover_game_state_after_crash::closure_env$6,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<sentry_types::protocol::v7::Breadcrumb,alloc::vec::impl$21::extend_trusted::closure_env$0<sentry_types::protocol::v7::Breadcrumb,alloc::alloc::Global,core::iter::adapters::map::Map<core::option::IntoIter<ref$<anyhow::Error> >,tiny_glade::panic_reporter::recover_game_state_after_crash::closure_env$6> > > >; module=228, inline_record=52440; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$8
// 信息 ID: a10167233797765fe0d40c09
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 8, "lexical_owner": "tiny_glade::panic_reporter::recover_game_state_after_crash", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=35, inline_record=8160; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$9
// 信息 ID: 853f9bbc8b55f8cd702c72aa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 9, "lexical_owner": "tiny_glade::panic_reporter::recover_game_state_after_crash", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=35, inline_record=8416; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$9::closure$0
// 信息 ID: 54e8b001acad9fce05e9d986
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 9, "lexical_owner": "tiny_glade::panic_reporter::recover_game_state_after_crash", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=35, inline_record=8500; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash::logging_map_error
// 信息 ID: dedeada93c3b500f972eea67
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=35, inline_record=5416; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=35, inline_record=5700; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=35, inline_record=5984; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=35, inline_record=6268; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=35, inline_record=6552; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=35, inline_record=6836; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::setup_panic_hook
// 信息 ID: c1eb9b66b61d44b550c1d0a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::main @ 0x643689 -> 0x184fb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::setup_panic_hook::closure$0
// 信息 ID: ba2e2b893d61e020cef8a406
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::setup_panic_hook", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::setup_panic_hook::closure_env$0 <- core::ops::function::FnOnce::call_once<tiny_glade::panic_reporter::setup_panic_hook::closure_env$0,tuple$<ref$<std::panic::PanicHookInfo> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::setup_panic_hook::closure_env$0 <- core::ptr::drop_in_place<tiny_glade::panic_reporter::setup_panic_hook::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::panic_reporter::setup_panic_hook::closure_env$0,tuple$<ref$<std::panic::PanicHookInfo> > > @ 0x189f0a -> 0x185050 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
