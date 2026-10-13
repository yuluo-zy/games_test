// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tiny-glade/src/panic_reporter/mod.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 57532ecf5adbd85ec27f5ce17d5ef96792a2baef700b32fe34ba21fdfe629c15
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 32, 184
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use backtrace; // namespace candidate
//   package identities: [{"name":"backtrace","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.3.71"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
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
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":32,"record_offset":12404,"rva":898480,"va":5369607600}
// 编译实例 RVAs: 0xdb5b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400db370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369607600,"raw_hex":"e8d4fcffff","size":5,"soft_fail":false,"target_va":5369607024,"va":5369607831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f5310
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5348507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369607600,"raw_hex":"e89b9c5100","size":5,"soft_fail":false,"target_va":5374956304,"va":5369607792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5473803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369607600,"raw_hex":"e80b865300","size":5,"soft_fail":false,"target_va":5375081488,"va":5369607680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41933245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369607600,"raw_hex":"e8bdd97f02","size":5,"soft_fail":false,"target_va":5411540992,"va":5369607742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::set_hook
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13149268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369607600,"raw_hex":"e854a4c800","size":5,"soft_fail":false,"target_va":5382756992,"va":5369607719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::take_hook
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13150021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369607600,"raw_hex":"e845a7c800","size":5,"soft_fail":false,"target_va":5382757664,"va":5369607638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":32,"record_offset":12536,"parent_record_offset":12404,"ancestor_record_offset":12404,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":32,"record_offset":12716,"parent_record_offset":12404,"ancestor_record_offset":12404,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":32,"record_offset":12620,"parent_record_offset":12596,"ancestor_record_offset":12404,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":32,"record_offset":12644,"parent_record_offset":12620,"ancestor_record_offset":12404,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":32,"record_offset":12568,"parent_record_offset":12536,"ancestor_record_offset":12404,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":32,"record_offset":12596,"parent_record_offset":12568,"ancestor_record_offset":12404,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":32,"record_offset":12668,"parent_record_offset":12644,"ancestor_record_offset":12404,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::setup_panic_hook::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":32,"record_offset":12748,"rva":898736,"va":5369607856}
// 编译实例 RVAs: 0xdb6b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140043000
//     owner=unresolved_namespace; count=1
//     target_aliases=30 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-624739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369607856,"raw_hex":"e89d77f6ff","size":5,"soft_fail":false,"target_va":5368983552,"va":5369608286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b1430
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4020706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369607856,"raw_hex":"e8e2593d00","size":5,"soft_fail":false,"target_va":5373629488,"va":5369608777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ebcf0
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5308773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369607856,"raw_hex":"e865015100","size":5,"soft_fail":false,"target_va":5374917872,"va":5369609094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406009e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5394088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369607856,"raw_hex":"e8a84e5200","size":5,"soft_fail":false,"target_va":5375003104,"va":5369609011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=2
//     target_aliases=69 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5907313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369607856,"raw_hex":"e871235a00","size":5,"soft_fail":false,"target_va":5375516032,"va":5369608714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42238663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369607856,"raw_hex":"e8c7828402","size":5,"soft_fail":false,"target_va":5411847168,"va":5369608500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369607856,"raw_hex":"e84cfcffff","size":5,"soft_fail":false,"target_va":5369607472,"va":5369608415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] backtrace::capture::Backtrace::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"backtrace","version":"0.3.71","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12224414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369607856,"raw_hex":"e89e87ba00","size":5,"soft_fail":false,"target_va":5381832720,"va":5369608301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<std::sync::poison::mutex::MutexGuard<bool>,std::sync::poison::PoisonError<std::sync::poison::mutex::MutexGuard<bool> > > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369607856,"raw_hex":"e8faf5ffff","size":5,"soft_fail":false,"target_va":5369606624,"va":5369609185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::Result::expect<tuple$<>,enum2$<tiny_glade::crash_pickup::Error> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-1251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369607856,"raw_hex":"e81dfbffff","size":5,"soft_fail":false,"target_va":5369607344,"va":5369608590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::stdio::_eprint
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13111909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369607856,"raw_hex":"e86512c800","size":5,"soft_fail":false,"target_va":5382720160,"va":5369608246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t41930501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369607856,"raw_hex":"e805cf7f02","size":5,"soft_fail":false,"target_va":5411538608,"va":5369608102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41930683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369607856,"raw_hex":"e8bbcf7f02","size":5,"soft_fail":false,"target_va":5411538768,"va":5369608080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41930819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369607856,"raw_hex":"e843d07f02","size":5,"soft_fail":false,"target_va":5411538960,"va":5369608136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::crash_pickup::ClientHandle::send_message
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t520928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369607856,"raw_hex":"e8e0f20700","size":5,"soft_fail":false,"target_va":5370129504,"va":5369608571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::recover_game_state_after_crash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369607856,"raw_hex":"e88f030000","size":5,"soft_fail":false,"target_va":5369609264,"va":5369608348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::sentry_reporter::panic_info_to_message
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4037614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369607856,"raw_hex":"e8ee9b3d00","size":5,"soft_fail":false,"target_va":5373646016,"va":5369608397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$30::call
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":32,"record_offset":14408,"parent_record_offset":12748,"ancestor_record_offset":12748,"inlinee":8202,"source_line":1989,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":32,"record_offset":12896,"parent_record_offset":12748,"ancestor_record_offset":12748,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::panic_reporter::panic_handler
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":32,"record_offset":12928,"parent_record_offset":12748,"ancestor_record_offset":12748,"inlinee":8200,"source_line":40,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":32,"record_offset":14144,"parent_record_offset":12928,"ancestor_record_offset":12748,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":32,"record_offset":13976,"parent_record_offset":12928,"ancestor_record_offset":12748,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":32,"record_offset":14348,"parent_record_offset":12928,"ancestor_record_offset":12748,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":32,"record_offset":13532,"parent_record_offset":12928,"ancestor_record_offset":12748,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":32,"record_offset":13460,"parent_record_offset":13436,"ancestor_record_offset":12748,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":32,"record_offset":13812,"parent_record_offset":13624,"ancestor_record_offset":12748,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":32,"record_offset":13148,"parent_record_offset":13116,"ancestor_record_offset":12748,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":32,"record_offset":13904,"parent_record_offset":13872,"ancestor_record_offset":12748,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":32,"record_offset":13368,"parent_record_offset":13336,"ancestor_record_offset":12748,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":32,"record_offset":13172,"parent_record_offset":13148,"ancestor_record_offset":12748,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":32,"record_offset":13392,"parent_record_offset":13368,"ancestor_record_offset":12748,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":32,"record_offset":13836,"parent_record_offset":13812,"ancestor_record_offset":12748,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":32,"record_offset":13928,"parent_record_offset":13904,"ancestor_record_offset":12748,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":32,"record_offset":14060,"parent_record_offset":12928,"ancestor_record_offset":12748,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":32,"record_offset":13336,"parent_record_offset":13308,"ancestor_record_offset":12748,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":32,"record_offset":13308,"parent_record_offset":13280,"ancestor_record_offset":12748,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] std::process::abort
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\process.rs
//     evidence={"module":32,"record_offset":14004,"parent_record_offset":12928,"ancestor_record_offset":12748,"inlinee":7827,"source_line":2378,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":32,"record_offset":13624,"parent_record_offset":13588,"ancestor_record_offset":12748,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":32,"record_offset":13436,"parent_record_offset":13240,"ancestor_record_offset":12748,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":32,"record_offset":13240,"parent_record_offset":13208,"ancestor_record_offset":12748,"inlinee":6011,"source_line":122,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":32,"record_offset":13072,"parent_record_offset":12928,"ancestor_record_offset":12748,"inlinee":6007,"source_line":435,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":32,"record_offset":13208,"parent_record_offset":13072,"ancestor_record_offset":12748,"inlinee":6010,"source_line":647,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":32,"record_offset":13588,"parent_record_offset":13560,"ancestor_record_offset":12748,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::abort_internal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\mod.rs
//     evidence={"module":32,"record_offset":14028,"parent_record_offset":14004,"ancestor_record_offset":12748,"inlinee":7829,"source_line":331,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":32,"record_offset":13116,"parent_record_offset":13072,"ancestor_record_offset":12748,"inlinee":6008,"source_line":27,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":32,"record_offset":13872,"parent_record_offset":13588,"ancestor_record_offset":12748,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":32,"record_offset":13280,"parent_record_offset":13240,"ancestor_record_offset":12748,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_handler
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2008
// 首条位置: {"module":32,"inlinee":8200}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":32,"record_offset":14144,"parent_record_offset":12928,"ancestor_record_offset":12928,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":32,"record_offset":13976,"parent_record_offset":12928,"ancestor_record_offset":12928,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":32,"record_offset":14348,"parent_record_offset":12928,"ancestor_record_offset":12928,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":32,"record_offset":13532,"parent_record_offset":12928,"ancestor_record_offset":12928,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":32,"record_offset":14060,"parent_record_offset":12928,"ancestor_record_offset":12928,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_direct] std::process::abort
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\process.rs
//     evidence={"module":32,"record_offset":14004,"parent_record_offset":12928,"ancestor_record_offset":12928,"inlinee":7827,"source_line":2378,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":32,"record_offset":13072,"parent_record_offset":12928,"ancestor_record_offset":12928,"inlinee":6007,"source_line":435,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":32,"record_offset":13560,"parent_record_offset":13532,"ancestor_record_offset":12928,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":32,"record_offset":13460,"parent_record_offset":13436,"ancestor_record_offset":12928,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":32,"record_offset":13812,"parent_record_offset":13624,"ancestor_record_offset":12928,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":32,"record_offset":13148,"parent_record_offset":13116,"ancestor_record_offset":12928,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":32,"record_offset":13904,"parent_record_offset":13872,"ancestor_record_offset":12928,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":32,"record_offset":13368,"parent_record_offset":13336,"ancestor_record_offset":12928,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":32,"record_offset":13172,"parent_record_offset":13148,"ancestor_record_offset":12928,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":32,"record_offset":13392,"parent_record_offset":13368,"ancestor_record_offset":12928,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":32,"record_offset":13836,"parent_record_offset":13812,"ancestor_record_offset":12928,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":32,"record_offset":13928,"parent_record_offset":13904,"ancestor_record_offset":12928,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":32,"record_offset":13336,"parent_record_offset":13308,"ancestor_record_offset":12928,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":32,"record_offset":13308,"parent_record_offset":13280,"ancestor_record_offset":12928,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":32,"record_offset":13624,"parent_record_offset":13588,"ancestor_record_offset":12928,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":32,"record_offset":13436,"parent_record_offset":13240,"ancestor_record_offset":12928,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":32,"record_offset":13240,"parent_record_offset":13208,"ancestor_record_offset":12928,"inlinee":6011,"source_line":122,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":32,"record_offset":13208,"parent_record_offset":13072,"ancestor_record_offset":12928,"inlinee":6010,"source_line":647,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":32,"record_offset":13588,"parent_record_offset":13560,"ancestor_record_offset":12928,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::abort_internal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\mod.rs
//     evidence={"module":32,"record_offset":14028,"parent_record_offset":14004,"ancestor_record_offset":12928,"inlinee":7829,"source_line":331,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":32,"record_offset":13116,"parent_record_offset":13072,"ancestor_record_offset":12928,"inlinee":6008,"source_line":27,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":32,"record_offset":13872,"parent_record_offset":13588,"ancestor_record_offset":12928,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":32,"record_offset":13280,"parent_record_offset":13240,"ancestor_record_offset":12928,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash
// 记录形式: compiled_procedure; instances=1
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":32,"record_offset":14440,"rva":900144,"va":5369609264}
// 编译实例 RVAs: 0xdbc30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14014b4d0
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t453856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609264,"raw_hex":"e8e0ec0600","size":5,"soft_fail":false,"target_va":5370066128,"va":5369612267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c7d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3867920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609264,"raw_hex":"e810053b00","size":5,"soft_fail":false,"target_va":5373478864,"va":5369610939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d0570
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5193218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609264,"raw_hex":"e8023e4f00","size":5,"soft_fail":false,"target_va":5374805360,"va":5369612137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5288656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609264,"raw_hex":"e8d0b25000","size":5,"soft_fail":false,"target_va":5374900736,"va":5369612075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5470642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609264,"raw_hex":"e8b2795300","size":5,"soft_fail":false,"target_va":5375081488,"va":5369610841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5472079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609264,"raw_hex":"e84f7f5300","size":5,"soft_fail":false,"target_va":5375081504,"va":5369609420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=9
//     target_aliases=5 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13218245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609264,"raw_hex":"e8c5b1c900","size":5,"soft_fail":false,"target_va":5382827728,"va":5369609478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a3530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41711521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609264,"raw_hex":"e8a1777c02","size":5,"soft_fail":false,"target_va":5411321136,"va":5369609610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42235293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609264,"raw_hex":"e89d758402","size":5,"soft_fail":false,"target_va":5411847168,"va":5369611870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t13230813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609264,"raw_hex":"e8dde2c900","size":5,"soft_fail":false,"target_va":5382840208,"va":5369609390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41929001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609264,"raw_hex":"e829c97f02","size":5,"soft_fail":false,"target_va":5411540963,"va":5369611957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13043767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609264,"raw_hex":"e83708c700","size":5,"soft_fail":false,"target_va":5382655024,"va":5369611252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<std::path::PathBuf,alloc::string::String> > >
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t-531667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609264,"raw_hex":"e82de3f7ff","size":5,"soft_fail":false,"target_va":5369080752,"va":5369612414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<std::path::PathBuf,enum2$<core::option::Option<country_core::resources::session::ActiveSession> > >,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609264,"raw_hex":"e803e9ffff","size":5,"soft_fail":false,"target_va":5369606704,"va":5369612584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::backup_folder_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10919101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609264,"raw_hex":"e8bd9ca600","size":5,"soft_fail":false,"target_va":5380530560,"va":5369611454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::save_folder_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10918732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609264,"raw_hex":"e84c9ba600","size":5,"soft_fail":false,"target_va":5380530176,"va":5369611439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13085120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609264,"raw_hex":"e8c0a9c700","size":5,"soft_fail":false,"target_va":5382694656,"va":5369609531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirBuilder::_create
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13100079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609264,"raw_hex":"e82fe4c700","size":5,"soft_fail":false,"target_va":5382709584,"va":5369609500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::panic_dump::dump_gpu_crash_dump_file
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2958247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609264,"raw_hex":"e8a7232d00","size":5,"soft_fail":false,"target_va":5372568944,"va":5369610692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::panic_dump::dump_inputs
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2953007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609264,"raw_hex":"e82f0f2d00","size":5,"soft_fail":false,"target_va":5372563056,"va":5369610044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::panic_dump::dump_log_file
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2952333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609264,"raw_hex":"e88d0c2d00","size":5,"soft_fail":false,"target_va":5372562544,"va":5369610206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::panic_dump::dump_profile_file
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2957769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609264,"raw_hex":"e8c9212d00","size":5,"soft_fail":false,"target_va":5372568304,"va":5369610530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::panic_dump::dump_rhapsody_log
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2959569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609264,"raw_hex":"e8d1282d00","size":5,"soft_fail":false,"target_va":5372569456,"va":5369609882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::panic_dump::dump_save
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2955389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609264,"raw_hex":"e87d182d00","size":5,"soft_fail":false,"target_va":5372565056,"va":5369609662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::panic_dump::dump_settings_file
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2957259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609264,"raw_hex":"e8cb1f2d00","size":5,"soft_fail":false,"target_va":5372567632,"va":5369610368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::panic_screenshot::dump_screenshot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4907619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609264,"raw_hex":"e863e24a00","size":5,"soft_fail":false,"target_va":5374517344,"va":5369609720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::recover_game_state_after_crash::logging_map_error
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609264,"raw_hex":"e8ea0d0000","size":5,"soft_fail":false,"target_va":5369613344,"va":5369609777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::file::date_time_string_for_filename
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t12298800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369609264,"raw_hex":"e830aabb00","size":5,"soft_fail":false,"target_va":5381908112,"va":5369609307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":32,"record_offset":14624,"parent_record_offset":14440,"ancestor_record_offset":14440,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":32,"record_offset":15268,"parent_record_offset":14440,"ancestor_record_offset":14440,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":32,"record_offset":17004,"parent_record_offset":14440,"ancestor_record_offset":14440,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":32,"record_offset":14704,"parent_record_offset":14440,"ancestor_record_offset":14440,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":32,"record_offset":16948,"parent_record_offset":14440,"ancestor_record_offset":14440,"inlinee":8221,"source_line":709,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":32,"record_offset":16976,"parent_record_offset":14440,"ancestor_record_offset":14440,"inlinee":8222,"source_line":681,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":32,"record_offset":17540,"parent_record_offset":14440,"ancestor_record_offset":14440,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":32,"record_offset":15660,"parent_record_offset":14440,"ancestor_record_offset":14440,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":32,"record_offset":14988,"parent_record_offset":14440,"ancestor_record_offset":14440,"inlinee":8207,"source_line":2766,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$40::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":32,"record_offset":14932,"parent_record_offset":14440,"ancestor_record_offset":14440,"inlinee":8204,"source_line":1858,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":32,"record_offset":15380,"parent_record_offset":14440,"ancestor_record_offset":14440,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":32,"record_offset":17332,"parent_record_offset":17308,"ancestor_record_offset":14440,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":32,"record_offset":17356,"parent_record_offset":17332,"ancestor_record_offset":14440,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":32,"record_offset":14880,"parent_record_offset":14856,"ancestor_record_offset":14440,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":32,"record_offset":17308,"parent_record_offset":17276,"ancestor_record_offset":14440,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":32,"record_offset":14856,"parent_record_offset":14800,"ancestor_record_offset":14440,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":32,"record_offset":17988,"parent_record_offset":17960,"ancestor_record_offset":14440,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":32,"record_offset":14672,"parent_record_offset":14648,"ancestor_record_offset":14440,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":32,"record_offset":17216,"parent_record_offset":17188,"ancestor_record_offset":14440,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":32,"record_offset":14828,"parent_record_offset":14800,"ancestor_record_offset":14440,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":32,"record_offset":14800,"parent_record_offset":14776,"ancestor_record_offset":14440,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":32,"record_offset":17276,"parent_record_offset":17244,"ancestor_record_offset":14440,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":32,"record_offset":17244,"parent_record_offset":17216,"ancestor_record_offset":14440,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":32,"record_offset":14776,"parent_record_offset":14752,"ancestor_record_offset":14440,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":32,"record_offset":17436,"parent_record_offset":17412,"ancestor_record_offset":14440,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":32,"record_offset":17160,"parent_record_offset":17112,"ancestor_record_offset":14440,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":32,"record_offset":17188,"parent_record_offset":17160,"ancestor_record_offset":14440,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":32,"record_offset":17040,"parent_record_offset":17004,"ancestor_record_offset":14440,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":32,"record_offset":17412,"parent_record_offset":17112,"ancestor_record_offset":14440,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":32,"record_offset":17076,"parent_record_offset":17040,"ancestor_record_offset":14440,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":32,"record_offset":17112,"parent_record_offset":17076,"ancestor_record_offset":14440,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":32,"record_offset":15340,"parent_record_offset":15300,"ancestor_record_offset":14440,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":32,"record_offset":15300,"parent_record_offset":15268,"ancestor_record_offset":14440,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":32,"record_offset":15040,"parent_record_offset":14988,"ancestor_record_offset":14440,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":32,"record_offset":18072,"parent_record_offset":17960,"ancestor_record_offset":14440,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":32,"record_offset":17460,"parent_record_offset":17436,"ancestor_record_offset":14440,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":32,"record_offset":17484,"parent_record_offset":17460,"ancestor_record_offset":14440,"inlinee":6052,"source_line":796,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":32,"record_offset":18244,"parent_record_offset":18204,"ancestor_record_offset":14440,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":32,"record_offset":14648,"parent_record_offset":14624,"ancestor_record_offset":14440,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":32,"record_offset":14728,"parent_record_offset":14704,"ancestor_record_offset":14440,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":32,"record_offset":15136,"parent_record_offset":15112,"ancestor_record_offset":14440,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":32,"record_offset":15184,"parent_record_offset":15160,"ancestor_record_offset":14440,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":32,"record_offset":15160,"parent_record_offset":15136,"ancestor_record_offset":14440,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":32,"record_offset":15236,"parent_record_offset":14988,"ancestor_record_offset":14440,"inlinee":8210,"source_line":3032,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":32,"record_offset":15012,"parent_record_offset":14988,"ancestor_record_offset":14440,"inlinee":8209,"source_line":3008,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":32,"record_offset":15112,"parent_record_offset":15088,"ancestor_record_offset":14440,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$38::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":32,"record_offset":14956,"parent_record_offset":14932,"ancestor_record_offset":14440,"inlinee":8206,"source_line":1836,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":32,"record_offset":15088,"parent_record_offset":15064,"ancestor_record_offset":14440,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":32,"record_offset":15064,"parent_record_offset":15040,"ancestor_record_offset":14440,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::recover_game_state_after_crash::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":32,"record_offset":15692,"parent_record_offset":15660,"ancestor_record_offset":14440,"inlinee":8214,"source_line":106,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::recover_game_state_after_crash::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":32,"record_offset":15896,"parent_record_offset":15864,"ancestor_record_offset":14440,"inlinee":8215,"source_line":109,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::recover_game_state_after_crash::closure$10
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":32,"record_offset":18204,"parent_record_offset":18164,"ancestor_record_offset":14440,"inlinee":8224,"source_line":143,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::recover_game_state_after_crash::closure$10::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":32,"record_offset":18280,"parent_record_offset":18244,"ancestor_record_offset":14440,"inlinee":8226,"source_line":145,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::recover_game_state_after_crash::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":32,"record_offset":16100,"parent_record_offset":16068,"ancestor_record_offset":14440,"inlinee":8216,"source_line":112,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::recover_game_state_after_crash::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":32,"record_offset":16304,"parent_record_offset":16272,"ancestor_record_offset":14440,"inlinee":8217,"source_line":114,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::recover_game_state_after_crash::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":32,"record_offset":16508,"parent_record_offset":16476,"ancestor_record_offset":14440,"inlinee":8218,"source_line":117,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::recover_game_state_after_crash::closure$5
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":32,"record_offset":16712,"parent_record_offset":16680,"ancestor_record_offset":14440,"inlinee":8219,"source_line":120,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::recover_game_state_after_crash::closure$6
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":32,"record_offset":16916,"parent_record_offset":16884,"ancestor_record_offset":14440,"inlinee":8220,"source_line":123,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::recover_game_state_after_crash::closure$9
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":32,"record_offset":17960,"parent_record_offset":17916,"ancestor_record_offset":14440,"inlinee":8223,"source_line":141,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash::logging_map_error
// 记录形式: compiled_procedure; instances=1
// 源码行: 95
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":32,"record_offset":19564,"rva":904224,"va":5369613344}
// 编译实例 RVAs: 0xdcc20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13226523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369613344,"raw_hex":"e81bd2c900","size":5,"soft_fail":false,"target_va":5382840208,"va":5369613680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13041312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369613344,"raw_hex":"e8a0fec600","size":5,"soft_fail":false,"target_va":5382655024,"va":5369613707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::stdio::_eprint
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13106577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369613344,"raw_hex":"e891fdc700","size":5,"soft_fail":false,"target_va":5382720160,"va":5369613578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":32,"record_offset":19768,"parent_record_offset":19564,"ancestor_record_offset":19564,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":32,"record_offset":19740,"parent_record_offset":19564,"ancestor_record_offset":19564,"inlinee":7173,"source_line":624,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":32,"record_offset":19852,"parent_record_offset":19564,"ancestor_record_offset":19564,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":32,"record_offset":19880,"parent_record_offset":19564,"ancestor_record_offset":19564,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":32,"record_offset":19816,"parent_record_offset":19792,"ancestor_record_offset":19564,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":32,"record_offset":19792,"parent_record_offset":19768,"ancestor_record_offset":19564,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 106
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2016
// 首条位置: {"module":32,"inlinee":8214}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 109
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2017
// 首条位置: {"module":32,"inlinee":8215}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 112
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2018
// 首条位置: {"module":32,"inlinee":8216}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2019
// 首条位置: {"module":32,"inlinee":8217}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 117
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x201A
// 首条位置: {"module":32,"inlinee":8218}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 120
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x201B
// 首条位置: {"module":32,"inlinee":8219}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 123
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x201C
// 首条位置: {"module":32,"inlinee":8220}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$7
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3783
// 首条位置: {"module":184,"inlinee":14211}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":184,"record_offset":5556,"parent_record_offset":4968,"ancestor_record_offset":4968,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":184,"record_offset":5020,"parent_record_offset":4968,"ancestor_record_offset":4968,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":184,"record_offset":5640,"parent_record_offset":4968,"ancestor_record_offset":4968,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":184,"record_offset":5668,"parent_record_offset":4968,"ancestor_record_offset":4968,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":5392,"parent_record_offset":5368,"ancestor_record_offset":4968,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":5416,"parent_record_offset":5392,"ancestor_record_offset":4968,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":5868,"parent_record_offset":5844,"ancestor_record_offset":4968,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":5368,"parent_record_offset":5340,"ancestor_record_offset":4968,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":5844,"parent_record_offset":5788,"ancestor_record_offset":4968,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":184,"record_offset":5604,"parent_record_offset":5580,"ancestor_record_offset":4968,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":184,"record_offset":5280,"parent_record_offset":5252,"ancestor_record_offset":4968,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":184,"record_offset":5816,"parent_record_offset":5788,"ancestor_record_offset":4968,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":184,"record_offset":5788,"parent_record_offset":5764,"ancestor_record_offset":4968,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":184,"record_offset":5340,"parent_record_offset":5308,"ancestor_record_offset":4968,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":184,"record_offset":5308,"parent_record_offset":5280,"ancestor_record_offset":4968,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":184,"record_offset":5764,"parent_record_offset":5740,"ancestor_record_offset":4968,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":184,"record_offset":5216,"parent_record_offset":5188,"ancestor_record_offset":4968,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":184,"record_offset":5188,"parent_record_offset":5160,"ancestor_record_offset":4968,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":184,"record_offset":5132,"parent_record_offset":5104,"ancestor_record_offset":4968,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":184,"record_offset":5160,"parent_record_offset":5132,"ancestor_record_offset":4968,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":184,"record_offset":5104,"parent_record_offset":5076,"ancestor_record_offset":4968,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":184,"record_offset":5076,"parent_record_offset":5048,"ancestor_record_offset":4968,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":184,"record_offset":5048,"parent_record_offset":5020,"ancestor_record_offset":4968,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":5252,"parent_record_offset":5216,"ancestor_record_offset":4968,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":184,"record_offset":5492,"parent_record_offset":5468,"ancestor_record_offset":4968,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":184,"record_offset":5580,"parent_record_offset":5556,"ancestor_record_offset":4968,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":184,"record_offset":5468,"parent_record_offset":5216,"ancestor_record_offset":4968,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":184,"record_offset":5692,"parent_record_offset":5668,"ancestor_record_offset":4968,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$9
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 141
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x201F
// 首条位置: {"module":32,"inlinee":8223}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":32,"record_offset":17988,"parent_record_offset":17960,"ancestor_record_offset":17960,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":32,"record_offset":18072,"parent_record_offset":17960,"ancestor_record_offset":17960,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":32,"record_offset":18100,"parent_record_offset":17960,"ancestor_record_offset":17960,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":32,"record_offset":18036,"parent_record_offset":18012,"ancestor_record_offset":17960,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":32,"record_offset":18012,"parent_record_offset":17988,"ancestor_record_offset":17960,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$10
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 143
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2020
// 首条位置: {"module":32,"inlinee":8224}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":32,"record_offset":18244,"parent_record_offset":18204,"ancestor_record_offset":18204,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":32,"record_offset":18644,"parent_record_offset":18204,"ancestor_record_offset":18204,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":32,"record_offset":18572,"parent_record_offset":18548,"ancestor_record_offset":18204,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":32,"record_offset":18548,"parent_record_offset":18492,"ancestor_record_offset":18204,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":32,"record_offset":18520,"parent_record_offset":18492,"ancestor_record_offset":18204,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":32,"record_offset":18492,"parent_record_offset":18468,"ancestor_record_offset":18204,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":32,"record_offset":18468,"parent_record_offset":18444,"ancestor_record_offset":18204,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":32,"record_offset":18324,"parent_record_offset":18280,"ancestor_record_offset":18204,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::recover_game_state_after_crash::closure$10::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":32,"record_offset":18280,"parent_record_offset":18244,"ancestor_record_offset":18204,"inlinee":8226,"source_line":145,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$10::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 145
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2022
// 首条位置: {"module":32,"inlinee":8226}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":32,"record_offset":18324,"parent_record_offset":18280,"ancestor_record_offset":18280,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":32,"record_offset":18572,"parent_record_offset":18548,"ancestor_record_offset":18280,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":32,"record_offset":18548,"parent_record_offset":18492,"ancestor_record_offset":18280,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":32,"record_offset":18520,"parent_record_offset":18492,"ancestor_record_offset":18280,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":32,"record_offset":18492,"parent_record_offset":18468,"ancestor_record_offset":18280,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":32,"record_offset":18468,"parent_record_offset":18444,"ancestor_record_offset":18280,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00032.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":32,"record_offset":18348,"parent_record_offset":18324,"ancestor_record_offset":18280,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00032.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] alloc::string::String (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] serde::__private::de::content::ContentDeserializer<serde_json::error::Error> (*)(enum2$<serde::__private::de::content::Content>)
// 候选参数类型: ["enum2$<serde::__private::de::content::Content>"]
// [文件级接口候选，具体函数归属 unknown] sentry_types::protocol::v7::Breadcrumb (*)(ref$<sentry_types::protocol::v7::Breadcrumb>)
// 候选参数类型: ["ref$<sentry_types::protocol::v7::Breadcrumb>"]

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_handler
// 信息 ID: ad94d3afb881dfbd5d849638
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::setup_panic_hook::closure$0; module=32, inline_record=12928; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash
// 信息 ID: 1ffb737508be027990374d85
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] crash_handler::make_crash_event::impl$0::on_crash<tiny_glade::crash_pickup::client::start::closure_env$1> @ 0xda21f -> 0xdbc30 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::setup_panic_hook::closure$0 @ 0xdb89c -> 0xdbc30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$0
// 信息 ID: 694ac9367be4894c5a6008df
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::recover_game_state_after_crash", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=32, inline_record=15692; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$1
// 信息 ID: a9303d29cf2eb92f1eaff0cc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::panic_reporter::recover_game_state_after_crash", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=32, inline_record=15896; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$10
// 信息 ID: 73f34e3414651504071842f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 10, "lexical_owner": "tiny_glade::panic_reporter::recover_game_state_after_crash", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=32, inline_record=18204; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$10::closure$0
// 信息 ID: 86f611b451ce5edeb68aa636
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 10, "lexical_owner": "tiny_glade::panic_reporter::recover_game_state_after_crash", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=32, inline_record=18280; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$2
// 信息 ID: 16739157ea0e31a8fbc18fd4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::panic_reporter::recover_game_state_after_crash", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=32, inline_record=16100; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$3
// 信息 ID: bc4408524665dbff71bf32e5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tiny_glade::panic_reporter::recover_game_state_after_crash", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=32, inline_record=16304; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$4
// 信息 ID: d5bf91e2ff119596f93a1bf1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::panic_reporter::recover_game_state_after_crash", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=32, inline_record=16508; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$5
// 信息 ID: ca019e6b7b624f4dacc551fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "tiny_glade::panic_reporter::recover_game_state_after_crash", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=32, inline_record=16712; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$6
// 信息 ID: 61d175745afdea7520ec2187
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "tiny_glade::panic_reporter::recover_game_state_after_crash", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=32, inline_record=16916; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$7
// 信息 ID: c75fe32a77140f338e9250ed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "tiny_glade::panic_reporter::recover_game_state_after_crash", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::recover_game_state_after_crash::closure_env$7 <- core::iter::adapters::map::impl$2::fold<sentry_types::protocol::v7::Breadcrumb,core::option::IntoIter<ref$<anyhow::Error> >,tiny_glade::panic_reporter::recover_game_state_after_crash::closure_env$7,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<sentry_types::protocol::v7::Breadcrumb,alloc::vec::impl$20::extend_trusted::closure_env$0<sentry_types::protocol::v7::Breadcrumb,alloc::alloc::Global,core::iter::adapters::map::Map<core::option::IntoIter<ref$<anyhow::Error> >,tiny_glade::panic_reporter::recover_game_state_after_crash::closure_env$7> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::recover_game_state_after_crash::closure_env$7 <- core::iter::adapters::map::impl$2::fold<sentry_types::protocol::v7::Breadcrumb,core::option::IntoIter<ref$<anyhow::Error> >,tiny_glade::panic_reporter::recover_game_state_after_crash::closure_env$7,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<sentry_types::protocol::v7::Breadcrumb,alloc::vec::impl$20::extend_trusted::closure_env$0<sentry_types::protocol::v7::Breadcrumb,alloc::alloc::Global,core::iter::adapters::map::Map<core::option::IntoIter<ref$<anyhow::Error> >,tiny_glade::panic_reporter::recover_game_state_after_crash::closure_env$7> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<sentry_types::protocol::v7::Breadcrumb,core::option::IntoIter<ref$<anyhow::Error> >,tiny_glade::panic_reporter::recover_game_state_after_crash::closure_env$7,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<sentry_types::protocol::v7::Breadcrumb,alloc::vec::impl$20::extend_trusted::closure_env$0<sentry_types::protocol::v7::Breadcrumb,alloc::alloc::Global,core::iter::adapters::map::Map<core::option::IntoIter<ref$<anyhow::Error> >,tiny_glade::panic_reporter::recover_game_state_after_crash::closure_env$7> > > >; module=184, inline_record=4968; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$9
// 信息 ID: 7e8a596368b909241bf3d75c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 9, "lexical_owner": "tiny_glade::panic_reporter::recover_game_state_after_crash", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=32, inline_record=17960; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash::logging_map_error
// 信息 ID: 6bcf401228ca49004dd9c194
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0xdbe31 -> 0xdcc20 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0xdbed3 -> 0xdcc20 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0xdbf75 -> 0xdcc20 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0xdc017 -> 0xdcc20 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0xdc0b9 -> 0xdcc20 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0xdc15b -> 0xdcc20 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0xdc1fd -> 0xdcc20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::setup_panic_hook
// 信息 ID: afd6a882d782ffb6f937ae9d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::main @ 0x3dd5c9 -> 0xdb5b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::setup_panic_hook::closure$0
// 信息 ID: 1d7c2d01b5e62c8696ed49e2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::setup_panic_hook", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::setup_panic_hook::closure_env$0 <- core::ops::function::FnOnce::call_once<tiny_glade::panic_reporter::setup_panic_hook::closure_env$0,tuple$<ref$<std::panic::PanicHookInfo> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::setup_panic_hook::closure_env$0 <- core::ptr::drop_in_place<tiny_glade::panic_reporter::setup_panic_hook::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::panic_reporter::setup_panic_hook::closure_env$0,tuple$<ref$<std::panic::PanicHookInfo> > > @ 0xdb1a6 -> 0xdb6b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
