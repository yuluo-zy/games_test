// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tiny-glade/src/panic_reporter/panic_screenshot.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: e26a558c975d33cf6f17498eabcac77f237313602a81fa458f8dd4bd5ba41255
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: panic_screenshot.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 184, 238
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use image; // namespace candidate
//   package identities: [{"name":"image","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.24.9"},{"name":"image","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.25.1"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.9"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_screenshot::create_graphics_resources
// 记录形式: compiled_procedure; instances=1
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":238,"record_offset":23596,"rva":5806240,"va":5374515360}
// 编译实例 RVAs: 0x5898a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8128573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374515360,"raw_hex":"e83d087c00","size":5,"soft_fail":false,"target_va":5382644528,"va":5374515950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dcef0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4902239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374515360,"raw_hex":"e8a132b5ff","size":5,"soft_fail":false,"target_va":5369614064,"va":5374516298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ebcf0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t401869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374515360,"raw_hex":"e8cd210600","size":5,"soft_fail":false,"target_va":5374917872,"va":5374515998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t565452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374515360,"raw_hex":"e8cca00800","size":5,"soft_fail":false,"target_va":5375081504,"va":5374516047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1000213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374515360,"raw_hex":"e815430f00","size":5,"soft_fail":false,"target_va":5375516032,"va":5374515814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6654198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374515360,"raw_hex":"e8f6886500","size":5,"soft_fail":false,"target_va":5381170352,"va":5374516149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140beea80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6705548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374515360,"raw_hex":"e88c516600","size":5,"soft_fail":false,"target_va":5381220992,"va":5374515439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37025867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374515360,"raw_hex":"e84bf83402","size":5,"soft_fail":false,"target_va":5411542064,"va":5374516192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,enum2$<core::option::Option<tiny_glade::panic_reporter::panic_screenshot::GraphicsResources> > > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374515360,"raw_hex":"e86af5ffff","size":5,"soft_fail":false,"target_va":5374513456,"va":5374516161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t36976228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374515360,"raw_hex":"e864363402","size":5,"soft_fail":false,"target_va":5411492368,"va":5374516135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t36979100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374515360,"raw_hex":"e89c413402","size":5,"soft_fail":false,"target_va":5411494800,"va":5374515695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7308068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374515360,"raw_hex":"e824836f00","size":5,"soft_fail":false,"target_va":5381823600,"va":5374515527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7308355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374515360,"raw_hex":"e843846f00","size":5,"soft_fail":false,"target_va":5381824016,"va":5374515656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6600074","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374515360,"raw_hex":"e88ab56400","size":5,"soft_fail":false,"target_va":5381115488,"va":5374515409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":238,"record_offset":24368,"parent_record_offset":23596,"ancestor_record_offset":23596,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":238,"record_offset":24048,"parent_record_offset":23596,"ancestor_record_offset":23596,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":238,"record_offset":23932,"parent_record_offset":23596,"ancestor_record_offset":23596,"inlinee":8786,"source_line":1784,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":238,"record_offset":24104,"parent_record_offset":23596,"ancestor_record_offset":23596,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":238,"record_offset":23776,"parent_record_offset":23596,"ancestor_record_offset":23596,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":238,"record_offset":24400,"parent_record_offset":23596,"ancestor_record_offset":23596,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":238,"record_offset":24284,"parent_record_offset":23596,"ancestor_record_offset":23596,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":238,"record_offset":24800,"parent_record_offset":24776,"ancestor_record_offset":23596,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":238,"record_offset":24776,"parent_record_offset":24720,"ancestor_record_offset":23596,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":238,"record_offset":24748,"parent_record_offset":24720,"ancestor_record_offset":23596,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":238,"record_offset":24720,"parent_record_offset":24696,"ancestor_record_offset":23596,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":238,"record_offset":24696,"parent_record_offset":24672,"ancestor_record_offset":23596,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":238,"record_offset":23956,"parent_record_offset":23932,"ancestor_record_offset":23596,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":238,"record_offset":24072,"parent_record_offset":24048,"ancestor_record_offset":23596,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":238,"record_offset":24456,"parent_record_offset":24424,"ancestor_record_offset":23596,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":238,"record_offset":24484,"parent_record_offset":24424,"ancestor_record_offset":23596,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":238,"record_offset":24600,"parent_record_offset":24576,"ancestor_record_offset":23596,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":238,"record_offset":23984,"parent_record_offset":23956,"ancestor_record_offset":23596,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":238,"record_offset":24012,"parent_record_offset":23956,"ancestor_record_offset":23596,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":238,"record_offset":24216,"parent_record_offset":24152,"ancestor_record_offset":23596,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":238,"record_offset":23840,"parent_record_offset":23804,"ancestor_record_offset":23596,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":238,"record_offset":24308,"parent_record_offset":24284,"ancestor_record_offset":23596,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":238,"record_offset":24240,"parent_record_offset":24216,"ancestor_record_offset":23596,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":238,"record_offset":23864,"parent_record_offset":23840,"ancestor_record_offset":23596,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":238,"record_offset":24332,"parent_record_offset":24308,"ancestor_record_offset":23596,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":238,"record_offset":24128,"parent_record_offset":24104,"ancestor_record_offset":23596,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":238,"record_offset":24512,"parent_record_offset":24424,"ancestor_record_offset":23596,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":238,"record_offset":24424,"parent_record_offset":24400,"ancestor_record_offset":23596,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":238,"record_offset":24536,"parent_record_offset":24512,"ancestor_record_offset":23596,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":238,"record_offset":23804,"parent_record_offset":23776,"ancestor_record_offset":23596,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":238,"record_offset":24152,"parent_record_offset":24128,"ancestor_record_offset":23596,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":238,"record_offset":23896,"parent_record_offset":23804,"ancestor_record_offset":23596,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":238,"record_offset":24188,"parent_record_offset":24152,"ancestor_record_offset":23596,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_screenshot::capture_image
// 记录形式: compiled_procedure; instances=1
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":238,"record_offset":25620,"rva":5807248,"va":5374516368}
// 编译实例 RVAs: 0x589c90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140155400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4409845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374516368,"raw_hex":"e80bb6bcff","size":5,"soft_fail":false,"target_va":5370106880,"va":5374516720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140587eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374516368,"raw_hex":"e8f2e0ffff","size":5,"soft_fail":false,"target_va":5374508720,"va":5374516665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140609520
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t521498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374516368,"raw_hex":"e81af50700","size":5,"soft_fail":false,"target_va":5375038752,"va":5374517249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t564684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374516368,"raw_hex":"e8cc9d0800","size":5,"soft_fail":false,"target_va":5375081488,"va":5374516799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t564645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374516368,"raw_hex":"e8a59d0800","size":5,"soft_fail":false,"target_va":5375081504,"va":5374516854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd4fc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6599221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374516368,"raw_hex":"e835b26400","size":5,"soft_fail":false,"target_va":5381115840,"va":5374516614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd5120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6599241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374516368,"raw_hex":"e849b26400","size":5,"soft_fail":false,"target_va":5381116192,"va":5374516946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37330418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374516368,"raw_hex":"e8f29d3902","size":5,"soft_fail":false,"target_va":5411847168,"va":5374516745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37023804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374516368,"raw_hex":"e83cf03402","size":5,"soft_fail":false,"target_va":5411540963,"va":5374517154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37024929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374516368,"raw_hex":"e8a1f43402","size":5,"soft_fail":false,"target_va":5411542064,"va":5374517130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,enum2$<core::option::Option<tiny_glade::panic_reporter::panic_screenshot::GraphicsResources> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374516368,"raw_hex":"e861f1ffff","size":5,"soft_fail":false,"target_va":5374513456,"va":5374517194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::bind_external_storage_buffer
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4080754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374516368,"raw_hex":"e872443e00","size":5,"soft_fail":false,"target_va":5378597536,"va":5374516777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_1d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4082731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374516368,"raw_hex":"e82b4c3e00","size":5,"soft_fail":false,"target_va":5378599664,"va":5374516928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t36975357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374516368,"raw_hex":"e8fd323402","size":5,"soft_fail":false,"target_va":5411492368,"va":5374517006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t36977809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374516368,"raw_hex":"e8913c3402","size":5,"soft_fail":false,"target_va":5411494800,"va":5374516986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7307157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374516368,"raw_hex":"e8957f6f00","size":5,"soft_fail":false,"target_va":5381823600,"va":5374516438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7307053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374516368,"raw_hex":"e82d7f6f00","size":5,"soft_fail":false,"target_va":5381824016,"va":5374516958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6601375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374516368,"raw_hex":"e89fba6400","size":5,"soft_fail":false,"target_va":5381117840,"va":5374516460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tjmp\t36977689","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5374516368,"raw_hex":"e9193c3402","size":5,"soft_fail":false,"target_va":5411494800,"va":5374517106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":238,"record_offset":26056,"parent_record_offset":25620,"ancestor_record_offset":25620,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::div_ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":238,"record_offset":26828,"parent_record_offset":25620,"ancestor_record_offset":25620,"inlinee":8810,"source_line":3197,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":238,"record_offset":25968,"parent_record_offset":25620,"ancestor_record_offset":25620,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":238,"record_offset":26888,"parent_record_offset":25620,"ancestor_record_offset":25620,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::push_constants
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":238,"record_offset":26156,"parent_record_offset":25620,"ancestor_record_offset":25620,"inlinee":8813,"source_line":415,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":238,"record_offset":26112,"parent_record_offset":25620,"ancestor_record_offset":25620,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":238,"record_offset":25804,"parent_record_offset":25620,"ancestor_record_offset":25620,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::export
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":238,"record_offset":26860,"parent_record_offset":25620,"ancestor_record_offset":25620,"inlinee":9695,"source_line":75,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::import_unowned
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":238,"record_offset":26084,"parent_record_offset":25620,"ancestor_record_offset":25620,"inlinee":9692,"source_line":55,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":238,"record_offset":26000,"parent_record_offset":25620,"ancestor_record_offset":25620,"inlinee":8273,"source_line":315,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":238,"record_offset":26028,"parent_record_offset":25620,"ancestor_record_offset":25620,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":238,"record_offset":26476,"parent_record_offset":26452,"ancestor_record_offset":25620,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":238,"record_offset":26500,"parent_record_offset":26476,"ancestor_record_offset":25620,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":238,"record_offset":26776,"parent_record_offset":26752,"ancestor_record_offset":25620,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":238,"record_offset":26452,"parent_record_offset":26424,"ancestor_record_offset":25620,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":238,"record_offset":26752,"parent_record_offset":26696,"ancestor_record_offset":25620,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":238,"record_offset":26364,"parent_record_offset":26336,"ancestor_record_offset":25620,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":238,"record_offset":26724,"parent_record_offset":26696,"ancestor_record_offset":25620,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":238,"record_offset":26696,"parent_record_offset":26672,"ancestor_record_offset":25620,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":238,"record_offset":26424,"parent_record_offset":26392,"ancestor_record_offset":25620,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":238,"record_offset":26392,"parent_record_offset":26364,"ancestor_record_offset":25620,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":238,"record_offset":26672,"parent_record_offset":26648,"ancestor_record_offset":25620,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":238,"record_offset":26300,"parent_record_offset":26268,"ancestor_record_offset":25620,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":238,"record_offset":26268,"parent_record_offset":26236,"ancestor_record_offset":25620,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":238,"record_offset":26204,"parent_record_offset":26156,"ancestor_record_offset":25620,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":238,"record_offset":26236,"parent_record_offset":26204,"ancestor_record_offset":25620,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":238,"record_offset":26336,"parent_record_offset":26300,"ancestor_record_offset":25620,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":238,"record_offset":26576,"parent_record_offset":26552,"ancestor_record_offset":25620,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":238,"record_offset":26552,"parent_record_offset":26300,"ancestor_record_offset":25620,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":238,"record_offset":26624,"parent_record_offset":26156,"ancestor_record_offset":25620,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":238,"record_offset":26996,"parent_record_offset":26936,"ancestor_record_offset":25620,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":238,"record_offset":25872,"parent_record_offset":25832,"ancestor_record_offset":25620,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":238,"record_offset":27020,"parent_record_offset":26996,"ancestor_record_offset":25620,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":238,"record_offset":25896,"parent_record_offset":25872,"ancestor_record_offset":25620,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":238,"record_offset":26912,"parent_record_offset":26888,"ancestor_record_offset":25620,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":238,"record_offset":25832,"parent_record_offset":25804,"ancestor_record_offset":25620,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":238,"record_offset":26936,"parent_record_offset":26912,"ancestor_record_offset":25620,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":238,"record_offset":25928,"parent_record_offset":25832,"ancestor_record_offset":25620,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":238,"record_offset":26968,"parent_record_offset":26936,"ancestor_record_offset":25620,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_screenshot::dump_screenshot
// 记录形式: compiled_procedure; instances=1
// 源码行: 105
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":238,"record_offset":27256,"rva":5808224,"va":5374517344}
// 编译实例 RVAs: 0x58a060
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402bad40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2946618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374517344,"raw_hex":"e8c609d3ff","size":5,"soft_fail":false,"target_va":5371571520,"va":5374518133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ce730
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t278707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374517344,"raw_hex":"e8b3400400","size":5,"soft_fail":false,"target_va":5374797616,"va":5374518904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t563471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374517344,"raw_hex":"e80f990800","size":5,"soft_fail":false,"target_va":5375081504,"va":5374518028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t997822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374517344,"raw_hex":"e8be390f00","size":5,"soft_fail":false,"target_va":5375516032,"va":5374518205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8309834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374517344,"raw_hex":"e84acc7e00","size":5,"soft_fail":false,"target_va":5382827728,"va":5374517889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a2020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36798231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374517344,"raw_hex":"e8177f3102","size":5,"soft_fail":false,"target_va":5411315744,"va":5374517508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a2b20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36800837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374517344,"raw_hex":"e845893102","size":5,"soft_fail":false,"target_va":5411318560,"va":5374517718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a31b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36801641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374517344,"raw_hex":"e8698c3102","size":5,"soft_fail":false,"target_va":5411320240,"va":5374518594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a4000
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36805955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374517344,"raw_hex":"e8439d3102","size":5,"soft_fail":false,"target_va":5411323904,"va":5374517944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t8322494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374517344,"raw_hex":"e8befd7e00","size":5,"soft_fail":false,"target_va":5382840208,"va":5374517709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37022997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374517344,"raw_hex":"e815ed3402","size":5,"soft_fail":false,"target_va":5411541776,"va":5374518774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,enum2$<core::option::Option<tiny_glade::panic_reporter::panic_screenshot::GraphicsResources> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374517344,"raw_hex":"e833eaffff","size":5,"soft_fail":false,"target_va":5374513456,"va":5374519032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t37024372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374517344,"raw_hex":"e874f23402","size":5,"soft_fail":false,"target_va":5411543104,"va":5374518727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::io::free_functions::save_buffer_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6394198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374517344,"raw_hex":"e856916100","size":5,"soft_fail":false,"target_va":5380912464,"va":5374518261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t36973684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374517344,"raw_hex":"e8742c3402","size":5,"soft_fail":false,"target_va":5411492368,"va":5374518679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t36976405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374517344,"raw_hex":"e815373402","size":5,"soft_fail":false,"target_va":5411494800,"va":5374518390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7306154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374517344,"raw_hex":"e8aa7b6f00","size":5,"soft_fail":false,"target_va":5381823600,"va":5374517441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7306031","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374517344,"raw_hex":"e82f7b6f00","size":5,"soft_fail":false,"target_va":5381824016,"va":5374517980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8176193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374517344,"raw_hex":"e841c27c00","size":5,"soft_fail":false,"target_va":5382694656,"va":5374518458,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirBuilder::_create
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8191648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374517344,"raw_hex":"e8a0fe7c00","size":5,"soft_fail":false,"target_va":5382709584,"va":5374517931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8226671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374517344,"raw_hex":"e86f877d00","size":5,"soft_fail":false,"target_va":5382744080,"va":5374517404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::parent
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8223298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374517344,"raw_hex":"e8427a7d00","size":5,"soft_fail":false,"target_va":5382741200,"va":5374517897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":238,"record_offset":28008,"parent_record_offset":27256,"ancestor_record_offset":27256,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":238,"record_offset":29744,"parent_record_offset":27256,"ancestor_record_offset":27256,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$6::try_into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":238,"record_offset":27772,"parent_record_offset":27256,"ancestor_record_offset":27256,"inlinee":9618,"source_line":799,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":238,"record_offset":27676,"parent_record_offset":27256,"ancestor_record_offset":27256,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":238,"record_offset":29060,"parent_record_offset":27256,"ancestor_record_offset":27256,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":238,"record_offset":27648,"parent_record_offset":27256,"ancestor_record_offset":27256,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":238,"record_offset":30532,"parent_record_offset":27256,"ancestor_record_offset":27256,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":238,"record_offset":28428,"parent_record_offset":27256,"ancestor_record_offset":27256,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":238,"record_offset":28400,"parent_record_offset":27256,"ancestor_record_offset":27256,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":238,"record_offset":27704,"parent_record_offset":27256,"ancestor_record_offset":27256,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::ImageBuffer::from_raw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":238,"record_offset":29144,"parent_record_offset":27256,"ancestor_record_offset":27256,"inlinee":7813,"source_line":674,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::ImageBuffer::save
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":238,"record_offset":29488,"parent_record_offset":27256,"ancestor_record_offset":27256,"inlinee":9521,"source_line":996,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":238,"record_offset":27492,"parent_record_offset":27256,"ancestor_record_offset":27256,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":238,"record_offset":28316,"parent_record_offset":27256,"ancestor_record_offset":27256,"inlinee":8207,"source_line":2766,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":238,"record_offset":27464,"parent_record_offset":27256,"ancestor_record_offset":27256,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":238,"record_offset":28176,"parent_record_offset":27256,"ancestor_record_offset":27256,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":238,"record_offset":28864,"parent_record_offset":28840,"ancestor_record_offset":27256,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":238,"record_offset":28840,"parent_record_offset":28784,"ancestor_record_offset":27256,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":238,"record_offset":28056,"parent_record_offset":28032,"ancestor_record_offset":27256,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":238,"record_offset":28812,"parent_record_offset":28784,"ancestor_record_offset":27256,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":238,"record_offset":28784,"parent_record_offset":28760,"ancestor_record_offset":27256,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":238,"record_offset":28760,"parent_record_offset":28736,"ancestor_record_offset":27256,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":238,"record_offset":29084,"parent_record_offset":29060,"ancestor_record_offset":27256,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":238,"record_offset":29820,"parent_record_offset":29780,"ancestor_record_offset":27256,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":238,"record_offset":29780,"parent_record_offset":29744,"ancestor_record_offset":27256,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$7::try_from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":238,"record_offset":27796,"parent_record_offset":27772,"ancestor_record_offset":27256,"inlinee":10569,"source_line":236,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":238,"record_offset":29540,"parent_record_offset":29516,"ancestor_record_offset":27256,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":238,"record_offset":29108,"parent_record_offset":29084,"ancestor_record_offset":27256,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":238,"record_offset":29272,"parent_record_offset":29248,"ancestor_record_offset":27256,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":238,"record_offset":29296,"parent_record_offset":29272,"ancestor_record_offset":27256,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":238,"record_offset":29988,"parent_record_offset":29960,"ancestor_record_offset":27256,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":238,"record_offset":29224,"parent_record_offset":29196,"ancestor_record_offset":27256,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":238,"record_offset":29452,"parent_record_offset":29168,"ancestor_record_offset":27256,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":238,"record_offset":28032,"parent_record_offset":28008,"ancestor_record_offset":27256,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":238,"record_offset":28640,"parent_record_offset":28616,"ancestor_record_offset":27256,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":238,"record_offset":27820,"parent_record_offset":27796,"ancestor_record_offset":27256,"inlinee":10570,"source_line":1547,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":238,"record_offset":29020,"parent_record_offset":28996,"ancestor_record_offset":27256,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":238,"record_offset":28996,"parent_record_offset":28960,"ancestor_record_offset":27256,"inlinee":4612,"source_line":101,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":238,"record_offset":27732,"parent_record_offset":27704,"ancestor_record_offset":27256,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":238,"record_offset":28548,"parent_record_offset":28484,"ancestor_record_offset":27256,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":238,"record_offset":27556,"parent_record_offset":27520,"ancestor_record_offset":27256,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":238,"record_offset":28572,"parent_record_offset":28548,"ancestor_record_offset":27256,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":238,"record_offset":27580,"parent_record_offset":27556,"ancestor_record_offset":27256,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::check_image_fits
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":238,"record_offset":29168,"parent_record_offset":29144,"ancestor_record_offset":27256,"inlinee":7814,"source_line":802,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::image_buffer_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":238,"record_offset":29196,"parent_record_offset":29168,"ancestor_record_offset":27256,"inlinee":7815,"source_line":807,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":238,"record_offset":29248,"parent_record_offset":29224,"ancestor_record_offset":27256,"inlinee":7817,"source_line":809,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$1
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":238,"record_offset":29360,"parent_record_offset":29336,"ancestor_record_offset":27256,"inlinee":7818,"source_line":810,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] image::dynimage::save_buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/dynimage.rs
//     evidence={"module":238,"record_offset":29516,"parent_record_offset":29488,"ancestor_record_offset":27256,"inlinee":7842,"source_line":1111,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":238,"record_offset":28456,"parent_record_offset":28428,"ancestor_record_offset":27256,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":238,"record_offset":27520,"parent_record_offset":27492,"ancestor_record_offset":27256,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":238,"record_offset":28484,"parent_record_offset":28456,"ancestor_record_offset":27256,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":238,"record_offset":27612,"parent_record_offset":27520,"ancestor_record_offset":27256,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":238,"record_offset":28520,"parent_record_offset":28484,"ancestor_record_offset":27256,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":238,"record_offset":28224,"parent_record_offset":28200,"ancestor_record_offset":27256,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":238,"record_offset":28272,"parent_record_offset":28248,"ancestor_record_offset":27256,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":238,"record_offset":28248,"parent_record_offset":28224,"ancestor_record_offset":27256,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":238,"record_offset":28368,"parent_record_offset":28316,"ancestor_record_offset":27256,"inlinee":8210,"source_line":3032,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":238,"record_offset":28340,"parent_record_offset":28316,"ancestor_record_offset":27256,"inlinee":8209,"source_line":3008,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":238,"record_offset":28200,"parent_record_offset":28176,"ancestor_record_offset":27256,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":238,"record_offset":29588,"parent_record_offset":29564,"ancestor_record_offset":27256,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":238,"record_offset":29564,"parent_record_offset":29540,"ancestor_record_offset":27256,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00238.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::ycbcr_to_srgb
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3796
// 首条位置: {"module":184,"inlinee":14230}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":184,"record_offset":27772,"parent_record_offset":27700,"ancestor_record_offset":27700,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 158
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3795
// 首条位置: {"module":184,"inlinee":14229}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::ycbcr_to_srgb
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_screenshot.rs
//     evidence={"module":184,"record_offset":27700,"parent_record_offset":27632,"ancestor_record_offset":27632,"inlinee":14230,"source_line":136,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":184,"record_offset":27772,"parent_record_offset":27700,"ancestor_record_offset":27632,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00184.jsonl.gz"}
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
// 完整结构及原始类型记录: panic_screenshot.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::panic_reporter::panic_screenshot::GraphicsResources
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<tiny_glade::panic_reporter::panic_screenshot::GraphicsResources>", "constructor": "core::option::Option", "arguments": ["tiny_glade::panic_reporter::panic_screenshot::GraphicsResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<tiny_glade::panic_reporter::panic_screenshot::GraphicsResources> >", "constructor": "enum2$", "arguments": ["core::option::Option<tiny_glade::panic_reporter::panic_screenshot::GraphicsResources>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,enum2$<core::option::Option<tiny_glade::panic_reporter::panic_screenshot::GraphicsResources> > >", "constructor": "lock_api::mutex::MutexGuard", "arguments": ["parking_lot::raw_mutex::RawMutex", "enum2$<core::option::Option<tiny_glade::panic_reporter::panic_screenshot::GraphicsResources> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,enum2$<core::option::Option<tiny_glade::panic_reporter::panic_screenshot::GraphicsResources> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,enum2$<core::option::Option<tiny_glade::panic_reporter::panic_screenshot::GraphicsResources> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: panic_screenshot.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: panic_screenshot.rs.enriched.json.gz
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
// 函数: tiny_glade::panic_reporter::panic_screenshot::capture_image
// 信息 ID: 6ce5ebdd151b5c598152aa02
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x16356b -> 0x589c90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_screenshot::create_graphics_resources
// 信息 ID: 4a3b9a2290dca0a53db962d4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x59d67b -> 0x5898a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_screenshot::dump_screenshot
// 信息 ID: b069e6e8123ccbdbbc4a2841
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0xdbdf8 -> 0x58a060 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure$0
// 信息 ID: 6f628836365a6659330e9f8a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::panic_screenshot::dump_screenshot", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<array$<array$<u8,3>,2>,core::iter::adapters::map::Map<core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0 <- core::iter::adapters::map::impl$2::fold<array$<array$<u8,3>,2>,core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<array$<u8,3>,2>,alloc::vec::impl$20::extend_trusted::closure_env$0<array$<array$<u8,3>,2>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0 <- core::iter::adapters::map::impl$2::fold<array$<array$<u8,3>,2>,core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<array$<u8,3>,2>,alloc::vec::impl$20::extend_trusted::closure_env$0<array$<array$<u8,3>,2>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<array$<array$<u8,3>,2>,core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<array$<u8,3>,2>,alloc::vec::impl$20::extend_trusted::closure_env$0<array$<array$<u8,3>,2>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0> > > >; module=184, inline_record=27632; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::ycbcr_to_srgb
// 信息 ID: 404df432ec2a50a654fd601b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<array$<array$<u8,3>,2>,core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<array$<u8,3>,2>,alloc::vec::impl$20::extend_trusted::closure_env$0<array$<array$<u8,3>,2>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0> > > >; module=184, inline_record=27700; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<array$<array$<u8,3>,2>,core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<array$<u8,3>,2>,alloc::vec::impl$20::extend_trusted::closure_env$0<array$<array$<u8,3>,2>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0> > > >; module=184, inline_record=27888; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
