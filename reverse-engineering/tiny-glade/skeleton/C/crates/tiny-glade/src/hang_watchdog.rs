// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/hang_watchdog.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 63fe31cc7037322a205eaf6e85f41ca0a21fa06c47ad750d1459dadd77311551
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: hang_watchdog.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 109
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
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.12"}]
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
// 首条位置: {"module":109,"record_offset":640,"rva":3797760,"va":5372506880}
// 编译实例 RVAs: 0x39f300
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::time::Instant as core::ops::arith::Add<core::time::Duration>>::add
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t12261588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372506880,"raw_hex":"e8d418bb00","size":5,"soft_fail":false,"target_va":5384768864,"va":5372507271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::now
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t12140773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372506880,"raw_hex":"e8e540b900","size":5,"soft_fail":false,"target_va":5384648032,"va":5372507254,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t12102164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372506880,"raw_hex":"e814aab800","size":5,"soft_fail":false,"target_va":5384609408,"va":5372507239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t3171223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372506880,"raw_hex":"e897633000","size":5,"soft_fail":false,"target_va":5375678704,"va":5372507476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400814a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hang_watchdog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3269852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372506880,"raw_hex":"e8241bceff","size":5,"soft_fail":false,"target_va":5369238688,"va":5372508535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140523d10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hang_watchdog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1590260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372506880,"raw_hex":"e8f4431800","size":5,"soft_fail":false,"target_va":5374098704,"va":5372508439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14071e1d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hang_watchdog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3665568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372506880,"raw_hex":"e8a0ee3700","size":5,"soft_fail":false,"target_va":5376172496,"va":5372506923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::check_deadlock
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11472689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372506880,"raw_hex":"e8310faf00","size":5,"soft_fail":false,"target_va":5383980080,"va":5372507386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::functions::sleep
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t12220849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372506880,"raw_hex":"e8b179ba00","size":5,"soft_fail":false,"target_va":5384728160,"va":5372507306,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":109,"record_offset":1560,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":109,"record_offset":2564,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":109,"record_offset":892,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":109,"record_offset":1364,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":6450,"source_line":1410,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":109,"record_offset":2228,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":109,"record_offset":1588,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":109,"record_offset":2144,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":109,"record_offset":2676,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":109,"record_offset":948,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":109,"record_offset":808,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_direct] parking_lot_core::parking_lot::deadlock::check_deadlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":109,"record_offset":1532,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":12433,"source_line":1137,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::hang_watchdog::raise_segfault
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/hang_watchdog.rs
//     evidence={"module":109,"record_offset":3524,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":12435,"source_line":98,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":109,"record_offset":1788,"parent_record_offset":1764,"ancestor_record_offset":640,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":109,"record_offset":1812,"parent_record_offset":1788,"ancestor_record_offset":640,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":109,"record_offset":1836,"parent_record_offset":1812,"ancestor_record_offset":640,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":109,"record_offset":1764,"parent_record_offset":1668,"ancestor_record_offset":640,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":109,"record_offset":1700,"parent_record_offset":1668,"ancestor_record_offset":640,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":109,"record_offset":1668,"parent_record_offset":1640,"ancestor_record_offset":640,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":109,"record_offset":1640,"parent_record_offset":1612,"ancestor_record_offset":640,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":109,"record_offset":3920,"parent_record_offset":3896,"ancestor_record_offset":640,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":109,"record_offset":1732,"parent_record_offset":1700,"ancestor_record_offset":640,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":109,"record_offset":916,"parent_record_offset":892,"ancestor_record_offset":640,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":109,"record_offset":1612,"parent_record_offset":1588,"ancestor_record_offset":640,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":109,"record_offset":2192,"parent_record_offset":2168,"ancestor_record_offset":640,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":109,"record_offset":2736,"parent_record_offset":2676,"ancestor_record_offset":640,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":109,"record_offset":2276,"parent_record_offset":2252,"ancestor_record_offset":640,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":109,"record_offset":3552,"parent_record_offset":3524,"ancestor_record_offset":640,"inlinee":12436,"source_line":2167,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":109,"record_offset":2588,"parent_record_offset":2564,"ancestor_record_offset":640,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":109,"record_offset":2168,"parent_record_offset":2144,"ancestor_record_offset":640,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":109,"record_offset":2252,"parent_record_offset":2228,"ancestor_record_offset":640,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":109,"record_offset":832,"parent_record_offset":808,"ancestor_record_offset":640,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":109,"record_offset":856,"parent_record_offset":832,"ancestor_record_offset":640,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$21::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":109,"record_offset":1488,"parent_record_offset":1460,"ancestor_record_offset":640,"inlinee":6433,"source_line":79,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":109,"record_offset":996,"parent_record_offset":972,"ancestor_record_offset":640,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":109,"record_offset":972,"parent_record_offset":948,"ancestor_record_offset":640,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":109,"record_offset":1020,"parent_record_offset":996,"ancestor_record_offset":640,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] std::process::abort
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\process.rs
//     evidence={"module":109,"record_offset":3580,"parent_record_offset":3524,"ancestor_record_offset":640,"inlinee":4946,"source_line":2532,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::abort_internal
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\pal\windows\mod.rs
//     evidence={"module":109,"record_offset":3604,"parent_record_offset":3580,"ancestor_record_offset":640,"inlinee":4948,"source_line":266,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::time::windows::impl$20::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\time\windows.rs
//     evidence={"module":109,"record_offset":1460,"parent_record_offset":1432,"ancestor_record_offset":640,"inlinee":6431,"source_line":13,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] std::time::impl$24::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\time.rs
//     evidence={"module":109,"record_offset":1432,"parent_record_offset":1364,"ancestor_record_offset":640,"inlinee":6429,"source_line":154,"archive":"_pdb/C/modules/00109.jsonl.gz"}
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
// 首条位置: {"module":109,"record_offset":4244,"rva":3799488,"va":5372508608}
// 编译实例 RVAs: 0x39f9c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::sync::Arc<std::thread::lifecycle::Packet<()>>>::drop_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t12194703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372508608,"raw_hex":"e88f13ba00","size":5,"soft_fail":false,"target_va":5384703584,"va":5372508876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <alloc::sync::Arc<std::thread::thread::Inner, std::alloc::System>>::drop_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t12195043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372508608,"raw_hex":"e8e314ba00","size":5,"soft_fail":false,"target_va":5384703904,"va":5372508856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::thread::builder::Builder>::name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t12140540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372508608,"raw_hex":"e8fc3fb900","size":5,"soft_fail":false,"target_va":5384649296,"va":5372508751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3170009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372508608,"raw_hex":"e8d95e3000","size":5,"soft_fail":false,"target_va":5375678688,"va":5372508674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3169663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372508608,"raw_hex":"e87f5d3000","size":5,"soft_fail":false,"target_va":5375678704,"va":5372509036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: hang_watchdog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3786472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372508608,"raw_hex":"e81839c6ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5372508659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: hang_watchdog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3170065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372508608,"raw_hex":"e8efa0cfff","size":5,"soft_fail":false,"target_va":5369338848,"va":5372508908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t44111855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372508608,"raw_hex":"e8ef17a102","size":5,"soft_fail":false,"target_va":5416620794,"va":5372508934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::lifecycle::spawn_unchecked
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t857989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372508608,"raw_hex":"e885170d00","size":5,"soft_fail":false,"target_va":5373366800,"va":5372508806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*53652330(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5372508608,"raw_hex":"ff156aab3203","size":6,"soft_fail":false,"va":5372508840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":109,"record_offset":4424,"parent_record_offset":4244,"ancestor_record_offset":4244,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":109,"record_offset":5040,"parent_record_offset":4244,"ancestor_record_offset":4244,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_direct] std::thread::builder::Builder::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":109,"record_offset":4396,"parent_record_offset":4244,"ancestor_record_offset":4244,"inlinee":7549,"source_line":78,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_direct] std::thread::builder::Builder::spawn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":109,"record_offset":4940,"parent_record_offset":4244,"ancestor_record_offset":4244,"inlinee":7491,"source_line":185,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":109,"record_offset":4760,"parent_record_offset":4736,"ancestor_record_offset":4244,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":109,"record_offset":4784,"parent_record_offset":4760,"ancestor_record_offset":4244,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":109,"record_offset":6032,"parent_record_offset":6008,"ancestor_record_offset":4244,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":109,"record_offset":6056,"parent_record_offset":6032,"ancestor_record_offset":4244,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":109,"record_offset":4808,"parent_record_offset":4784,"ancestor_record_offset":4244,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":109,"record_offset":6080,"parent_record_offset":6056,"ancestor_record_offset":4244,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":109,"record_offset":4736,"parent_record_offset":4704,"ancestor_record_offset":4244,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":109,"record_offset":6008,"parent_record_offset":5980,"ancestor_record_offset":4244,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":109,"record_offset":4636,"parent_record_offset":4604,"ancestor_record_offset":4244,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":109,"record_offset":5980,"parent_record_offset":5956,"ancestor_record_offset":4244,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":109,"record_offset":4704,"parent_record_offset":4668,"ancestor_record_offset":4244,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":109,"record_offset":4668,"parent_record_offset":4636,"ancestor_record_offset":4244,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":109,"record_offset":5956,"parent_record_offset":5932,"ancestor_record_offset":4244,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":109,"record_offset":4500,"parent_record_offset":4456,"ancestor_record_offset":4244,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":109,"record_offset":4532,"parent_record_offset":4500,"ancestor_record_offset":4244,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":109,"record_offset":4564,"parent_record_offset":4532,"ancestor_record_offset":4244,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":109,"record_offset":4456,"parent_record_offset":4424,"ancestor_record_offset":4244,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":109,"record_offset":5508,"parent_record_offset":5472,"ancestor_record_offset":4244,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":109,"record_offset":5312,"parent_record_offset":5288,"ancestor_record_offset":4244,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":109,"record_offset":4604,"parent_record_offset":4564,"ancestor_record_offset":4244,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":109,"record_offset":4864,"parent_record_offset":4564,"ancestor_record_offset":4244,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":109,"record_offset":4888,"parent_record_offset":4864,"ancestor_record_offset":4244,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":109,"record_offset":5072,"parent_record_offset":5040,"ancestor_record_offset":4244,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":109,"record_offset":5540,"parent_record_offset":5508,"ancestor_record_offset":4244,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":109,"record_offset":5000,"parent_record_offset":4968,"ancestor_record_offset":4244,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":109,"record_offset":5348,"parent_record_offset":5312,"ancestor_record_offset":4244,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":109,"record_offset":5372,"parent_record_offset":5348,"ancestor_record_offset":4244,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":109,"record_offset":5404,"parent_record_offset":5312,"ancestor_record_offset":4244,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":109,"record_offset":5200,"parent_record_offset":5176,"ancestor_record_offset":4244,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::builder::Builder::spawn_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":109,"record_offset":4968,"parent_record_offset":4940,"ancestor_record_offset":4244,"inlinee":7492,"source_line":254,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::hang_watchdog::raise_segfault
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 98
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3093
// 首条位置: {"module":109,"inlinee":12435}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::write_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":109,"record_offset":3552,"parent_record_offset":3524,"ancestor_record_offset":3524,"inlinee":12436,"source_line":2167,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_direct] std::process::abort
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\process.rs
//     evidence={"module":109,"record_offset":3580,"parent_record_offset":3524,"ancestor_record_offset":3524,"inlinee":4946,"source_line":2532,"archive":"_pdb/C/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::abort_internal
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\pal\windows\mod.rs
//     evidence={"module":109,"record_offset":3604,"parent_record_offset":3580,"ancestor_record_offset":3524,"inlinee":4948,"source_line":266,"archive":"_pdb/C/modules/00109.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::system_param::Local<system_wall_constructor::ui_arch_controls::ArchHeightControl>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::system_param::Local<system_wall_constructor::ui_arch_controls::ArchHeightControl>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_wall_constructor::ui_arch_controls::ArchHeightControl>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::defered_autoclutter_history::RecordAutoClutterHistory>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::defered_autoclutter_history::RecordAutoClutterHistory>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::defered_autoclutter_history::RecordAutoClutterHistory>
// [文件级接口候选，具体函数归属 unknown] void (*)()
// 候选参数类型: []

// ------------------------------------------------------------------
// 函数: tiny_glade::hang_watchdog::hang_watchdog
// 信息 ID: 3bfe9ef64ac6c3a8097600da
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 48212e4505979347661a8a0d: void (*)()
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<void (*)(),tuple$<> > @ 0x5e6cf4 -> 0x39f300 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::hang_watchdog::raise_segfault
// 信息 ID: ebe1ed82853a7ba387d08cc0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::hang_watchdog::hang_watchdog; module=109, inline_record=3524; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::hang_watchdog::hang_watchdog; module=109, inline_record=3780; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::hang_watchdog::spawn_hang_watchdog_thread
// 信息 ID: f900a916f1742140be5a98f8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x4233af -> 0x39f9c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
