// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tiny-glade/src/hang_watchdog.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 42d3f11497a9486bc3e300a1a0ea7a8edfa46f999ecf731737295cf4e07a4bff
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: hang_watchdog.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 145
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::hang_watchdog::hang_watchdog
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":145,"record_offset":24276,"rva":3563040,"va":5372272160}
// 编译实例 RVAs: 0x365e20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t10371925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372272160,"raw_hex":"e855439e00","size":5,"soft_fail":false,"target_va":5382644528,"va":5372272598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..time..Instant$u20$as$u20$core..ops..arith..Add$LT$core..time..Duration$GT$$GT$::add
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t10476069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372272160,"raw_hex":"e825da9f00","size":5,"soft_fail":false,"target_va":5382748704,"va":5372272630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b1430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hang_watchdog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1355200","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372272160,"raw_hex":"e8c0ad1400","size":5,"soft_fail":false,"target_va":5373629488,"va":5372274283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d2000
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hang_watchdog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2537780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372272160,"raw_hex":"e834b92600","size":5,"soft_fail":false,"target_va":5374812160,"va":5372274375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: hang_watchdog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2808660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372272160,"raw_hex":"e854db2a00","size":5,"soft_fail":false,"target_va":5375081504,"va":5372272839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067d350
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hang_watchdog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3241215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372272160,"raw_hex":"e8ff743100","size":5,"soft_fail":false,"target_va":5375513424,"va":5372272204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=8
//     target_aliases=69 candidates; complete list: hang_watchdog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3243781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372272160,"raw_hex":"e8057f3100","size":5,"soft_fail":false,"target_va":5375516032,"va":5372272246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::check_deadlock
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9551696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372272160,"raw_hex":"e850bf9100","size":5,"soft_fail":false,"target_va":5381824448,"va":5372272747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::sleep
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10418529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372272160,"raw_hex":"e861f99e00","size":5,"soft_fail":false,"target_va":5382691200,"va":5372272666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::now
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t10474998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372272160,"raw_hex":"e8f6d59f00","size":5,"soft_fail":false,"target_va":5382747616,"va":5372272613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":145,"record_offset":25348,"parent_record_offset":24276,"ancestor_record_offset":24276,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":145,"record_offset":26204,"parent_record_offset":24276,"ancestor_record_offset":24276,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":145,"record_offset":24532,"parent_record_offset":24276,"ancestor_record_offset":24276,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":145,"record_offset":25136,"parent_record_offset":24276,"ancestor_record_offset":24276,"inlinee":5087,"source_line":1371,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":145,"record_offset":27900,"parent_record_offset":24276,"ancestor_record_offset":24276,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":145,"record_offset":25824,"parent_record_offset":24276,"ancestor_record_offset":24276,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":145,"record_offset":25376,"parent_record_offset":24276,"ancestor_record_offset":24276,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":145,"record_offset":25740,"parent_record_offset":24276,"ancestor_record_offset":24276,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":145,"record_offset":26464,"parent_record_offset":24276,"ancestor_record_offset":24276,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":145,"record_offset":24564,"parent_record_offset":24276,"ancestor_record_offset":24276,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":145,"record_offset":24448,"parent_record_offset":24276,"ancestor_record_offset":24276,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] parking_lot_core::parking_lot::deadlock::check_deadlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":145,"record_offset":25320,"parent_record_offset":24276,"ancestor_record_offset":24276,"inlinee":13091,"source_line":1129,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::hang_watchdog::raise_segfault
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/hang_watchdog.rs
//     evidence={"module":145,"record_offset":27492,"parent_record_offset":24276,"ancestor_record_offset":24276,"inlinee":13093,"source_line":98,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":145,"record_offset":25576,"parent_record_offset":25552,"ancestor_record_offset":24276,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":145,"record_offset":25552,"parent_record_offset":25456,"ancestor_record_offset":24276,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":145,"record_offset":26308,"parent_record_offset":26284,"ancestor_record_offset":24276,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":145,"record_offset":25488,"parent_record_offset":25456,"ancestor_record_offset":24276,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":145,"record_offset":25456,"parent_record_offset":25428,"ancestor_record_offset":24276,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":145,"record_offset":26356,"parent_record_offset":26332,"ancestor_record_offset":24276,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":145,"record_offset":26332,"parent_record_offset":26308,"ancestor_record_offset":24276,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":145,"record_offset":25428,"parent_record_offset":25400,"ancestor_record_offset":24276,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":145,"record_offset":26284,"parent_record_offset":26260,"ancestor_record_offset":24276,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":145,"record_offset":26260,"parent_record_offset":26228,"ancestor_record_offset":24276,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":145,"record_offset":28068,"parent_record_offset":28044,"ancestor_record_offset":24276,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":145,"record_offset":26228,"parent_record_offset":26204,"ancestor_record_offset":24276,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$66::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":145,"record_offset":25272,"parent_record_offset":25244,"ancestor_record_offset":24276,"inlinee":5097,"source_line":1820,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":145,"record_offset":25520,"parent_record_offset":25488,"ancestor_record_offset":24276,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":145,"record_offset":24616,"parent_record_offset":24588,"ancestor_record_offset":24276,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":145,"record_offset":24644,"parent_record_offset":24588,"ancestor_record_offset":24276,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":145,"record_offset":25400,"parent_record_offset":25376,"ancestor_record_offset":24276,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":145,"record_offset":25788,"parent_record_offset":25764,"ancestor_record_offset":24276,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":145,"record_offset":26576,"parent_record_offset":26552,"ancestor_record_offset":24276,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":145,"record_offset":25876,"parent_record_offset":25848,"ancestor_record_offset":24276,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":145,"record_offset":27520,"parent_record_offset":27492,"ancestor_record_offset":24276,"inlinee":13094,"source_line":1820,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":145,"record_offset":25764,"parent_record_offset":25740,"ancestor_record_offset":24276,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":145,"record_offset":26552,"parent_record_offset":26528,"ancestor_record_offset":24276,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":145,"record_offset":25848,"parent_record_offset":25824,"ancestor_record_offset":24276,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":145,"record_offset":26528,"parent_record_offset":26464,"ancestor_record_offset":24276,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":145,"record_offset":24472,"parent_record_offset":24448,"ancestor_record_offset":24276,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":145,"record_offset":24496,"parent_record_offset":24472,"ancestor_record_offset":24276,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$21::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":145,"record_offset":25244,"parent_record_offset":25216,"ancestor_record_offset":24276,"inlinee":5095,"source_line":79,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":145,"record_offset":24672,"parent_record_offset":24588,"ancestor_record_offset":24276,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":145,"record_offset":24588,"parent_record_offset":24564,"ancestor_record_offset":24276,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":145,"record_offset":24696,"parent_record_offset":24672,"ancestor_record_offset":24276,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::process::abort
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\process.rs
//     evidence={"module":145,"record_offset":27548,"parent_record_offset":27492,"ancestor_record_offset":24276,"inlinee":7827,"source_line":2378,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::abort_internal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\mod.rs
//     evidence={"module":145,"record_offset":27572,"parent_record_offset":27548,"ancestor_record_offset":24276,"inlinee":7829,"source_line":331,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::time::impl$20::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\time.rs
//     evidence={"module":145,"record_offset":25216,"parent_record_offset":25188,"ancestor_record_offset":24276,"inlinee":5093,"source_line":14,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::time::impl$23::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\time.rs
//     evidence={"module":145,"record_offset":25188,"parent_record_offset":25136,"ancestor_record_offset":24276,"inlinee":5091,"source_line":149,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::hang_watchdog::spawn_hang_watchdog_thread
// 记录形式: compiled_procedure; instances=1
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":145,"record_offset":21716,"rva":3562464,"va":5372271584}
// 编译实例 RVAs: 0x365be0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: hang_watchdog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2809831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372271584,"raw_hex":"e8e7df2a00","size":5,"soft_fail":false,"target_va":5375081488,"va":5372271652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: hang_watchdog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2809624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372271584,"raw_hex":"e818df2a00","size":5,"soft_fail":false,"target_va":5375081504,"va":5372271875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39269021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372271584,"raw_hex":"e89d325702","size":5,"soft_fail":false,"target_va":5411540963,"va":5372271937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t10411707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372271584,"raw_hex":"e8bbde9e00","size":5,"soft_fail":false,"target_va":5382683488,"va":5372271776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::Builder::name
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10419322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372271584,"raw_hex":"e87afc9e00","size":5,"soft_fail":false,"target_va":5382691056,"va":5372271729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::Builder::spawn
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-890659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372271584,"raw_hex":"e8dd68f2ff","size":5,"soft_fail":false,"target_va":5371381088,"va":5372271742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: hang_watchdog.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t2809595","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5372271584,"raw_hex":"e9fbde2a00","size":5,"soft_fail":false,"target_va":5375081504,"va":5372271904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*48416602(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5372271584,"raw_hex":"ff155ac7e202","size":6,"soft_fail":false,"va":5372271760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":145,"record_offset":21892,"parent_record_offset":21716,"ancestor_record_offset":21716,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":145,"record_offset":22420,"parent_record_offset":21716,"ancestor_record_offset":21716,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] std::thread::Builder::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":145,"record_offset":21864,"parent_record_offset":21716,"ancestor_record_offset":21716,"inlinee":12559,"source_line":289,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":145,"record_offset":22264,"parent_record_offset":22240,"ancestor_record_offset":21716,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":145,"record_offset":22288,"parent_record_offset":22264,"ancestor_record_offset":21716,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":145,"record_offset":23636,"parent_record_offset":23612,"ancestor_record_offset":21716,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":145,"record_offset":22240,"parent_record_offset":22208,"ancestor_record_offset":21716,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":145,"record_offset":23612,"parent_record_offset":23528,"ancestor_record_offset":21716,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":145,"record_offset":23528,"parent_record_offset":23500,"ancestor_record_offset":21716,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":145,"record_offset":22140,"parent_record_offset":22108,"ancestor_record_offset":21716,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":145,"record_offset":24160,"parent_record_offset":24136,"ancestor_record_offset":21716,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":145,"record_offset":22208,"parent_record_offset":22172,"ancestor_record_offset":21716,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":145,"record_offset":22172,"parent_record_offset":22140,"ancestor_record_offset":21716,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":145,"record_offset":24136,"parent_record_offset":24112,"ancestor_record_offset":21716,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":145,"record_offset":22068,"parent_record_offset":22036,"ancestor_record_offset":21716,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":145,"record_offset":22036,"parent_record_offset":22004,"ancestor_record_offset":21716,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":145,"record_offset":21972,"parent_record_offset":21924,"ancestor_record_offset":21716,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":145,"record_offset":22004,"parent_record_offset":21972,"ancestor_record_offset":21716,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":145,"record_offset":21924,"parent_record_offset":21892,"ancestor_record_offset":21716,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":145,"record_offset":22888,"parent_record_offset":22852,"ancestor_record_offset":21716,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":145,"record_offset":22688,"parent_record_offset":22664,"ancestor_record_offset":21716,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":145,"record_offset":22108,"parent_record_offset":22068,"ancestor_record_offset":21716,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":145,"record_offset":23556,"parent_record_offset":23528,"ancestor_record_offset":21716,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":145,"record_offset":22364,"parent_record_offset":22340,"ancestor_record_offset":21716,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":145,"record_offset":23580,"parent_record_offset":23556,"ancestor_record_offset":21716,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":145,"record_offset":22340,"parent_record_offset":22068,"ancestor_record_offset":21716,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":145,"record_offset":22448,"parent_record_offset":22420,"ancestor_record_offset":21716,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":145,"record_offset":23328,"parent_record_offset":23300,"ancestor_record_offset":21716,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":145,"record_offset":23376,"parent_record_offset":23352,"ancestor_record_offset":21716,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":145,"record_offset":23352,"parent_record_offset":23328,"ancestor_record_offset":21716,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":145,"record_offset":22920,"parent_record_offset":22888,"ancestor_record_offset":21716,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":145,"record_offset":22724,"parent_record_offset":22688,"ancestor_record_offset":21716,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":145,"record_offset":22748,"parent_record_offset":22724,"ancestor_record_offset":21716,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":145,"record_offset":22780,"parent_record_offset":22688,"ancestor_record_offset":21716,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":145,"record_offset":23300,"parent_record_offset":23272,"ancestor_record_offset":21716,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":145,"record_offset":23272,"parent_record_offset":23244,"ancestor_record_offset":21716,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":145,"record_offset":22576,"parent_record_offset":22552,"ancestor_record_offset":21716,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::hang_watchdog::raise_segfault
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 98
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3325
// 首条位置: {"module":145,"inlinee":13093}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::write_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":145,"record_offset":27520,"parent_record_offset":27492,"ancestor_record_offset":27492,"inlinee":13094,"source_line":1820,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] std::process::abort
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\process.rs
//     evidence={"module":145,"record_offset":27548,"parent_record_offset":27492,"ancestor_record_offset":27492,"inlinee":7827,"source_line":2378,"archive":"_pdb/B/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::abort_internal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\mod.rs
//     evidence={"module":145,"record_offset":27572,"parent_record_offset":27548,"ancestor_record_offset":27492,"inlinee":7829,"source_line":331,"archive":"_pdb/B/modules/00145.jsonl.gz"}
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
// 完整结构及原始类型记录: hang_watchdog.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: hang_watchdog.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: hang_watchdog.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)()
// 候选参数类型: []

// ------------------------------------------------------------------
// 函数: tiny_glade::hang_watchdog::hang_watchdog
// 信息 ID: c9de60865429155a4a29b47e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 48212e4505979347661a8a0d: void (*)()
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<void (*)(),tuple$<> > @ 0xd9934 -> 0x365e20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::hang_watchdog::raise_segfault
// 信息 ID: c4f985c00226d40235e9a8c7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::hang_watchdog::hang_watchdog; module=145, inline_record=27492; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::hang_watchdog::hang_watchdog; module=145, inline_record=27928; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::hang_watchdog::spawn_hang_watchdog_thread
// 信息 ID: 9187cff9b72e747361101da0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x5a4097 -> 0x365be0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
