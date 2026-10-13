// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/utils/debug_viz.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: a61516ad1a665052a5880ce96271211648aa7313dfdaf3610d2ceaa0646c44d6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: debug_viz.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 503, 510, 541, 550, 551, 553, 569, 570, 574, 577, 578, 582, 583, 590, 1681, 1745, 1762
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
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::drain_global_debug_spheres
// 记录形式: compiled_procedure; instances=1
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":541,"record_offset":10032,"rva":10968160,"va":5379677280}
// 编译实例 RVAs: 0xa75c60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::lock_contended
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t36937939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379677280,"raw_hex":"e8d3a03302","size":5,"soft_fail":false,"target_va":5416616112,"va":5379678168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::wake
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t36938028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379677280,"raw_hex":"e82ca13302","size":5,"soft_fail":false,"target_va":5416616304,"va":5379678271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-3998931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379677280,"raw_hex":"e82dfbc2ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379677630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140087be0
//     owner=unresolved_namespace; count=2
//     target_aliases=122 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10413765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379677280,"raw_hex":"e83b1961ff","size":5,"soft_fail":false,"target_va":5369265120,"va":5379678880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140904120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1514613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379677280,"raw_hex":"e88be3e8ff","size":5,"soft_fail":false,"target_va":5378162976,"va":5379677584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140906710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1505257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379677280,"raw_hex":"e81708e9ff","size":5,"soft_fail":false,"target_va":5378172688,"va":5379677940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096cd00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1086629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379677280,"raw_hex":"e85b6befff","size":5,"soft_fail":false,"target_va":5378592000,"va":5379678624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a26c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-867115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379677280,"raw_hex":"e8d5c4f2ff","size":5,"soft_fail":false,"target_va":5378811584,"va":5379678694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd5d7e
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37092799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379677280,"raw_hex":"e8bffd3502","size":5,"soft_fail":false,"target_va":5416770942,"va":5379678138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd5e61
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37092871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379677280,"raw_hex":"e807fe3502","size":5,"soft_fail":false,"target_va":5416771169,"va":5379678293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36943378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379677280,"raw_hex":"e812b63302","size":5,"soft_fail":false,"target_va":5416621632,"va":5379678249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t36941513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379677280,"raw_hex":"e8c9ae3302","size":5,"soft_fail":false,"target_va":5416619696,"va":5379678178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":541,"record_offset":13180,"parent_record_offset":10032,"ancestor_record_offset":10032,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":541,"record_offset":11436,"parent_record_offset":10032,"ancestor_record_offset":10032,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":541,"record_offset":10956,"parent_record_offset":10032,"ancestor_record_offset":10032,"inlinee":4795,"source_line":820,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":541,"record_offset":11016,"parent_record_offset":10032,"ancestor_record_offset":10032,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":541,"record_offset":10912,"parent_record_offset":10032,"ancestor_record_offset":10032,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugLine::show
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":541,"record_offset":13364,"parent_record_offset":10032,"ancestor_record_offset":10032,"inlinee":28669,"source_line":446,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugSphere::show
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":541,"record_offset":11564,"parent_record_offset":10032,"ancestor_record_offset":10032,"inlinee":28664,"source_line":141,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] std::sync::once_lock::OnceLock::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":541,"record_offset":10208,"parent_record_offset":10032,"ancestor_record_offset":10032,"inlinee":14249,"source_line":317,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":541,"record_offset":10444,"parent_record_offset":10032,"ancestor_record_offset":10032,"inlinee":4978,"source_line":490,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":11848,"parent_record_offset":11824,"ancestor_record_offset":10032,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":11872,"parent_record_offset":11848,"ancestor_record_offset":10032,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":11896,"parent_record_offset":11872,"ancestor_record_offset":10032,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":11824,"parent_record_offset":11736,"ancestor_record_offset":10032,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":11764,"parent_record_offset":11736,"ancestor_record_offset":10032,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":11736,"parent_record_offset":11712,"ancestor_record_offset":10032,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":11712,"parent_record_offset":11688,"ancestor_record_offset":10032,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":541,"record_offset":14144,"parent_record_offset":13416,"ancestor_record_offset":10032,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":541,"record_offset":11616,"parent_record_offset":11592,"ancestor_record_offset":10032,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":541,"record_offset":11664,"parent_record_offset":11640,"ancestor_record_offset":10032,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":541,"record_offset":14216,"parent_record_offset":14192,"ancestor_record_offset":10032,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":541,"record_offset":14192,"parent_record_offset":14168,"ancestor_record_offset":10032,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":541,"record_offset":14168,"parent_record_offset":14144,"ancestor_record_offset":10032,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":541,"record_offset":10980,"parent_record_offset":10956,"ancestor_record_offset":10032,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":541,"record_offset":11792,"parent_record_offset":11764,"ancestor_record_offset":10032,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":541,"record_offset":14288,"parent_record_offset":14264,"ancestor_record_offset":10032,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":541,"record_offset":11640,"parent_record_offset":11616,"ancestor_record_offset":10032,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":541,"record_offset":13204,"parent_record_offset":13180,"ancestor_record_offset":10032,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":541,"record_offset":14264,"parent_record_offset":14240,"ancestor_record_offset":10032,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":541,"record_offset":14240,"parent_record_offset":14216,"ancestor_record_offset":10032,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":541,"record_offset":11504,"parent_record_offset":11436,"ancestor_record_offset":10032,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":541,"record_offset":11472,"parent_record_offset":11436,"ancestor_record_offset":10032,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":541,"record_offset":11528,"parent_record_offset":11504,"ancestor_record_offset":10032,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":10832,"parent_record_offset":10804,"ancestor_record_offset":10032,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":11276,"parent_record_offset":11088,"ancestor_record_offset":10032,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":10364,"parent_record_offset":10340,"ancestor_record_offset":10032,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":10516,"parent_record_offset":10484,"ancestor_record_offset":10032,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":11368,"parent_record_offset":11336,"ancestor_record_offset":10032,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":10736,"parent_record_offset":10704,"ancestor_record_offset":10032,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":10544,"parent_record_offset":10516,"ancestor_record_offset":10032,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":10388,"parent_record_offset":10364,"ancestor_record_offset":10032,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":11300,"parent_record_offset":11276,"ancestor_record_offset":10032,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":11392,"parent_record_offset":11368,"ancestor_record_offset":10032,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":541,"record_offset":10704,"parent_record_offset":10676,"ancestor_record_offset":10032,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":541,"record_offset":10676,"parent_record_offset":10648,"ancestor_record_offset":10032,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":541,"record_offset":10316,"parent_record_offset":10292,"ancestor_record_offset":10032,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":541,"record_offset":10268,"parent_record_offset":10236,"ancestor_record_offset":10032,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get_or_try_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":541,"record_offset":10236,"parent_record_offset":10208,"ancestor_record_offset":10032,"inlinee":14250,"source_line":398,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":541,"record_offset":10292,"parent_record_offset":10268,"ancestor_record_offset":10032,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":541,"record_offset":11088,"parent_record_offset":11048,"ancestor_record_offset":10032,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":541,"record_offset":10804,"parent_record_offset":10612,"ancestor_record_offset":10032,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":541,"record_offset":10612,"parent_record_offset":10584,"ancestor_record_offset":10032,"inlinee":4982,"source_line":118,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":541,"record_offset":10584,"parent_record_offset":10444,"ancestor_record_offset":10032,"inlinee":4981,"source_line":719,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":541,"record_offset":11048,"parent_record_offset":11016,"ancestor_record_offset":10032,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":541,"record_offset":10484,"parent_record_offset":10444,"ancestor_record_offset":10032,"inlinee":4979,"source_line":31,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":541,"record_offset":11336,"parent_record_offset":11048,"ancestor_record_offset":10032,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":541,"record_offset":10340,"parent_record_offset":10316,"ancestor_record_offset":10032,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":541,"record_offset":10648,"parent_record_offset":10612,"ancestor_record_offset":10032,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::impl$6::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 99
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":120528,"rva":9486928,"va":5378196048}
// 编译实例 RVAs: 0x90c250
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::fmt::Formatter>::debug_struct_field5_finish
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6625882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378196048,"raw_hex":"e85a1a6500","size":5,"soft_fail":false,"target_va":5384822192,"va":5378196305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugSphere::new
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 109
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x82B9
// 首条位置: {"module":541,"record_offset":4024,"rva":10966064,"va":5379675184}
// 编译实例 RVAs: 0xa75430
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugSphere::color
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 119
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x82BA
// 首条位置: {"module":541,"record_offset":4276,"rva":10966128,"va":5379675248}
// 编译实例 RVAs: 0xa75470
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugSphere::radius
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=14
// 源码行: 125
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6FF7
// 首条位置: {"module":503,"inlinee":28663}
// 编译实例 RVAs: 0xa754b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugSphere::solid
// 记录形式: compiled_procedure; instances=1
// 源码行: 130
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":541,"record_offset":4404,"rva":10966160,"va":5379675280}
// 编译实例 RVAs: 0xa75490
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugSphere::line_width
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x82B7
// 首条位置: {"module":577,"inlinee":33463}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugSphere::show
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=21
// 源码行: 141, 142
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6FF8
// 首条位置: {"module":503,"inlinee":28664}
// 编译实例 RVAs: 0xa75460
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140904120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-1512261","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379675232,"raw_hex":"e9bbece8ff","size":5,"soft_fail":false,"target_va":5378162976,"va":5379675232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::impl$1::show::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 142
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6FBF
// 首条位置: {"module":503,"inlinee":28607}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":503,"record_offset":28916,"parent_record_offset":28892,"ancestor_record_offset":28892,"inlinee":28609,"source_line":61,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugSphere::show_for
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 146, 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x82BC
// 首条位置: {"module":541,"record_offset":4660,"rva":10966224,"va":5379675344}
// 编译实例 RVAs: 0xa754d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x1409043b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-1511717","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379675344,"raw_hex":"e9dbeee8ff","size":5,"soft_fail":false,"target_va":5378163632,"va":5379675344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::impl$1::show_for::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6FC3
// 首条位置: {"module":503,"inlinee":28611}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::ui::signifiers::signifier_create::OwnedSignifierStream::write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":503,"record_offset":30864,"parent_record_offset":30840,"ancestor_record_offset":30840,"inlinee":28613,"source_line":95,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugSphere::emit<country_core::utils::debug_viz::impl$1::show::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 150
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":27292,"rva":9453856,"va":5378162976}
// 编译实例 RVAs: 0x904120
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2484456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378162976,"raw_hex":"e81817daff","size":5,"soft_fail":false,"target_va":5375678688,"va":5378163139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2484625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378162976,"raw_hex":"e86f16daff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378163324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9440937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378162976,"raw_hex":"e857f16fff","size":5,"soft_fail":false,"target_va":5368722192,"va":5378163124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c922e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3727191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378162976,"raw_hex":"e857df3800","size":5,"soft_fail":false,"target_va":5381890784,"va":5378163588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418055f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15733579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378162976,"raw_hex":"e84b13f000","size":5,"soft_fail":false,"target_va":5393896944,"va":5378163360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38893750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378162976,"raw_hex":"e8b6785102","size":5,"soft_fail":false,"target_va":5417057056,"va":5378163301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38457344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378162976,"raw_hex":"e800d04a02","size":5,"soft_fail":false,"target_va":5416620769,"va":5378163420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6631753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378162976,"raw_hex":"e849316500","size":5,"soft_fail":false,"target_va":5384794864,"va":5378163106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38457546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378162976,"raw_hex":"e8cad04a02","size":5,"soft_fail":false,"target_va":5416620794,"va":5378163243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":503,"record_offset":27504,"parent_record_offset":27292,"ancestor_record_offset":27292,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":503,"record_offset":27584,"parent_record_offset":27292,"ancestor_record_offset":27292,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":28584,"parent_record_offset":27292,"ancestor_record_offset":27292,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":503,"record_offset":27808,"parent_record_offset":27292,"ancestor_record_offset":27292,"inlinee":28605,"source_line":191,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::impl$1::show::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":503,"record_offset":28892,"parent_record_offset":27292,"ancestor_record_offset":27292,"inlinee":28607,"source_line":142,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":27704,"parent_record_offset":27680,"ancestor_record_offset":27292,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":27728,"parent_record_offset":27704,"ancestor_record_offset":27292,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":28784,"parent_record_offset":28760,"ancestor_record_offset":27292,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":28808,"parent_record_offset":28784,"ancestor_record_offset":27292,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":27752,"parent_record_offset":27728,"ancestor_record_offset":27292,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":28832,"parent_record_offset":28808,"ancestor_record_offset":27292,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":27680,"parent_record_offset":27648,"ancestor_record_offset":27292,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":28760,"parent_record_offset":28696,"ancestor_record_offset":27292,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":27648,"parent_record_offset":27612,"ancestor_record_offset":27292,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":27612,"parent_record_offset":27584,"ancestor_record_offset":27292,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":503,"record_offset":27552,"parent_record_offset":27528,"ancestor_record_offset":27292,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":28012,"parent_record_offset":27988,"ancestor_record_offset":27292,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":28728,"parent_record_offset":28696,"ancestor_record_offset":27292,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":28696,"parent_record_offset":28668,"ancestor_record_offset":27292,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":28068,"parent_record_offset":28036,"ancestor_record_offset":27292,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":28036,"parent_record_offset":28012,"ancestor_record_offset":27292,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":28668,"parent_record_offset":28640,"ancestor_record_offset":27292,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":28108,"parent_record_offset":28068,"ancestor_record_offset":27292,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":27912,"parent_record_offset":27888,"ancestor_record_offset":27292,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":27936,"parent_record_offset":27912,"ancestor_record_offset":27292,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":27960,"parent_record_offset":27936,"ancestor_record_offset":27292,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":27888,"parent_record_offset":27864,"ancestor_record_offset":27292,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":503,"record_offset":27864,"parent_record_offset":27840,"ancestor_record_offset":27292,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":503,"record_offset":27840,"parent_record_offset":27808,"ancestor_record_offset":27292,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":503,"record_offset":28452,"parent_record_offset":28428,"ancestor_record_offset":27292,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":27988,"parent_record_offset":27960,"ancestor_record_offset":27292,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":503,"record_offset":28132,"parent_record_offset":28108,"ancestor_record_offset":27292,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":503,"record_offset":28180,"parent_record_offset":28156,"ancestor_record_offset":27292,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":503,"record_offset":28156,"parent_record_offset":28132,"ancestor_record_offset":27292,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":503,"record_offset":27528,"parent_record_offset":27504,"ancestor_record_offset":27292,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":503,"record_offset":28348,"parent_record_offset":27960,"ancestor_record_offset":27292,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":28372,"parent_record_offset":28348,"ancestor_record_offset":27292,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":28428,"parent_record_offset":27808,"ancestor_record_offset":27292,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":28488,"parent_record_offset":28452,"ancestor_record_offset":27292,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":28512,"parent_record_offset":28488,"ancestor_record_offset":27292,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":28544,"parent_record_offset":28452,"ancestor_record_offset":27292,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":503,"record_offset":28916,"parent_record_offset":28892,"ancestor_record_offset":27292,"inlinee":28609,"source_line":61,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugSphere::emit<country_core::utils::debug_viz::impl$1::show_for::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 150
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":29236,"rva":9454512,"va":5378163632}
// 编译实例 RVAs: 0x9043b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2485112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378163632,"raw_hex":"e88814daff","size":5,"soft_fail":false,"target_va":5375678688,"va":5378163795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2485281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378163632,"raw_hex":"e8df13daff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378163980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9441593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378163632,"raw_hex":"e8c7ee6fff","size":5,"soft_fail":false,"target_va":5368722192,"va":5378163780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c922e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3726535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378163632,"raw_hex":"e8c7dc3800","size":5,"soft_fail":false,"target_va":5381890784,"va":5378164244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418055f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15732912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378163632,"raw_hex":"e8b010f000","size":5,"soft_fail":false,"target_va":5393896944,"va":5378164027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38893094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378163632,"raw_hex":"e826765102","size":5,"soft_fail":false,"target_va":5417057056,"va":5378163957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38456677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378163632,"raw_hex":"e865cd4a02","size":5,"soft_fail":false,"target_va":5416620769,"va":5378164087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6631097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378163632,"raw_hex":"e8b92e6500","size":5,"soft_fail":false,"target_va":5384794864,"va":5378163762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38456890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378163632,"raw_hex":"e83ace4a02","size":5,"soft_fail":false,"target_va":5416620794,"va":5378163899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":503,"record_offset":29452,"parent_record_offset":29236,"ancestor_record_offset":29236,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":503,"record_offset":29532,"parent_record_offset":29236,"ancestor_record_offset":29236,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":30532,"parent_record_offset":29236,"ancestor_record_offset":29236,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":503,"record_offset":29756,"parent_record_offset":29236,"ancestor_record_offset":29236,"inlinee":28605,"source_line":191,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::impl$1::show_for::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":503,"record_offset":30840,"parent_record_offset":29236,"ancestor_record_offset":29236,"inlinee":28611,"source_line":147,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":29652,"parent_record_offset":29628,"ancestor_record_offset":29236,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":29676,"parent_record_offset":29652,"ancestor_record_offset":29236,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":30732,"parent_record_offset":30708,"ancestor_record_offset":29236,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":30756,"parent_record_offset":30732,"ancestor_record_offset":29236,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":29700,"parent_record_offset":29676,"ancestor_record_offset":29236,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":30780,"parent_record_offset":30756,"ancestor_record_offset":29236,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":29628,"parent_record_offset":29596,"ancestor_record_offset":29236,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":30708,"parent_record_offset":30644,"ancestor_record_offset":29236,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":29596,"parent_record_offset":29560,"ancestor_record_offset":29236,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":29560,"parent_record_offset":29532,"ancestor_record_offset":29236,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":503,"record_offset":29500,"parent_record_offset":29476,"ancestor_record_offset":29236,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":29960,"parent_record_offset":29936,"ancestor_record_offset":29236,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":30676,"parent_record_offset":30644,"ancestor_record_offset":29236,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":30644,"parent_record_offset":30616,"ancestor_record_offset":29236,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":30016,"parent_record_offset":29984,"ancestor_record_offset":29236,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":29984,"parent_record_offset":29960,"ancestor_record_offset":29236,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":30616,"parent_record_offset":30588,"ancestor_record_offset":29236,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":30056,"parent_record_offset":30016,"ancestor_record_offset":29236,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":29860,"parent_record_offset":29836,"ancestor_record_offset":29236,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":29884,"parent_record_offset":29860,"ancestor_record_offset":29236,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":29908,"parent_record_offset":29884,"ancestor_record_offset":29236,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":29836,"parent_record_offset":29812,"ancestor_record_offset":29236,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":503,"record_offset":29812,"parent_record_offset":29788,"ancestor_record_offset":29236,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":503,"record_offset":29788,"parent_record_offset":29756,"ancestor_record_offset":29236,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":503,"record_offset":30400,"parent_record_offset":30376,"ancestor_record_offset":29236,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":29936,"parent_record_offset":29908,"ancestor_record_offset":29236,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":503,"record_offset":30080,"parent_record_offset":30056,"ancestor_record_offset":29236,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":503,"record_offset":30128,"parent_record_offset":30104,"ancestor_record_offset":29236,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":503,"record_offset":30104,"parent_record_offset":30080,"ancestor_record_offset":29236,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":503,"record_offset":29476,"parent_record_offset":29452,"ancestor_record_offset":29236,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":503,"record_offset":30296,"parent_record_offset":29908,"ancestor_record_offset":29236,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":30320,"parent_record_offset":30296,"ancestor_record_offset":29236,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":30376,"parent_record_offset":29756,"ancestor_record_offset":29236,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":30436,"parent_record_offset":30400,"ancestor_record_offset":29236,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":30460,"parent_record_offset":30436,"ancestor_record_offset":29236,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":30492,"parent_record_offset":30400,"ancestor_record_offset":29236,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::signifiers::signifier_create::OwnedSignifierStream::write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":503,"record_offset":30864,"parent_record_offset":30840,"ancestor_record_offset":29236,"inlinee":28613,"source_line":95,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::impl$1::emit::closure$0<country_core::utils::debug_viz::impl$1::show::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 167
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":31184,"rva":9455168,"va":5378164288}
// 编译实例 RVAs: 0x904640
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2486069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378164288,"raw_hex":"e8cb10daff","size":5,"soft_fail":false,"target_va":5375678688,"va":5378164752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2486883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378164288,"raw_hex":"e89d0ddaff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378165582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9442547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378164288,"raw_hex":"e80deb6fff","size":5,"soft_fail":false,"target_va":5368722192,"va":5378164734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a1390
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t640942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378164288,"raw_hex":"e8aec70900","size":5,"soft_fail":false,"target_va":5378806672,"va":5378165725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f4de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14615260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378164288,"raw_hex":"e8dc02df00","size":5,"soft_fail":false,"target_va":5392780768,"va":5378165503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38219753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378164288,"raw_hex":"e8e92f4702","size":5,"soft_fail":false,"target_va":5416385024,"va":5378165266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38892277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378164288,"raw_hex":"e8f5725102","size":5,"soft_fail":false,"target_va":5417057056,"va":5378164774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38455523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378164288,"raw_hex":"e8e3c84a02","size":5,"soft_fail":false,"target_va":5416620769,"va":5378165241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38456074","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378164288,"raw_hex":"e80acb4a02","size":5,"soft_fail":false,"target_va":5416620794,"va":5378164715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38456147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378164288,"raw_hex":"e853cb4a02","size":5,"soft_fail":false,"target_va":5416621408,"va":5378165256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38456028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378164288,"raw_hex":"e8dcca4a02","size":5,"soft_fail":false,"target_va":5416621376,"va":5378165343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t773625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378164288,"raw_hex":"e8f9cd0b00","size":5,"soft_fail":false,"target_va":5378938736,"va":5378165106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5799137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378164288,"raw_hex":"e8e17c5800","size":5,"soft_fail":false,"target_va":5383963792,"va":5378164650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5807651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378164288,"raw_hex":"e8239e5800","size":5,"soft_fail":false,"target_va":5383972224,"va":5378164568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5799715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378164288,"raw_hex":"e8237f5800","size":5,"soft_fail":false,"target_va":5383964912,"va":5378165192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5798469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378164288,"raw_hex":"e8457a5800","size":5,"soft_fail":false,"target_va":5383962816,"va":5378164342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38454047","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378164288,"raw_hex":"e81fc34a02","size":5,"soft_fail":false,"target_va":5416619360,"va":5378165308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":34052,"parent_record_offset":31184,"ancestor_record_offset":31184,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":33852,"parent_record_offset":31184,"ancestor_record_offset":31184,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":503,"record_offset":33208,"parent_record_offset":31184,"ancestor_record_offset":31184,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":34136,"parent_record_offset":31184,"ancestor_record_offset":31184,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::scale
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":503,"record_offset":33824,"parent_record_offset":31184,"ancestor_record_offset":31184,"inlinee":28615,"source_line":306,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":503,"record_offset":32412,"parent_record_offset":31184,"ancestor_record_offset":31184,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":503,"record_offset":31408,"parent_record_offset":31184,"ancestor_record_offset":31184,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":33636,"parent_record_offset":33612,"ancestor_record_offset":31184,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":33660,"parent_record_offset":33636,"ancestor_record_offset":31184,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":35188,"parent_record_offset":35164,"ancestor_record_offset":31184,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":35212,"parent_record_offset":35188,"ancestor_record_offset":31184,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":33684,"parent_record_offset":33660,"ancestor_record_offset":31184,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":35236,"parent_record_offset":35212,"ancestor_record_offset":31184,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":33612,"parent_record_offset":33460,"ancestor_record_offset":31184,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":35164,"parent_record_offset":35140,"ancestor_record_offset":31184,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":33888,"parent_record_offset":33852,"ancestor_record_offset":31184,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":35140,"parent_record_offset":35116,"ancestor_record_offset":31184,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":33404,"parent_record_offset":33380,"ancestor_record_offset":31184,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":33460,"parent_record_offset":33428,"ancestor_record_offset":31184,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":33428,"parent_record_offset":33404,"ancestor_record_offset":31184,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":33500,"parent_record_offset":33460,"ancestor_record_offset":31184,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":34076,"parent_record_offset":34052,"ancestor_record_offset":31184,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":33304,"parent_record_offset":33280,"ancestor_record_offset":31184,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":33328,"parent_record_offset":33304,"ancestor_record_offset":31184,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":33352,"parent_record_offset":33328,"ancestor_record_offset":31184,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":33280,"parent_record_offset":33256,"ancestor_record_offset":31184,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":503,"record_offset":33256,"parent_record_offset":33232,"ancestor_record_offset":31184,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":503,"record_offset":33232,"parent_record_offset":33208,"ancestor_record_offset":31184,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":34100,"parent_record_offset":34076,"ancestor_record_offset":31184,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":33380,"parent_record_offset":33352,"ancestor_record_offset":31184,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":503,"record_offset":33524,"parent_record_offset":33500,"ancestor_record_offset":31184,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":503,"record_offset":32728,"parent_record_offset":32700,"ancestor_record_offset":31184,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":503,"record_offset":32756,"parent_record_offset":32728,"ancestor_record_offset":31184,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":503,"record_offset":32676,"parent_record_offset":32648,"ancestor_record_offset":31184,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":503,"record_offset":32700,"parent_record_offset":32676,"ancestor_record_offset":31184,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":503,"record_offset":32816,"parent_record_offset":32700,"ancestor_record_offset":31184,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":503,"record_offset":32928,"parent_record_offset":32904,"ancestor_record_offset":31184,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":503,"record_offset":33572,"parent_record_offset":33548,"ancestor_record_offset":31184,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":503,"record_offset":32780,"parent_record_offset":32756,"ancestor_record_offset":31184,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":503,"record_offset":33548,"parent_record_offset":33524,"ancestor_record_offset":31184,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":503,"record_offset":32548,"parent_record_offset":32508,"ancestor_record_offset":31184,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":503,"record_offset":32360,"parent_record_offset":32332,"ancestor_record_offset":31184,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":503,"record_offset":33164,"parent_record_offset":32508,"ancestor_record_offset":31184,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":503,"record_offset":33740,"parent_record_offset":33352,"ancestor_record_offset":31184,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":33764,"parent_record_offset":33740,"ancestor_record_offset":31184,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":32880,"parent_record_offset":32648,"ancestor_record_offset":31184,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":503,"record_offset":31820,"parent_record_offset":31796,"ancestor_record_offset":31184,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":503,"record_offset":31796,"parent_record_offset":31772,"ancestor_record_offset":31184,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":503,"record_offset":32004,"parent_record_offset":31976,"ancestor_record_offset":31184,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":503,"record_offset":32104,"parent_record_offset":32076,"ancestor_record_offset":31184,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":503,"record_offset":32076,"parent_record_offset":32048,"ancestor_record_offset":31184,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":503,"record_offset":31748,"parent_record_offset":31724,"ancestor_record_offset":31184,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":503,"record_offset":31772,"parent_record_offset":31748,"ancestor_record_offset":31184,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":503,"record_offset":32132,"parent_record_offset":32104,"ancestor_record_offset":31184,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":503,"record_offset":31724,"parent_record_offset":31612,"ancestor_record_offset":31184,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":503,"record_offset":32280,"parent_record_offset":32252,"ancestor_record_offset":31184,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":503,"record_offset":31440,"parent_record_offset":31408,"ancestor_record_offset":31184,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":503,"record_offset":31580,"parent_record_offset":31548,"ancestor_record_offset":31184,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":503,"record_offset":31548,"parent_record_offset":31512,"ancestor_record_offset":31184,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":503,"record_offset":31488,"parent_record_offset":31464,"ancestor_record_offset":31184,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":503,"record_offset":31512,"parent_record_offset":31488,"ancestor_record_offset":31184,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":503,"record_offset":31464,"parent_record_offset":31440,"ancestor_record_offset":31184,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":503,"record_offset":32048,"parent_record_offset":31512,"ancestor_record_offset":31184,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":503,"record_offset":32252,"parent_record_offset":32220,"ancestor_record_offset":31184,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":503,"record_offset":32308,"parent_record_offset":32252,"ancestor_record_offset":31184,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":503,"record_offset":32220,"parent_record_offset":32192,"ancestor_record_offset":31184,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":503,"record_offset":32192,"parent_record_offset":31408,"ancestor_record_offset":31184,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":503,"record_offset":32332,"parent_record_offset":32308,"ancestor_record_offset":31184,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":503,"record_offset":31612,"parent_record_offset":31580,"ancestor_record_offset":31184,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":503,"record_offset":31892,"parent_record_offset":31612,"ancestor_record_offset":31184,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":503,"record_offset":31864,"parent_record_offset":31612,"ancestor_record_offset":31184,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":503,"record_offset":31668,"parent_record_offset":31612,"ancestor_record_offset":31184,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":503,"record_offset":32444,"parent_record_offset":32412,"ancestor_record_offset":31184,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":503,"record_offset":34200,"parent_record_offset":34168,"ancestor_record_offset":31184,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":503,"record_offset":34640,"parent_record_offset":34436,"ancestor_record_offset":31184,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":503,"record_offset":32576,"parent_record_offset":32548,"ancestor_record_offset":31184,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":503,"record_offset":32648,"parent_record_offset":32508,"ancestor_record_offset":31184,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":503,"record_offset":32852,"parent_record_offset":32648,"ancestor_record_offset":31184,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":503,"record_offset":32604,"parent_record_offset":32576,"ancestor_record_offset":31184,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":503,"record_offset":32508,"parent_record_offset":32476,"ancestor_record_offset":31184,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":503,"record_offset":32476,"parent_record_offset":32444,"ancestor_record_offset":31184,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::impl$7::fmt
// 记录形式: compiled_procedure; instances=2
// 源码行: 197
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":120308,"rva":9486720,"va":5378195840}
// 编译实例 RVAs: 0x90c180, 0xa7cfa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::fmt::Formatter>::debug_tuple_field1_finish
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6621627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378195840,"raw_hex":"e8bb096500","size":5,"soft_fail":false,"target_va":5384817664,"va":5378196032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140f5e510
//     owner=unresolved_namespace; count=10
//     target_aliases=4 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t6628190","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5378195840,"raw_hex":"e95e236500","size":5,"soft_fail":false,"target_va":5384824080,"va":5378195885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugColor::as_array
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 208
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6FCD
// 首条位置: {"module":503,"inlinee":28621}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugRectangle3d::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 229
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":541,"record_offset":9636,"rva":10968048,"va":5379677168}
// 编译实例 RVAs: 0xa75bf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugRectangle3d::color
// 记录形式: compiled_procedure; instances=1
// 源码行: 238
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":541,"record_offset":9900,"rva":10968112,"va":5379677232}
// 编译实例 RVAs: 0xa75c30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugRectangle3d::line_width
// 记录形式: compiled_procedure; instances=1
// 源码行: 244
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":541,"record_offset":9360,"rva":10967952,"va":5379677072}
// 编译实例 RVAs: 0xa75b90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugRectangle3d::dashed_line
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 249
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x82CB
// 首条位置: {"module":541,"record_offset":9496,"rva":10968000,"va":5379677120}
// 编译实例 RVAs: 0xa75bc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugRectangle3d::show
// 记录形式: compiled_procedure; instances=1
// 源码行: 255
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":541,"record_offset":9768,"rva":10968096,"va":5379677216}
// 编译实例 RVAs: 0xa75c20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140905710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-1508629","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379677216,"raw_hex":"e9ebfae8ff","size":5,"soft_fail":false,"target_va":5378168592,"va":5379677216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::impl$3::show::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 255
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6FCB
// 首条位置: {"module":503,"inlinee":28619}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":503,"record_offset":42836,"parent_record_offset":42812,"ancestor_record_offset":42812,"inlinee":28609,"source_line":61,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugRectangle3d::show_for
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 259
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x82CF
// 首条位置: {"module":553,"inlinee":33487}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::impl$3::show_for::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 260
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6FC9
// 首条位置: {"module":503,"inlinee":28617}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::ui::signifiers::signifier_create::OwnedSignifierStream::write
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":503,"record_offset":37552,"parent_record_offset":37528,"ancestor_record_offset":37528,"inlinee":28613,"source_line":95,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugRectangle3d::emit<country_core::utils::debug_viz::impl$3::show::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 263
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":40576,"rva":9459472,"va":5378168592}
// 编译实例 RVAs: 0x905710
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-2490330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168592,"raw_hex":"e82600daff","size":5,"soft_fail":false,"target_va":5375678688,"va":5378169013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t-2490135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168592,"raw_hex":"e8e900daff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378168834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=4
//     target_aliases=1699 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9446811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168592,"raw_hex":"e865da6fff","size":5,"soft_fail":false,"target_va":5368722192,"va":5378168998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c922e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3720399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168592,"raw_hex":"e8cfc43800","size":5,"soft_fail":false,"target_va":5381890784,"va":5378170380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5155703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168592,"raw_hex":"e877ab4e00","size":5,"soft_fail":false,"target_va":5383325056,"va":5378169348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418055f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15727717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168592,"raw_hex":"e865fcef00","size":5,"soft_fail":false,"target_va":5393896944,"va":5378169222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38887883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168592,"raw_hex":"e8cb615102","size":5,"soft_fail":false,"target_va":5417057056,"va":5378169168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38450741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168592,"raw_hex":"e835b64a02","size":5,"soft_fail":false,"target_va":5416620769,"va":5378170023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6625905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168592,"raw_hex":"e8711a6500","size":5,"soft_fail":false,"target_va":5384794864,"va":5378168954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38451512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168592,"raw_hex":"e838b94a02","size":5,"soft_fail":false,"target_va":5416620794,"va":5378169277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_curve3
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5153577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168592,"raw_hex":"e829a34e00","size":5,"soft_fail":false,"target_va":5383322368,"va":5378168786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5153888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168592,"raw_hex":"e860a44e00","size":5,"soft_fail":false,"target_va":5383323216,"va":5378169323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5154557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168592,"raw_hex":"e8fda64e00","size":5,"soft_fail":false,"target_va":5383323424,"va":5378168862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":503,"record_offset":41304,"parent_record_offset":40576,"ancestor_record_offset":40576,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":503,"record_offset":41416,"parent_record_offset":40576,"ancestor_record_offset":40576,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":503,"record_offset":40808,"parent_record_offset":40576,"ancestor_record_offset":40576,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":503,"record_offset":41388,"parent_record_offset":40576,"ancestor_record_offset":40576,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":503,"record_offset":43152,"parent_record_offset":40576,"ancestor_record_offset":40576,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":41012,"parent_record_offset":40576,"ancestor_record_offset":40576,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":503,"record_offset":42000,"parent_record_offset":40576,"ancestor_record_offset":40576,"inlinee":28605,"source_line":191,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::impl$3::show::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":503,"record_offset":42812,"parent_record_offset":40576,"ancestor_record_offset":40576,"inlinee":28619,"source_line":255,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":503,"record_offset":43596,"parent_record_offset":40576,"ancestor_record_offset":40576,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":41552,"parent_record_offset":41528,"ancestor_record_offset":40576,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":41576,"parent_record_offset":41552,"ancestor_record_offset":40576,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":41200,"parent_record_offset":41176,"ancestor_record_offset":40576,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":41224,"parent_record_offset":41200,"ancestor_record_offset":40576,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":41600,"parent_record_offset":41576,"ancestor_record_offset":40576,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":41248,"parent_record_offset":41224,"ancestor_record_offset":40576,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":41528,"parent_record_offset":41496,"ancestor_record_offset":40576,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":41176,"parent_record_offset":41084,"ancestor_record_offset":40576,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":41496,"parent_record_offset":41452,"ancestor_record_offset":40576,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":41452,"parent_record_offset":41416,"ancestor_record_offset":40576,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":503,"record_offset":41352,"parent_record_offset":41328,"ancestor_record_offset":40576,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":42240,"parent_record_offset":42212,"ancestor_record_offset":40576,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":41112,"parent_record_offset":41084,"ancestor_record_offset":40576,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":41084,"parent_record_offset":41060,"ancestor_record_offset":40576,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":42300,"parent_record_offset":42268,"ancestor_record_offset":40576,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":42268,"parent_record_offset":42240,"ancestor_record_offset":40576,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":41060,"parent_record_offset":41036,"ancestor_record_offset":40576,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":42336,"parent_record_offset":42300,"ancestor_record_offset":40576,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":42120,"parent_record_offset":42092,"ancestor_record_offset":40576,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":42148,"parent_record_offset":42120,"ancestor_record_offset":40576,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":42176,"parent_record_offset":42148,"ancestor_record_offset":40576,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":42092,"parent_record_offset":42064,"ancestor_record_offset":40576,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":503,"record_offset":42064,"parent_record_offset":42036,"ancestor_record_offset":40576,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":503,"record_offset":42036,"parent_record_offset":42000,"ancestor_record_offset":40576,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":503,"record_offset":42680,"parent_record_offset":42656,"ancestor_record_offset":40576,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":42212,"parent_record_offset":42176,"ancestor_record_offset":40576,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":503,"record_offset":42360,"parent_record_offset":42336,"ancestor_record_offset":40576,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":503,"record_offset":40832,"parent_record_offset":40808,"ancestor_record_offset":40576,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":503,"record_offset":40888,"parent_record_offset":40856,"ancestor_record_offset":40576,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":503,"record_offset":43208,"parent_record_offset":43180,"ancestor_record_offset":40576,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":503,"record_offset":43760,"parent_record_offset":43736,"ancestor_record_offset":40576,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":503,"record_offset":42408,"parent_record_offset":42384,"ancestor_record_offset":40576,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":503,"record_offset":43180,"parent_record_offset":43152,"ancestor_record_offset":40576,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":503,"record_offset":40912,"parent_record_offset":40888,"ancestor_record_offset":40576,"inlinee":4923,"source_line":775,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":503,"record_offset":42384,"parent_record_offset":42360,"ancestor_record_offset":40576,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":503,"record_offset":41144,"parent_record_offset":41112,"ancestor_record_offset":40576,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":503,"record_offset":43668,"parent_record_offset":43620,"ancestor_record_offset":40576,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":503,"record_offset":40856,"parent_record_offset":40832,"ancestor_record_offset":40576,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":503,"record_offset":41328,"parent_record_offset":41304,"ancestor_record_offset":40576,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":503,"record_offset":42576,"parent_record_offset":42176,"ancestor_record_offset":40576,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":503,"record_offset":40936,"parent_record_offset":40912,"ancestor_record_offset":40576,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":42600,"parent_record_offset":42576,"ancestor_record_offset":40576,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":41036,"parent_record_offset":41012,"ancestor_record_offset":40576,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":40960,"parent_record_offset":40936,"ancestor_record_offset":40576,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":42716,"parent_record_offset":42680,"ancestor_record_offset":40576,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":42740,"parent_record_offset":42716,"ancestor_record_offset":40576,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":42772,"parent_record_offset":42680,"ancestor_record_offset":40576,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":503,"record_offset":42836,"parent_record_offset":42812,"ancestor_record_offset":40576,"inlinee":28609,"source_line":61,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":503,"record_offset":43696,"parent_record_offset":43620,"ancestor_record_offset":40576,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":503,"record_offset":43620,"parent_record_offset":43596,"ancestor_record_offset":40576,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":503,"record_offset":43736,"parent_record_offset":43696,"ancestor_record_offset":40576,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugRectangle3d::emit<country_core::utils::debug_viz::impl$3::show_for::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 263
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":35288,"rva":9456656,"va":5378165776}
// 编译实例 RVAs: 0x904c10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-2487541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378165776,"raw_hex":"e80b0bdaff","size":5,"soft_fail":false,"target_va":5375678688,"va":5378166224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t-2487346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378165776,"raw_hex":"e8ce0bdaff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378166045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=4
//     target_aliases=1699 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9444022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378165776,"raw_hex":"e84ae56fff","size":5,"soft_fail":false,"target_va":5368722192,"va":5378166209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c922e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3723151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378165776,"raw_hex":"e88fcf3800","size":5,"soft_fail":false,"target_va":5381890784,"va":5378167628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5158460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378165776,"raw_hex":"e83cb64e00","size":5,"soft_fail":false,"target_va":5383325056,"va":5378166591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418055f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15730488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378165776,"raw_hex":"e83807f000","size":5,"soft_fail":false,"target_va":5393896944,"va":5378166451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38890668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378165776,"raw_hex":"e8ac6c5102","size":5,"soft_fail":false,"target_va":5417057056,"va":5378166383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38453493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378165776,"raw_hex":"e8f5c04a02","size":5,"soft_fail":false,"target_va":5416620769,"va":5378167271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6628694","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378165776,"raw_hex":"e856256500","size":5,"soft_fail":false,"target_va":5384794864,"va":5378166165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38454279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378165776,"raw_hex":"e807c44a02","size":5,"soft_fail":false,"target_va":5416620794,"va":5378166510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_curve3
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5156366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378165776,"raw_hex":"e80eae4e00","size":5,"soft_fail":false,"target_va":5383322368,"va":5378165997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5156645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378165776,"raw_hex":"e825af4e00","size":5,"soft_fail":false,"target_va":5383323216,"va":5378166566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5157342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378165776,"raw_hex":"e8deb14e00","size":5,"soft_fail":false,"target_va":5383323424,"va":5378166077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":503,"record_offset":36020,"parent_record_offset":35288,"ancestor_record_offset":35288,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":503,"record_offset":36132,"parent_record_offset":35288,"ancestor_record_offset":35288,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":503,"record_offset":35524,"parent_record_offset":35288,"ancestor_record_offset":35288,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":503,"record_offset":36104,"parent_record_offset":35288,"ancestor_record_offset":35288,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":503,"record_offset":37868,"parent_record_offset":35288,"ancestor_record_offset":35288,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":35728,"parent_record_offset":35288,"ancestor_record_offset":35288,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":503,"record_offset":36716,"parent_record_offset":35288,"ancestor_record_offset":35288,"inlinee":28605,"source_line":191,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::impl$3::show_for::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":503,"record_offset":37528,"parent_record_offset":35288,"ancestor_record_offset":35288,"inlinee":28617,"source_line":260,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":503,"record_offset":38300,"parent_record_offset":35288,"ancestor_record_offset":35288,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":36268,"parent_record_offset":36244,"ancestor_record_offset":35288,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":36292,"parent_record_offset":36268,"ancestor_record_offset":35288,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":35916,"parent_record_offset":35892,"ancestor_record_offset":35288,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":35940,"parent_record_offset":35916,"ancestor_record_offset":35288,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":36316,"parent_record_offset":36292,"ancestor_record_offset":35288,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":35964,"parent_record_offset":35940,"ancestor_record_offset":35288,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":36244,"parent_record_offset":36212,"ancestor_record_offset":35288,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":35892,"parent_record_offset":35800,"ancestor_record_offset":35288,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":36212,"parent_record_offset":36168,"ancestor_record_offset":35288,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":36168,"parent_record_offset":36132,"ancestor_record_offset":35288,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":503,"record_offset":36068,"parent_record_offset":36044,"ancestor_record_offset":35288,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":36956,"parent_record_offset":36928,"ancestor_record_offset":35288,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":35828,"parent_record_offset":35800,"ancestor_record_offset":35288,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":35800,"parent_record_offset":35776,"ancestor_record_offset":35288,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":37016,"parent_record_offset":36984,"ancestor_record_offset":35288,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":36984,"parent_record_offset":36956,"ancestor_record_offset":35288,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":35776,"parent_record_offset":35752,"ancestor_record_offset":35288,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":37052,"parent_record_offset":37016,"ancestor_record_offset":35288,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":36836,"parent_record_offset":36808,"ancestor_record_offset":35288,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":36864,"parent_record_offset":36836,"ancestor_record_offset":35288,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":36892,"parent_record_offset":36864,"ancestor_record_offset":35288,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":36808,"parent_record_offset":36780,"ancestor_record_offset":35288,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":503,"record_offset":36780,"parent_record_offset":36752,"ancestor_record_offset":35288,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":503,"record_offset":36752,"parent_record_offset":36716,"ancestor_record_offset":35288,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":503,"record_offset":37396,"parent_record_offset":37372,"ancestor_record_offset":35288,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":36928,"parent_record_offset":36892,"ancestor_record_offset":35288,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":503,"record_offset":37076,"parent_record_offset":37052,"ancestor_record_offset":35288,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":503,"record_offset":35548,"parent_record_offset":35524,"ancestor_record_offset":35288,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":503,"record_offset":35604,"parent_record_offset":35572,"ancestor_record_offset":35288,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":503,"record_offset":37916,"parent_record_offset":37892,"ancestor_record_offset":35288,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":503,"record_offset":38464,"parent_record_offset":38440,"ancestor_record_offset":35288,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":503,"record_offset":37124,"parent_record_offset":37100,"ancestor_record_offset":35288,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":503,"record_offset":37892,"parent_record_offset":37868,"ancestor_record_offset":35288,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":503,"record_offset":35628,"parent_record_offset":35604,"ancestor_record_offset":35288,"inlinee":4923,"source_line":775,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":503,"record_offset":37100,"parent_record_offset":37076,"ancestor_record_offset":35288,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":503,"record_offset":35860,"parent_record_offset":35828,"ancestor_record_offset":35288,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":503,"record_offset":38372,"parent_record_offset":38324,"ancestor_record_offset":35288,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":503,"record_offset":35572,"parent_record_offset":35548,"ancestor_record_offset":35288,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":503,"record_offset":36044,"parent_record_offset":36020,"ancestor_record_offset":35288,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":503,"record_offset":37292,"parent_record_offset":36892,"ancestor_record_offset":35288,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":503,"record_offset":35652,"parent_record_offset":35628,"ancestor_record_offset":35288,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":37316,"parent_record_offset":37292,"ancestor_record_offset":35288,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":35752,"parent_record_offset":35728,"ancestor_record_offset":35288,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":35676,"parent_record_offset":35652,"ancestor_record_offset":35288,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":37432,"parent_record_offset":37396,"ancestor_record_offset":35288,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":37456,"parent_record_offset":37432,"ancestor_record_offset":35288,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":37488,"parent_record_offset":37396,"ancestor_record_offset":35288,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::signifiers::signifier_create::OwnedSignifierStream::write
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":503,"record_offset":37552,"parent_record_offset":37528,"ancestor_record_offset":35288,"inlinee":28613,"source_line":95,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":503,"record_offset":38400,"parent_record_offset":38324,"ancestor_record_offset":35288,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":503,"record_offset":38324,"parent_record_offset":38300,"ancestor_record_offset":35288,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":503,"record_offset":38440,"parent_record_offset":38400,"ancestor_record_offset":35288,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::impl$3::emit::closure$0<country_core::utils::debug_viz::impl$3::show::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 281
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":45872,"rva":9462224,"va":5378171344}
// 编译实例 RVAs: 0x9061d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2492791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378171344,"raw_hex":"e889f6d9ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5378171474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2493342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378171344,"raw_hex":"e862f4d9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378172041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9449268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378171344,"raw_hex":"e8ccd06fff","size":5,"soft_fail":false,"target_va":5368722192,"va":5378171455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f6400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13566160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378171344,"raw_hex":"e8d000cf00","size":5,"soft_fail":false,"target_va":5391737856,"va":5378171691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38885540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378171344,"raw_hex":"e8a4585102","size":5,"soft_fail":false,"target_va":5417057056,"va":5378171511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38448789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378171344,"raw_hex":"e895ae4a02","size":5,"soft_fail":false,"target_va":5416620769,"va":5378171975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38449367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378171344,"raw_hex":"e8d7b04a02","size":5,"soft_fail":false,"target_va":5416620794,"va":5378171422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t766845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378171344,"raw_hex":"e87db30b00","size":5,"soft_fail":false,"target_va":5378938736,"va":5378171886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":46976,"parent_record_offset":45872,"ancestor_record_offset":45872,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":46792,"parent_record_offset":45872,"ancestor_record_offset":45872,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":46096,"parent_record_offset":45872,"ancestor_record_offset":45872,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":47060,"parent_record_offset":45872,"ancestor_record_offset":45872,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::line_topology
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":503,"record_offset":46764,"parent_record_offset":45872,"ancestor_record_offset":45872,"inlinee":28623,"source_line":284,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":503,"record_offset":46736,"parent_record_offset":45872,"ancestor_record_offset":45872,"inlinee":28622,"source_line":234,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugColor::as_array
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":503,"record_offset":46680,"parent_record_offset":45872,"ancestor_record_offset":45872,"inlinee":28621,"source_line":208,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":46508,"parent_record_offset":46484,"ancestor_record_offset":45872,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":46532,"parent_record_offset":46508,"ancestor_record_offset":45872,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":47244,"parent_record_offset":47220,"ancestor_record_offset":45872,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":47268,"parent_record_offset":47244,"ancestor_record_offset":45872,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":46556,"parent_record_offset":46532,"ancestor_record_offset":45872,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":47292,"parent_record_offset":47268,"ancestor_record_offset":45872,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":46484,"parent_record_offset":46304,"ancestor_record_offset":45872,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":47220,"parent_record_offset":47132,"ancestor_record_offset":45872,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":46820,"parent_record_offset":46792,"ancestor_record_offset":45872,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":46248,"parent_record_offset":46224,"ancestor_record_offset":45872,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":47160,"parent_record_offset":47132,"ancestor_record_offset":45872,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":47132,"parent_record_offset":47108,"ancestor_record_offset":45872,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":46304,"parent_record_offset":46272,"ancestor_record_offset":45872,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":46272,"parent_record_offset":46248,"ancestor_record_offset":45872,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":47108,"parent_record_offset":47084,"ancestor_record_offset":45872,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":46340,"parent_record_offset":46304,"ancestor_record_offset":45872,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":47000,"parent_record_offset":46976,"ancestor_record_offset":45872,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":46188,"parent_record_offset":46124,"ancestor_record_offset":45872,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":46160,"parent_record_offset":46124,"ancestor_record_offset":45872,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":47024,"parent_record_offset":47000,"ancestor_record_offset":45872,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":46224,"parent_record_offset":46188,"ancestor_record_offset":45872,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":46124,"parent_record_offset":46096,"ancestor_record_offset":45872,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":503,"record_offset":46364,"parent_record_offset":46340,"ancestor_record_offset":45872,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":503,"record_offset":46444,"parent_record_offset":46388,"ancestor_record_offset":45872,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":503,"record_offset":46388,"parent_record_offset":46364,"ancestor_record_offset":45872,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":503,"record_offset":46416,"parent_record_offset":46388,"ancestor_record_offset":45872,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":503,"record_offset":47188,"parent_record_offset":47160,"ancestor_record_offset":45872,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":503,"record_offset":46612,"parent_record_offset":46188,"ancestor_record_offset":45872,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":46636,"parent_record_offset":46612,"ancestor_record_offset":45872,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":47084,"parent_record_offset":47060,"ancestor_record_offset":45872,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::impl$3::emit::closure$1<country_core::utils::debug_viz::impl$3::show::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 314
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":47452,"rva":9462944,"va":5378172064}
// 编译实例 RVAs: 0x9064a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2493437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378172064,"raw_hex":"e803f4d9ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5378172120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2493962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378172064,"raw_hex":"e8f6f1d9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378172661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9449918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378172064,"raw_hex":"e842ce6fff","size":5,"soft_fail":false,"target_va":5368722192,"va":5378172105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f6400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13565532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378172064,"raw_hex":"e85cfece00","size":5,"soft_fail":false,"target_va":5391737856,"va":5378172319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38448163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378172064,"raw_hex":"e823ac4a02","size":5,"soft_fail":false,"target_va":5416620769,"va":5378172601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t766213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378172064,"raw_hex":"e805b10b00","size":5,"soft_fail":false,"target_va":5378938736,"va":5378172518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":48144,"parent_record_offset":47452,"ancestor_record_offset":47452,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":47672,"parent_record_offset":47452,"ancestor_record_offset":47452,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":48228,"parent_record_offset":47452,"ancestor_record_offset":47452,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::line_topology
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":503,"record_offset":47932,"parent_record_offset":47452,"ancestor_record_offset":47452,"inlinee":28623,"source_line":284,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":503,"record_offset":47904,"parent_record_offset":47452,"ancestor_record_offset":47452,"inlinee":28622,"source_line":234,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugColor::as_array
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":503,"record_offset":47852,"parent_record_offset":47452,"ancestor_record_offset":47452,"inlinee":28621,"source_line":208,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":47756,"parent_record_offset":47732,"ancestor_record_offset":47452,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":47780,"parent_record_offset":47756,"ancestor_record_offset":47452,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":48348,"parent_record_offset":48324,"ancestor_record_offset":47452,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":48372,"parent_record_offset":48348,"ancestor_record_offset":47452,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":47804,"parent_record_offset":47780,"ancestor_record_offset":47452,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":48396,"parent_record_offset":48372,"ancestor_record_offset":47452,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":47732,"parent_record_offset":47700,"ancestor_record_offset":47452,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":48324,"parent_record_offset":48300,"ancestor_record_offset":47452,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":47700,"parent_record_offset":47672,"ancestor_record_offset":47452,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":48300,"parent_record_offset":48276,"ancestor_record_offset":47452,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":48276,"parent_record_offset":48252,"ancestor_record_offset":47452,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":48168,"parent_record_offset":48144,"ancestor_record_offset":47452,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":48192,"parent_record_offset":48168,"ancestor_record_offset":47452,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":48252,"parent_record_offset":48228,"ancestor_record_offset":47452,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugRectangle2d::new
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 346
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x82C5
// 首条位置: {"module":550,"inlinee":33477}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugRectangle2d::color
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 355
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x82C9
// 首条位置: {"module":550,"inlinee":33481}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugRectangle2d::show
// 记录形式: compiled_procedure; instances=1
// 源码行: 366
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":541,"record_offset":4792,"rva":10966240,"va":5379675360}
// 编译实例 RVAs: 0xa754e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-3996916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379675360,"raw_hex":"e80c03c3ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5379675599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-3996774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379675360,"raw_hex":"e89a03c3ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379675473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10953397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379675360,"raw_hex":"e84bdd58ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5379675584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c922e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2214652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379675360,"raw_hex":"e8fcca2100","size":5,"soft_fail":false,"target_va":5381890784,"va":5379676127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418055f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14221131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379675360,"raw_hex":"e84bffd800","size":5,"soft_fail":false,"target_va":5393896944,"va":5379675808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37381286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379675360,"raw_hex":"e8a6643a02","size":5,"soft_fail":false,"target_va":5417057056,"va":5379675765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36944892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379675360,"raw_hex":"e8fcbb3302","size":5,"soft_fail":false,"target_va":5416620769,"va":5379675872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5119301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379675360,"raw_hex":"e8451d4e00","size":5,"soft_fail":false,"target_va":5384794864,"va":5379675558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36945078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379675360,"raw_hex":"e8b6bc3302","size":5,"soft_fail":false,"target_va":5416620794,"va":5379675711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_curve3
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3646938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379675360,"raw_hex":"e8daa53700","size":5,"soft_fail":false,"target_va":5383322368,"va":5379675425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":541,"record_offset":5232,"parent_record_offset":4792,"ancestor_record_offset":4792,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":541,"record_offset":5344,"parent_record_offset":4792,"ancestor_record_offset":4792,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":541,"record_offset":5316,"parent_record_offset":4792,"ancestor_record_offset":4792,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":541,"record_offset":4952,"parent_record_offset":4792,"ancestor_record_offset":4792,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":541,"record_offset":5920,"parent_record_offset":4792,"ancestor_record_offset":4792,"inlinee":28605,"source_line":191,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":541,"record_offset":6696,"parent_record_offset":4792,"ancestor_record_offset":4792,"inlinee":28609,"source_line":61,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":5472,"parent_record_offset":5448,"ancestor_record_offset":4792,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":5496,"parent_record_offset":5472,"ancestor_record_offset":4792,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":5128,"parent_record_offset":5104,"ancestor_record_offset":4792,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":5152,"parent_record_offset":5128,"ancestor_record_offset":4792,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":5520,"parent_record_offset":5496,"ancestor_record_offset":4792,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":5176,"parent_record_offset":5152,"ancestor_record_offset":4792,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":5448,"parent_record_offset":5416,"ancestor_record_offset":4792,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":5104,"parent_record_offset":5016,"ancestor_record_offset":4792,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":541,"record_offset":5416,"parent_record_offset":5376,"ancestor_record_offset":4792,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":541,"record_offset":5376,"parent_record_offset":5344,"ancestor_record_offset":4792,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":541,"record_offset":5280,"parent_record_offset":5256,"ancestor_record_offset":4792,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":6124,"parent_record_offset":6100,"ancestor_record_offset":4792,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":5044,"parent_record_offset":5016,"ancestor_record_offset":4792,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":5016,"parent_record_offset":4992,"ancestor_record_offset":4792,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":6180,"parent_record_offset":6148,"ancestor_record_offset":4792,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":6148,"parent_record_offset":6124,"ancestor_record_offset":4792,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":4992,"parent_record_offset":4972,"ancestor_record_offset":4792,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":6220,"parent_record_offset":6180,"ancestor_record_offset":4792,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":541,"record_offset":6024,"parent_record_offset":6000,"ancestor_record_offset":4792,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":541,"record_offset":6048,"parent_record_offset":6024,"ancestor_record_offset":4792,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":541,"record_offset":6072,"parent_record_offset":6048,"ancestor_record_offset":4792,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":541,"record_offset":6000,"parent_record_offset":5976,"ancestor_record_offset":4792,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":541,"record_offset":5976,"parent_record_offset":5952,"ancestor_record_offset":4792,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":541,"record_offset":5952,"parent_record_offset":5920,"ancestor_record_offset":4792,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":541,"record_offset":6564,"parent_record_offset":6540,"ancestor_record_offset":4792,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":541,"record_offset":6100,"parent_record_offset":6072,"ancestor_record_offset":4792,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":541,"record_offset":6244,"parent_record_offset":6220,"ancestor_record_offset":4792,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":541,"record_offset":6292,"parent_record_offset":6268,"ancestor_record_offset":4792,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":541,"record_offset":6268,"parent_record_offset":6244,"ancestor_record_offset":4792,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":541,"record_offset":5072,"parent_record_offset":5044,"ancestor_record_offset":4792,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":541,"record_offset":5256,"parent_record_offset":5232,"ancestor_record_offset":4792,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":541,"record_offset":6460,"parent_record_offset":6072,"ancestor_record_offset":4792,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":541,"record_offset":6484,"parent_record_offset":6460,"ancestor_record_offset":4792,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":541,"record_offset":4972,"parent_record_offset":4952,"ancestor_record_offset":4792,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":6600,"parent_record_offset":6564,"ancestor_record_offset":4792,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":6624,"parent_record_offset":6600,"ancestor_record_offset":4792,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":6656,"parent_record_offset":6564,"ancestor_record_offset":4792,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::impl$4::show::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 375
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":541,"record_offset":7580,"rva":10967136,"va":5379676256}
// 编译实例 RVAs: 0xa75860
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-3997638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379676256,"raw_hex":"e83a00c3ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5379676321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-3998256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379676256,"raw_hex":"e8d0fdc2ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379676955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10954119","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379676256,"raw_hex":"e879da58ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5379676306,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f6400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12061195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379676256,"raw_hex":"e80b0ab800","size":5,"soft_fail":false,"target_va":5391737856,"va":5379676656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37380593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379676256,"raw_hex":"e8f1613a02","size":5,"soft_fail":false,"target_va":5417057056,"va":5379676458,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36943864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379676256,"raw_hex":"e8f8b73302","size":5,"soft_fail":false,"target_va":5416620769,"va":5379676900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36944410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379676256,"raw_hex":"e81aba3302","size":5,"soft_fail":false,"target_va":5416620794,"va":5379676379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-738100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379676256,"raw_hex":"e8ccbcf4ff","size":5,"soft_fail":false,"target_va":5378938736,"va":5379676831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":541,"record_offset":8616,"parent_record_offset":7580,"ancestor_record_offset":7580,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":541,"record_offset":7744,"parent_record_offset":7580,"ancestor_record_offset":7580,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":541,"record_offset":7924,"parent_record_offset":7580,"ancestor_record_offset":7580,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":541,"record_offset":8700,"parent_record_offset":7580,"ancestor_record_offset":7580,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::line_topology
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":541,"record_offset":8588,"parent_record_offset":7580,"ancestor_record_offset":7580,"inlinee":28623,"source_line":284,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":541,"record_offset":8560,"parent_record_offset":7580,"ancestor_record_offset":7580,"inlinee":28622,"source_line":234,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugColor::as_array
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":541,"record_offset":8508,"parent_record_offset":7580,"ancestor_record_offset":7580,"inlinee":28621,"source_line":208,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":7828,"parent_record_offset":7804,"ancestor_record_offset":7580,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":7852,"parent_record_offset":7828,"ancestor_record_offset":7580,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":8772,"parent_record_offset":8748,"ancestor_record_offset":7580,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":8796,"parent_record_offset":8772,"ancestor_record_offset":7580,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":7876,"parent_record_offset":7852,"ancestor_record_offset":7580,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":8820,"parent_record_offset":8796,"ancestor_record_offset":7580,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":7804,"parent_record_offset":7772,"ancestor_record_offset":7580,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":8748,"parent_record_offset":8724,"ancestor_record_offset":7580,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":541,"record_offset":7772,"parent_record_offset":7744,"ancestor_record_offset":7580,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":541,"record_offset":8724,"parent_record_offset":8700,"ancestor_record_offset":7580,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":8076,"parent_record_offset":8052,"ancestor_record_offset":7580,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":8968,"parent_record_offset":8940,"ancestor_record_offset":7580,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":8940,"parent_record_offset":8916,"ancestor_record_offset":7580,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":8132,"parent_record_offset":8100,"ancestor_record_offset":7580,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":8100,"parent_record_offset":8076,"ancestor_record_offset":7580,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":8916,"parent_record_offset":8892,"ancestor_record_offset":7580,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":8168,"parent_record_offset":8132,"ancestor_record_offset":7580,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":541,"record_offset":8640,"parent_record_offset":8616,"ancestor_record_offset":7580,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":541,"record_offset":8016,"parent_record_offset":7952,"ancestor_record_offset":7580,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":541,"record_offset":7988,"parent_record_offset":7952,"ancestor_record_offset":7580,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":541,"record_offset":8664,"parent_record_offset":8640,"ancestor_record_offset":7580,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":541,"record_offset":8052,"parent_record_offset":8016,"ancestor_record_offset":7580,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":541,"record_offset":7952,"parent_record_offset":7924,"ancestor_record_offset":7580,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":541,"record_offset":8192,"parent_record_offset":8168,"ancestor_record_offset":7580,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":541,"record_offset":8272,"parent_record_offset":8216,"ancestor_record_offset":7580,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":541,"record_offset":8216,"parent_record_offset":8192,"ancestor_record_offset":7580,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":541,"record_offset":8244,"parent_record_offset":8216,"ancestor_record_offset":7580,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":541,"record_offset":8996,"parent_record_offset":8968,"ancestor_record_offset":7580,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":541,"record_offset":8440,"parent_record_offset":8016,"ancestor_record_offset":7580,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":541,"record_offset":8464,"parent_record_offset":8440,"ancestor_record_offset":7580,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":541,"record_offset":8892,"parent_record_offset":8868,"ancestor_record_offset":7580,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugLine::new
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=11
// 源码行: 403
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x827C
// 首条位置: {"module":541,"inlinee":33404}
// 编译实例 RVAs: 0xa76330
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugLine::color
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=13
// 源码行: 425
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x82D6
// 首条位置: {"module":541,"record_offset":15708,"rva":10969984,"va":5379679104}
// 编译实例 RVAs: 0xa76380
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugLine::dashed
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=14
// 源码行: 435
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x82D7
// 首条位置: {"module":541,"record_offset":15832,"rva":10970032,"va":5379679152}
// 编译实例 RVAs: 0xa763b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugLine::line_width
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=23
// 源码行: 441
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6FFC
// 首条位置: {"module":503,"inlinee":28668}
// 编译实例 RVAs: 0xa76300
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugLine::show
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=40
// 源码行: 446, 447
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6FFD
// 首条位置: {"module":503,"inlinee":28669}
// 编译实例 RVAs: 0xa76370
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140906710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-1506405","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379679088,"raw_hex":"e99b03e9ff","size":5,"soft_fail":false,"target_va":5378172688,"va":5379679088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::impl$5::show::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 447
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6FD3
// 首条位置: {"module":503,"inlinee":28627}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":503,"record_offset":50704,"parent_record_offset":50680,"ancestor_record_offset":50680,"inlinee":28609,"source_line":61,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugLine::show_for
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=10
// 源码行: 451, 452
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7261
// 首条位置: {"module":510,"inlinee":29281}
// 编译实例 RVAs: 0xa763e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140906c40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-1505189","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379679200,"raw_hex":"e95b08e9ff","size":5,"soft_fail":false,"target_va":5378174016,"va":5379679200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::impl$5::show_for::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 452
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6FD5
// 首条位置: {"module":503,"inlinee":28629}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::ui::signifiers::signifier_create::OwnedSignifierStream::write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":503,"record_offset":54448,"parent_record_offset":54424,"ancestor_record_offset":54424,"inlinee":28613,"source_line":95,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugLine::emit<country_core::utils::debug_viz::impl$5::show::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 455
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":48556,"rva":9463568,"va":5378172688}
// 编译实例 RVAs: 0x906710
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2494655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378172688,"raw_hex":"e841efd9ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5378173338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-2494855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378172688,"raw_hex":"e879eed9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378173554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9451136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378172688,"raw_hex":"e880c96fff","size":5,"soft_fail":false,"target_va":5368722192,"va":5378173323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140986f90
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t525508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378172688,"raw_hex":"e8c4040800","size":5,"soft_fail":false,"target_va":5378699152,"va":5378173639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c922e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3717050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378172688,"raw_hex":"e8bab73800","size":5,"soft_fail":false,"target_va":5381890784,"va":5378173729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f6400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13564780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378172688,"raw_hex":"e86cfbce00","size":5,"soft_fail":false,"target_va":5391737856,"va":5378173071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418055f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15723410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378172688,"raw_hex":"e892ebef00","size":5,"soft_fail":false,"target_va":5393896944,"va":5378173529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38883569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378172688,"raw_hex":"e8f1505102","size":5,"soft_fail":false,"target_va":5417057056,"va":5378173482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38447173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378172688,"raw_hex":"e845a84a02","size":5,"soft_fail":false,"target_va":5416620769,"va":5378173591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6622026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378172688,"raw_hex":"e84a0b6500","size":5,"soft_fail":false,"target_va":5384794864,"va":5378172833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38447362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378172688,"raw_hex":"e802a94a02","size":5,"soft_fail":false,"target_va":5416620794,"va":5378173427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":503,"record_offset":48812,"parent_record_offset":48556,"ancestor_record_offset":48556,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":503,"record_offset":49672,"parent_record_offset":48556,"ancestor_record_offset":48556,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":503,"record_offset":48896,"parent_record_offset":48556,"ancestor_record_offset":48556,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":503,"record_offset":48784,"parent_record_offset":48556,"ancestor_record_offset":48556,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":503,"record_offset":48924,"parent_record_offset":48556,"ancestor_record_offset":48556,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":50736,"parent_record_offset":48556,"ancestor_record_offset":48556,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":503,"record_offset":49904,"parent_record_offset":48556,"ancestor_record_offset":48556,"inlinee":28605,"source_line":191,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::impl$5::show::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":503,"record_offset":50680,"parent_record_offset":48556,"ancestor_record_offset":48556,"inlinee":28627,"source_line":447,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":503,"record_offset":49356,"parent_record_offset":48556,"ancestor_record_offset":48556,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":503,"record_offset":49052,"parent_record_offset":48556,"ancestor_record_offset":48556,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":49800,"parent_record_offset":49776,"ancestor_record_offset":48556,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":49824,"parent_record_offset":49800,"ancestor_record_offset":48556,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":50912,"parent_record_offset":50888,"ancestor_record_offset":48556,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":50936,"parent_record_offset":50912,"ancestor_record_offset":48556,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":49848,"parent_record_offset":49824,"ancestor_record_offset":48556,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":50960,"parent_record_offset":50936,"ancestor_record_offset":48556,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":49776,"parent_record_offset":49744,"ancestor_record_offset":48556,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":50888,"parent_record_offset":50832,"ancestor_record_offset":48556,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":49744,"parent_record_offset":49704,"ancestor_record_offset":48556,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":49704,"parent_record_offset":49672,"ancestor_record_offset":48556,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":503,"record_offset":48860,"parent_record_offset":48836,"ancestor_record_offset":48556,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":50108,"parent_record_offset":50084,"ancestor_record_offset":48556,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":50860,"parent_record_offset":50832,"ancestor_record_offset":48556,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":50832,"parent_record_offset":50808,"ancestor_record_offset":48556,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":50164,"parent_record_offset":50132,"ancestor_record_offset":48556,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":50132,"parent_record_offset":50108,"ancestor_record_offset":48556,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":50808,"parent_record_offset":50784,"ancestor_record_offset":48556,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":50204,"parent_record_offset":50164,"ancestor_record_offset":48556,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":50008,"parent_record_offset":49984,"ancestor_record_offset":48556,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":50032,"parent_record_offset":50008,"ancestor_record_offset":48556,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":50056,"parent_record_offset":50032,"ancestor_record_offset":48556,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":49984,"parent_record_offset":49960,"ancestor_record_offset":48556,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":503,"record_offset":49960,"parent_record_offset":49936,"ancestor_record_offset":48556,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":503,"record_offset":49936,"parent_record_offset":49904,"ancestor_record_offset":48556,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":503,"record_offset":50548,"parent_record_offset":50524,"ancestor_record_offset":48556,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":50084,"parent_record_offset":50056,"ancestor_record_offset":48556,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":503,"record_offset":50228,"parent_record_offset":50204,"ancestor_record_offset":48556,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":503,"record_offset":49596,"parent_record_offset":49568,"ancestor_record_offset":48556,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":503,"record_offset":50276,"parent_record_offset":50252,"ancestor_record_offset":48556,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":503,"record_offset":49180,"parent_record_offset":49156,"ancestor_record_offset":48556,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":503,"record_offset":49228,"parent_record_offset":49204,"ancestor_record_offset":48556,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":503,"record_offset":49252,"parent_record_offset":49228,"ancestor_record_offset":48556,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":503,"record_offset":50252,"parent_record_offset":50228,"ancestor_record_offset":48556,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":503,"record_offset":51144,"parent_record_offset":51120,"ancestor_record_offset":48556,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":503,"record_offset":49452,"parent_record_offset":49392,"ancestor_record_offset":48556,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":503,"record_offset":48836,"parent_record_offset":48812,"ancestor_record_offset":48556,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":503,"record_offset":49204,"parent_record_offset":49180,"ancestor_record_offset":48556,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":503,"record_offset":50444,"parent_record_offset":50056,"ancestor_record_offset":48556,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":50468,"parent_record_offset":50444,"ancestor_record_offset":48556,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":50524,"parent_record_offset":49904,"ancestor_record_offset":48556,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":503,"record_offset":49300,"parent_record_offset":49276,"ancestor_record_offset":48556,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":503,"record_offset":49124,"parent_record_offset":49092,"ancestor_record_offset":48556,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":503,"record_offset":49092,"parent_record_offset":49052,"ancestor_record_offset":48556,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":50584,"parent_record_offset":50548,"ancestor_record_offset":48556,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":50608,"parent_record_offset":50584,"ancestor_record_offset":48556,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":50640,"parent_record_offset":50548,"ancestor_record_offset":48556,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":503,"record_offset":50704,"parent_record_offset":50680,"ancestor_record_offset":48556,"inlinee":28609,"source_line":61,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::debug_viz::DebugColor::as_array
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":503,"record_offset":48988,"parent_record_offset":48960,"ancestor_record_offset":48556,"inlinee":28621,"source_line":208,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::debug_viz::impl$5::emit::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":503,"record_offset":48960,"parent_record_offset":48924,"ancestor_record_offset":48556,"inlinee":28625,"source_line":471,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":503,"record_offset":49516,"parent_record_offset":49392,"ancestor_record_offset":48556,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":503,"record_offset":49392,"parent_record_offset":49356,"ancestor_record_offset":48556,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":503,"record_offset":49156,"parent_record_offset":49052,"ancestor_record_offset":48556,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":503,"record_offset":49568,"parent_record_offset":49516,"ancestor_record_offset":48556,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::DebugLine::emit<country_core::utils::debug_viz::impl$5::show_for::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 455
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":52296,"rva":9464896,"va":5378174016}
// 编译实例 RVAs: 0x906c40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2495983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378174016,"raw_hex":"e811ead9ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5378174666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-2496194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378174016,"raw_hex":"e83ee9d9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378174893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9452464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378174016,"raw_hex":"e850c46fff","size":5,"soft_fail":false,"target_va":5368722192,"va":5378174651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140986f90
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t524164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378174016,"raw_hex":"e884ff0700","size":5,"soft_fail":false,"target_va":5378699152,"va":5378174983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c922e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3715706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378174016,"raw_hex":"e87ab23800","size":5,"soft_fail":false,"target_va":5381890784,"va":5378175073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f6400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13563452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378174016,"raw_hex":"e83cf6ce00","size":5,"soft_fail":false,"target_va":5391737856,"va":5378174399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418055f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15722071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378174016,"raw_hex":"e857e6ef00","size":5,"soft_fail":false,"target_va":5393896944,"va":5378174868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38882241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378174016,"raw_hex":"e8c14b5102","size":5,"soft_fail":false,"target_va":5417057056,"va":5378174810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38445834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378174016,"raw_hex":"e80aa34a02","size":5,"soft_fail":false,"target_va":5416620769,"va":5378174930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6620698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378174016,"raw_hex":"e81a066500","size":5,"soft_fail":false,"target_va":5384794864,"va":5378174161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38446034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378174016,"raw_hex":"e8d2a34a02","size":5,"soft_fail":false,"target_va":5416620794,"va":5378174755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":503,"record_offset":52556,"parent_record_offset":52296,"ancestor_record_offset":52296,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":503,"record_offset":53416,"parent_record_offset":52296,"ancestor_record_offset":52296,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":503,"record_offset":52640,"parent_record_offset":52296,"ancestor_record_offset":52296,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":503,"record_offset":52528,"parent_record_offset":52296,"ancestor_record_offset":52296,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":503,"record_offset":52668,"parent_record_offset":52296,"ancestor_record_offset":52296,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":54480,"parent_record_offset":52296,"ancestor_record_offset":52296,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":503,"record_offset":53648,"parent_record_offset":52296,"ancestor_record_offset":52296,"inlinee":28605,"source_line":191,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::impl$5::show_for::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":503,"record_offset":54424,"parent_record_offset":52296,"ancestor_record_offset":52296,"inlinee":28629,"source_line":452,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":503,"record_offset":53100,"parent_record_offset":52296,"ancestor_record_offset":52296,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":503,"record_offset":52796,"parent_record_offset":52296,"ancestor_record_offset":52296,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":53544,"parent_record_offset":53520,"ancestor_record_offset":52296,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":53568,"parent_record_offset":53544,"ancestor_record_offset":52296,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":54656,"parent_record_offset":54632,"ancestor_record_offset":52296,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":54680,"parent_record_offset":54656,"ancestor_record_offset":52296,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":53592,"parent_record_offset":53568,"ancestor_record_offset":52296,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":54704,"parent_record_offset":54680,"ancestor_record_offset":52296,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":53520,"parent_record_offset":53488,"ancestor_record_offset":52296,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":54632,"parent_record_offset":54576,"ancestor_record_offset":52296,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":53488,"parent_record_offset":53448,"ancestor_record_offset":52296,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":53448,"parent_record_offset":53416,"ancestor_record_offset":52296,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":503,"record_offset":52604,"parent_record_offset":52580,"ancestor_record_offset":52296,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":53852,"parent_record_offset":53828,"ancestor_record_offset":52296,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":54604,"parent_record_offset":54576,"ancestor_record_offset":52296,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":54576,"parent_record_offset":54552,"ancestor_record_offset":52296,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":53908,"parent_record_offset":53876,"ancestor_record_offset":52296,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":53876,"parent_record_offset":53852,"ancestor_record_offset":52296,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":54552,"parent_record_offset":54528,"ancestor_record_offset":52296,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":53948,"parent_record_offset":53908,"ancestor_record_offset":52296,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":53752,"parent_record_offset":53728,"ancestor_record_offset":52296,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":53776,"parent_record_offset":53752,"ancestor_record_offset":52296,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":53800,"parent_record_offset":53776,"ancestor_record_offset":52296,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":53728,"parent_record_offset":53704,"ancestor_record_offset":52296,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":503,"record_offset":53704,"parent_record_offset":53680,"ancestor_record_offset":52296,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":503,"record_offset":53680,"parent_record_offset":53648,"ancestor_record_offset":52296,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":503,"record_offset":54292,"parent_record_offset":54268,"ancestor_record_offset":52296,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":53828,"parent_record_offset":53800,"ancestor_record_offset":52296,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":503,"record_offset":53972,"parent_record_offset":53948,"ancestor_record_offset":52296,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":503,"record_offset":53340,"parent_record_offset":53312,"ancestor_record_offset":52296,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":503,"record_offset":54020,"parent_record_offset":53996,"ancestor_record_offset":52296,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":503,"record_offset":52924,"parent_record_offset":52900,"ancestor_record_offset":52296,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":503,"record_offset":52972,"parent_record_offset":52948,"ancestor_record_offset":52296,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":503,"record_offset":52996,"parent_record_offset":52972,"ancestor_record_offset":52296,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":503,"record_offset":53996,"parent_record_offset":53972,"ancestor_record_offset":52296,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":503,"record_offset":54888,"parent_record_offset":54864,"ancestor_record_offset":52296,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":503,"record_offset":53196,"parent_record_offset":53136,"ancestor_record_offset":52296,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":503,"record_offset":52580,"parent_record_offset":52556,"ancestor_record_offset":52296,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":503,"record_offset":52948,"parent_record_offset":52924,"ancestor_record_offset":52296,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":503,"record_offset":54188,"parent_record_offset":53800,"ancestor_record_offset":52296,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":54212,"parent_record_offset":54188,"ancestor_record_offset":52296,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":54268,"parent_record_offset":53648,"ancestor_record_offset":52296,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":503,"record_offset":53044,"parent_record_offset":53020,"ancestor_record_offset":52296,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":503,"record_offset":52868,"parent_record_offset":52836,"ancestor_record_offset":52296,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":503,"record_offset":52836,"parent_record_offset":52796,"ancestor_record_offset":52296,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":54328,"parent_record_offset":54292,"ancestor_record_offset":52296,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":54352,"parent_record_offset":54328,"ancestor_record_offset":52296,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":54384,"parent_record_offset":54292,"ancestor_record_offset":52296,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::signifiers::signifier_create::OwnedSignifierStream::write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":503,"record_offset":54448,"parent_record_offset":54424,"ancestor_record_offset":52296,"inlinee":28613,"source_line":95,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::debug_viz::DebugColor::as_array
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":503,"record_offset":52732,"parent_record_offset":52704,"ancestor_record_offset":52296,"inlinee":28621,"source_line":208,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::debug_viz::impl$5::emit::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":503,"record_offset":52704,"parent_record_offset":52668,"ancestor_record_offset":52296,"inlinee":28625,"source_line":471,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":503,"record_offset":53260,"parent_record_offset":53136,"ancestor_record_offset":52296,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":503,"record_offset":53136,"parent_record_offset":53100,"ancestor_record_offset":52296,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":503,"record_offset":52900,"parent_record_offset":52796,"ancestor_record_offset":52296,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":503,"record_offset":53312,"parent_record_offset":53260,"ancestor_record_offset":52296,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::impl$5::emit::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 471
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6FD1
// 首条位置: {"module":503,"inlinee":28625}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugColor::as_array
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":503,"record_offset":48988,"parent_record_offset":48960,"ancestor_record_offset":48960,"inlinee":28621,"source_line":208,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::debug_viz::impl$5::emit::closure$1<country_core::utils::debug_viz::impl$5::show_for::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 484
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":56040,"rva":9466240,"va":5378175360}
// 编译实例 RVAs: 0x907180
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-2496806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378175360,"raw_hex":"e8dae6d9ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5378175489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2497378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378175360,"raw_hex":"e89ee4d9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378176077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9453283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378175360,"raw_hex":"e81dc16fff","size":5,"soft_fail":false,"target_va":5368722192,"va":5378175470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: debug_viz.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38881517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378175360,"raw_hex":"e8ed485102","size":5,"soft_fail":false,"target_va":5417057056,"va":5378175534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38444755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378175360,"raw_hex":"e8d39e4a02","size":5,"soft_fail":false,"target_va":5416620769,"va":5378176009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38445351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378175360,"raw_hex":"e827a14a02","size":5,"soft_fail":false,"target_va":5416620794,"va":5378175438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t762878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378175360,"raw_hex":"e8fea30b00","size":5,"soft_fail":false,"target_va":5378938736,"va":5378175853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":57712,"parent_record_offset":56040,"ancestor_record_offset":56040,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":57528,"parent_record_offset":56040,"ancestor_record_offset":56040,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":56268,"parent_record_offset":56040,"ancestor_record_offset":56040,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":57796,"parent_record_offset":56040,"ancestor_record_offset":56040,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::line_topology
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":503,"record_offset":57500,"parent_record_offset":56040,"ancestor_record_offset":56040,"inlinee":28623,"source_line":284,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":503,"record_offset":57444,"parent_record_offset":56040,"ancestor_record_offset":56040,"inlinee":28622,"source_line":234,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::set_ui_pass
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":503,"record_offset":57472,"parent_record_offset":56040,"ancestor_record_offset":56040,"inlinee":28630,"source_line":256,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":56680,"parent_record_offset":56656,"ancestor_record_offset":56040,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":56704,"parent_record_offset":56680,"ancestor_record_offset":56040,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":57980,"parent_record_offset":57956,"ancestor_record_offset":56040,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":58004,"parent_record_offset":57980,"ancestor_record_offset":56040,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":56728,"parent_record_offset":56704,"ancestor_record_offset":56040,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":58028,"parent_record_offset":58004,"ancestor_record_offset":56040,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":56656,"parent_record_offset":56476,"ancestor_record_offset":56040,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":57956,"parent_record_offset":57868,"ancestor_record_offset":56040,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":57556,"parent_record_offset":57528,"ancestor_record_offset":56040,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":56420,"parent_record_offset":56396,"ancestor_record_offset":56040,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":57896,"parent_record_offset":57868,"ancestor_record_offset":56040,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":57868,"parent_record_offset":57844,"ancestor_record_offset":56040,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":56476,"parent_record_offset":56444,"ancestor_record_offset":56040,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":56444,"parent_record_offset":56420,"ancestor_record_offset":56040,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":57844,"parent_record_offset":57820,"ancestor_record_offset":56040,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":56512,"parent_record_offset":56476,"ancestor_record_offset":56040,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":57736,"parent_record_offset":57712,"ancestor_record_offset":56040,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":56360,"parent_record_offset":56296,"ancestor_record_offset":56040,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":56332,"parent_record_offset":56296,"ancestor_record_offset":56040,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":57760,"parent_record_offset":57736,"ancestor_record_offset":56040,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":56396,"parent_record_offset":56360,"ancestor_record_offset":56040,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":56296,"parent_record_offset":56268,"ancestor_record_offset":56040,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":503,"record_offset":56536,"parent_record_offset":56512,"ancestor_record_offset":56040,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":503,"record_offset":56616,"parent_record_offset":56560,"ancestor_record_offset":56040,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":503,"record_offset":56560,"parent_record_offset":56536,"ancestor_record_offset":56040,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":503,"record_offset":56588,"parent_record_offset":56560,"ancestor_record_offset":56040,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":503,"record_offset":57924,"parent_record_offset":57896,"ancestor_record_offset":56040,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":503,"record_offset":56784,"parent_record_offset":56360,"ancestor_record_offset":56040,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":56808,"parent_record_offset":56784,"ancestor_record_offset":56040,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":57820,"parent_record_offset":57796,"ancestor_record_offset":56040,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
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
// 完整结构及原始类型记录: debug_viz.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::utils::debug_viz::DebugColor
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::utils::debug_viz::DebugLine
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 12; 过程引用数: 16
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::debug_viz::DebugLine::emit<country_core::utils::debug_viz::impl$5::show::closure_env$0>", "constructor": "country_core::utils::debug_viz::DebugLine::emit", "arguments": ["country_core::utils::debug_viz::impl$5::show::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::debug_viz::DebugLine::emit<country_core::utils::debug_viz::impl$5::show_for::closure_env$0>", "constructor": "country_core::utils::debug_viz::DebugLine::emit", "arguments": ["country_core::utils::debug_viz::impl$5::show_for::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["country_core::utils::debug_viz::DebugLine", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::utils::debug_viz::DebugLine>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::utils::debug_viz::DebugLine"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::utils::debug_viz::DebugLine", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> >", "constructor": "std::sync::poison::mutex::Mutex", "arguments": ["alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once_lock::impl$0::get_or_init::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> >,std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> > (*)()>", "constructor": "std::sync::once_lock::impl$0::get_or_init::closure_env$0", "arguments": ["std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> >", "std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> > (*)()"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once_lock::impl$0::initialize::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> >,std::sync::once_lock::impl$0::get_or_init::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> >,std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> > (*)()>,never$>", "constructor": "std::sync::once_lock::impl$0::initialize::closure_env$0", "arguments": ["std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> >", "std::sync::once_lock::impl$0::get_or_init::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> >,std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> > (*)()>", "never$"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once::impl$2::call_once_force::closure$0<std::sync::once_lock::impl$0::initialize::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> >,std::sync::once_lock::impl$0::get_or_init::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> >,std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> > (*)()>,never$> >", "constructor": "std::sync::once::impl$2::call_once_force::closure$0", "arguments": ["std::sync::once_lock::impl$0::initialize::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> >,std::sync::once_lock::impl$0::get_or_init::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> >,std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> > (*)()>,never$>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once_lock::OnceLock::initialize<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> >,std::sync::once_lock::impl$0::get_or_init::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> >,std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> > (*)()>,never$>", "constructor": "std::sync::once_lock::OnceLock::initialize", "arguments": ["std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> >", "std::sync::once_lock::impl$0::get_or_init::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> >,std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> > (*)()>", "never$"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once::impl$2::call_once_force::closure_env$0<std::sync::once_lock::impl$0::initialize::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> >,std::sync::once_lock::impl$0::get_or_init::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> >,std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> > (*)()>,never$> >", "constructor": "std::sync::once::impl$2::call_once_force::closure_env$0", "arguments": ["std::sync::once_lock::impl$0::initialize::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> >,std::sync::once_lock::impl$0::get_or_init::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> >,std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> > (*)()>,never$>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<std::sync::once::impl$2::call_once_force::closure_env$0<std::sync::once_lock::impl$0::initialize::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> >,std::sync::once_lock::impl$0::get_or_init::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> >,std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> > (*)()>,never$> >,tuple$<ref$<std::sync::once::OnceState> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["std::sync::once::impl$2::call_once_force::closure_env$0<std::sync::once_lock::impl$0::initialize::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> >,std::sync::once_lock::impl$0::get_or_init::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> >,std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugLine,alloc::alloc::Global> > (*)()>,never$> >", "tuple$<ref$<std::sync::once::OnceState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::utils::debug_viz::DebugRectangle2d
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 1
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::utils::debug_viz::DebugRectangle3d
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 8; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::debug_viz::DebugRectangle3d::emit<country_core::utils::debug_viz::impl$3::show_for::closure_env$0>", "constructor": "country_core::utils::debug_viz::DebugRectangle3d::emit", "arguments": ["country_core::utils::debug_viz::impl$3::show_for::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::debug_viz::DebugRectangle3d::emit<country_core::utils::debug_viz::impl$3::show::closure_env$0>", "constructor": "country_core::utils::debug_viz::DebugRectangle3d::emit", "arguments": ["country_core::utils::debug_viz::impl$3::show::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::utils::debug_viz::DebugSphere
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 10; 过程引用数: 12
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::debug_viz::DebugSphere::emit<country_core::utils::debug_viz::impl$1::show::closure_env$0>", "constructor": "country_core::utils::debug_viz::DebugSphere::emit", "arguments": ["country_core::utils::debug_viz::impl$1::show::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::debug_viz::DebugSphere::emit<country_core::utils::debug_viz::impl$1::show_for::closure_env$0>", "constructor": "country_core::utils::debug_viz::DebugSphere::emit", "arguments": ["country_core::utils::debug_viz::impl$1::show_for::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["country_core::utils::debug_viz::DebugSphere", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::utils::debug_viz::DebugSphere", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> >", "constructor": "std::sync::poison::mutex::Mutex", "arguments": ["alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once_lock::impl$0::get_or_init::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> >,std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> > (*)()>", "constructor": "std::sync::once_lock::impl$0::get_or_init::closure_env$0", "arguments": ["std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> >", "std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> > (*)()"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once_lock::impl$0::initialize::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> >,std::sync::once_lock::impl$0::get_or_init::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> >,std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> > (*)()>,never$>", "constructor": "std::sync::once_lock::impl$0::initialize::closure_env$0", "arguments": ["std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> >", "std::sync::once_lock::impl$0::get_or_init::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> >,std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> > (*)()>", "never$"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once::impl$2::call_once_force::closure$0<std::sync::once_lock::impl$0::initialize::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> >,std::sync::once_lock::impl$0::get_or_init::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> >,std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> > (*)()>,never$> >", "constructor": "std::sync::once::impl$2::call_once_force::closure$0", "arguments": ["std::sync::once_lock::impl$0::initialize::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> >,std::sync::once_lock::impl$0::get_or_init::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> >,std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> > (*)()>,never$>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once_lock::OnceLock::initialize<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> >,std::sync::once_lock::impl$0::get_or_init::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> >,std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> > (*)()>,never$>", "constructor": "std::sync::once_lock::OnceLock::initialize", "arguments": ["std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> >", "std::sync::once_lock::impl$0::get_or_init::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> >,std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> > (*)()>", "never$"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once::impl$2::call_once_force::closure_env$0<std::sync::once_lock::impl$0::initialize::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> >,std::sync::once_lock::impl$0::get_or_init::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> >,std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> > (*)()>,never$> >", "constructor": "std::sync::once::impl$2::call_once_force::closure_env$0", "arguments": ["std::sync::once_lock::impl$0::initialize::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> >,std::sync::once_lock::impl$0::get_or_init::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> >,std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> > (*)()>,never$>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<std::sync::once::impl$2::call_once_force::closure_env$0<std::sync::once_lock::impl$0::initialize::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> >,std::sync::once_lock::impl$0::get_or_init::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> >,std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> > (*)()>,never$> >,tuple$<ref$<std::sync::once::OnceState> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["std::sync::once::impl$2::call_once_force::closure_env$0<std::sync::once_lock::impl$0::initialize::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> >,std::sync::once_lock::impl$0::get_or_init::closure_env$0<std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> >,std::sync::poison::mutex::Mutex<alloc::vec::Vec<country_core::utils::debug_viz::DebugSphere,alloc::alloc::Global> > (*)()>,never$> >", "tuple$<ref$<std::sync::once::OnceState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::utils::debug_viz::DebugTransform
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: debug_viz.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: debug_viz.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugColor::as_array
// 信息 ID: 01a47eac81a52d25bd85d766
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::debug_viz::impl$3::emit::closure$0<country_core::utils::debug_viz::impl$3::show::closure_env$0>; module=503, inline_record=46680; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::debug_viz::impl$3::emit::closure$1<country_core::utils::debug_viz::impl$3::show::closure_env$0>; module=503, inline_record=47852; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::debug_viz::DebugLine::emit<country_core::utils::debug_viz::impl$5::show::closure_env$0>; module=503, inline_record=48988; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::debug_viz::DebugLine::emit<country_core::utils::debug_viz::impl$5::show_for::closure_env$0>; module=503, inline_record=52732; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::debug_viz::impl$4::show::closure$0; module=541, inline_record=8508; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugLine::color
// 信息 ID: 42bf5995915ed41ef5e2adaa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x28c0046 -> 0xa76380 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug; module=550, inline_record=28164; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug; module=550, inline_record=28996; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug; module=550, inline_record=31628; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::ui_bulldozer::wall_eraser_bulldoze; module=577, inline_record=40744; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::structure_outline::structure_outline::closure$0; module=583, inline_record=16416; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::visualize_colliders::visualize_shape; module=1681, inline_record=38240; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::visualize_colliders::visualize_shape; module=1681, inline_record=38572; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::visualize_colliders::visualize_shape; module=1681, inline_record=39684; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugLine::dashed
// 信息 ID: df481f8fccc7e75d9a7bd798
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 10 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x858fe6 -> 0xa763b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x85c553 -> 0xa763b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x860ea3 -> 0xa763b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::blueprint::show_max_wall_y @ 0x8ec9fc -> 0xa763b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::set::Iter<country_core::resources::walls::decorator::DecoratorLocator>,country_core::resources::walls::decorator::DecoratorLocator,tuple$<>,core::iter::traits::iterator::Iterator::all::check::closure_env$0<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::blueprint::validate_blueprints::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tuple$<>,tuple$<> > > > @ 0x15094d6 -> 0xa763b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset @ 0x2170984 -> 0xa763b0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::ui_clutter::ui_autoclutter::ui_convert_autoclutter_to_decoclutter; module=551, inline_record=52648; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_clutter::ui_autoclutter::ui_convert_autoclutter_to_decoclutter; module=551, inline_record=52732; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_move_shapes::ui_move_shapes; module=570, inline_record=31368; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_move_shapes::ui_move_shapes; module=570, inline_record=37000; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_move_shapes::ui_move_shapes; module=570, inline_record=37432; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::shape_placement_params; module=570, inline_record=83328; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::shape_placement_params; module=570, inline_record=83800; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::shape_placement_params; module=570, inline_record=83940; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugLine::emit<country_core::utils::debug_viz::impl$5::show::closure_env$0>
// 信息 ID: a0d6650c6de32440fa7b59ab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::debug_viz::DebugLine::emit -> ["country_core::utils::debug_viz::impl$5::show::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 24 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::walls::decorator_outline::DecoratorCollisionOutline::display @ 0x90b805 -> 0x906710 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::debug_viz::drain_global_debug_spheres @ 0xa75ef4 -> 0x906710 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::debug_viz::DebugLine::show @ 0xa76370 -> 0x906710 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] country_core::resources::audio_resources::AudioCore::visualize_sounds @ 0xa78864 -> 0x906710 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::audio_resources::AudioCore::visualize_sounds @ 0xa788fd -> 0x906710 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::audio_resources::AudioCore::visualize_sounds @ 0xa789a8 -> 0x906710 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug @ 0xac3241 -> 0x906710 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug @ 0xac335f -> 0x906710 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug @ 0xac345a -> 0x906710 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug @ 0xac3794 -> 0x906710 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_clutter::ui_autoclutter::ui_convert_autoclutter_to_decoclutter @ 0xac9db0 -> 0x906710 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_clutter::ui_autoclutter::ui_convert_autoclutter_to_decoclutter @ 0xac9c04 -> 0x906710 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_move_shapes::ui_move_shapes @ 0xb5b4a8 -> 0x906710 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_move_shapes::ui_move_shapes @ 0xb5c2a7 -> 0x906710 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_move_shapes::ui_move_shapes @ 0xb5c3fc -> 0x906710 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle @ 0xb62b57 -> 0x906710 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugLine::emit<country_core::utils::debug_viz::impl$5::show_for::closure_env$0>
// 信息 ID: 9c7a723c0dee54044a3068d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::debug_viz::DebugLine::emit -> ["country_core::utils::debug_viz::impl$5::show_for::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui_elevate_wall_group::ui_elevate_wall_group @ 0x934734 -> 0x906c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::debug_viz::DebugLine::show_for @ 0xa763e0 -> 0x906c40 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle @ 0xb7dbef -> 0x906c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_focus_mode_shape_height::ui_focus_mode_shape_height @ 0xbd2110 -> 0x906c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_wall_transform_gizmos::wall_rotation_gizmo @ 0x1abafaf -> 0x906c40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::terrain::ui_terrain_focus_mode::ui_terrain_focus_mode @ 0x1abe1e0 -> 0x906c40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugLine::line_width
// 信息 ID: ed71b447e046e1dbefff0a71
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 10 条；内联宿主 12 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x858ffe -> 0xa76300 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x85c544 -> 0xa76300 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x860e98 -> 0xa76300 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::blueprint::show_max_wall_y @ 0x8eca16 -> 0xa76300 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::set::Iter<country_core::resources::walls::decorator::DecoratorLocator>,country_core::resources::walls::decorator::DecoratorLocator,tuple$<>,core::iter::traits::iterator::Iterator::all::check::closure_env$0<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::blueprint::validate_blueprints::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tuple$<>,tuple$<> > > > @ 0x15094f2 -> 0xa76300 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset @ 0x2170777 -> 0xa76300 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset @ 0x21709b5 -> 0xa76300 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation @ 0x296d404 -> 0xa76300 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation @ 0x296d4e7 -> 0xa76300 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation @ 0x296d58c -> 0xa76300 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::resources::walls::decorator_outline::DecoratorCollisionOutline::display; module=503, inline_record=108320; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_elevate_wall_group::ui_elevate_wall_group; module=510, inline_record=69596; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::audio_resources::AudioCore::visualize_sounds; module=541, inline_record=37092; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::audio_resources::AudioCore::visualize_sounds; module=541, inline_record=37444; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::audio_resources::AudioCore::visualize_sounds; module=541, inline_record=37828; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_clutter::ui_autoclutter::ui_convert_autoclutter_to_decoclutter; module=551, inline_record=52844; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=574, inline_record=21980; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::publish_public_wall_state::publish_public_wall_state; module=578, inline_record=86644; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugLine::new
// 信息 ID: ae22f2625a4fcaa773955286
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 18 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x854a15 -> 0xa76330 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x858fd6 -> 0xa76330 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x85a122 -> 0xa76330 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x85acac -> 0xa76330 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x85c52a -> 0xa76330 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x860e8a -> 0xa76330 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_rotate_lantern::ui_rotate_decorator @ 0x864c9a -> 0xa76330 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::blueprint::show_max_wall_y @ 0x8ec9ea -> 0xa76330 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::set::Iter<country_core::resources::walls::decorator::DecoratorLocator>,country_core::resources::walls::decorator::DecoratorLocator,tuple$<>,core::iter::traits::iterator::Iterator::all::check::closure_env$0<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::blueprint::validate_blueprints::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tuple$<>,tuple$<> > > > @ 0x15094c2 -> 0xa76330 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset @ 0x2170746 -> 0xa76330 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset @ 0x217095b -> 0xa76330 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x28c0021 -> 0xa76330 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$0::call<tuple$<utils::geometry::ray::Ray,f32>,system_critters::wren_boids::sim_wren_boids::closure_env$3> @ 0x293323c -> 0xa76330 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren::update_wrens @ 0x293d981 -> 0xa76330 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation @ 0x296d3ee -> 0xa76330 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation @ 0x296d4d1 -> 0xa76330 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug; module=550, inline_record=29108; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle; module=570, inline_record=94924; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::ui_bulldozer::wall_eraser_bulldoze; module=577, inline_record=40716; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::publish_public_wall_state::publish_public_wall_state; module=578, inline_record=86616; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugLine::show
// 信息 ID: 95bddae049a7470b3de5e883
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 13 条；内联宿主 28 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x854a4f -> 0xa76370 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x85acc7 -> 0xa76370 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x860eb2 -> 0xa76370 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_rotate_lantern::ui_rotate_decorator @ 0x864ca6 -> 0xa76370 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::blueprint::show_max_wall_y @ 0x8eca27 -> 0xa76370 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::set::Iter<country_core::resources::walls::decorator::DecoratorLocator>,country_core::resources::walls::decorator::DecoratorLocator,tuple$<>,core::iter::traits::iterator::Iterator::all::check::closure_env$0<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::blueprint::validate_blueprints::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tuple$<>,tuple$<> > > > @ 0x1509509 -> 0xa76370 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x28c005a -> 0xa76370 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$0::call<tuple$<utils::geometry::ray::Ray,f32>,system_critters::wren_boids::sim_wren_boids::closure_env$3> @ 0x293324b -> 0xa76370 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren::update_wrens @ 0x293d991 -> 0xa76370 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation @ 0x296d414 -> 0xa76370 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation @ 0x296d4f4 -> 0xa76370 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation @ 0x296d599 -> 0xa76370 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren_boids::sim_wren_boids::closure$3 @ 0x2b29044 -> 0xa76370 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::resources::walls::decorator_outline::DecoratorCollisionOutline::display; module=503, inline_record=108348; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::debug_viz::drain_global_debug_spheres; module=541, inline_record=13364; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::audio_resources::AudioCore::visualize_sounds; module=541, inline_record=37120; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::audio_resources::AudioCore::visualize_sounds; module=541, inline_record=37472; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::audio_resources::AudioCore::visualize_sounds; module=541, inline_record=37856; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug; module=550, inline_record=28192; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug; module=550, inline_record=29024; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug; module=550, inline_record=29136; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugLine::show_for
// 信息 ID: 843cae6c0a4451b79f8d515b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x859046 -> 0xa763e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x85a16f -> 0xa763e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x85c5b1 -> 0xa763e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset @ 0x21707c3 -> 0xa763e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::ui_stone_foundation_offset::ui_stone_foundation_offset @ 0x21709fa -> 0xa763e0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::ui_elevate_wall_group::ui_elevate_wall_group; module=510, inline_record=69624; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=574, inline_record=22036; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_focus_mode_shape_height::ui_focus_mode_shape_height; module=582, inline_record=17868; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_wall_transform_gizmos::wall_rotation_gizmo; module=1762, inline_record=36680; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::terrain::ui_terrain_focus_mode::ui_terrain_focus_mode; module=1762, inline_record=56952; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugRectangle2d::color
// 信息 ID: b6207253df2cf63d97eae333
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug; module=550, inline_record=30860; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugRectangle2d::new
// 信息 ID: 918f9b669c5f2ded9f3ae2e1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug; module=550, inline_record=27540; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug; module=550, inline_record=30832; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugRectangle2d::show
// 信息 ID: b02eeddc95ef406ca562695e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug @ 0xac3151 -> 0xa754e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug @ 0xac3671 -> 0xa754e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugRectangle3d::color
// 信息 ID: ae2480f1dd16cb8779d682e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x28bf5d2 -> 0xa75c30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x28bf1ee -> 0xa75c30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x28bfb53 -> 0xa75c30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates @ 0x28c9d2b -> 0xa75c30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugRectangle3d::dashed_line
// 信息 ID: cb761000effb45b74af76ba1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui_clutter::ui_focus_decoclutter::ui_focus_decoclutter; module=553, inline_record=34732; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugRectangle3d::emit<country_core::utils::debug_viz::impl$3::show::closure_env$0>
// 信息 ID: bc76fa6f136eba265d6d33ee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::debug_viz::DebugRectangle3d::emit -> ["country_core::utils::debug_viz::impl$3::show::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::debug_viz::DebugRectangle3d::show @ 0xa75c20 -> 0x905710 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugRectangle3d::emit<country_core::utils::debug_viz::impl$3::show_for::closure_env$0>
// 信息 ID: 6738b070f9a87b6e4a30f21f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::debug_viz::DebugRectangle3d::emit -> ["country_core::utils::debug_viz::impl$3::show_for::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui_clutter::ui_focus_decoclutter::ui_focus_decoclutter @ 0xad60a4 -> 0x904c10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugRectangle3d::line_width
// 信息 ID: 65587157c526370d4453f770
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x28bf5f5 -> 0xa75b90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x28bfb6f -> 0xa75b90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugRectangle3d::new
// 信息 ID: 1723c69d696dfb027e2e3b0f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x28bf5a6 -> 0xa75bf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x28bf1c2 -> 0xa75bf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x28bfb2e -> 0xa75bf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates @ 0x28c9d16 -> 0xa75bf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugRectangle3d::show
// 信息 ID: b7b9576d20228d6b2fd22d68
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x28bf610 -> 0xa75c20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x28bf209 -> 0xa75c20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x28bfb83 -> 0xa75c20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates @ 0x28c9d39 -> 0xa75c20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugRectangle3d::show_for
// 信息 ID: ce49ecb90e5e3fc645d801e0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui_clutter::ui_focus_decoclutter::ui_focus_decoclutter; module=553, inline_record=34848; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugSphere::color
// 信息 ID: b44c9497b07855399c3d869b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::wren::update_wrens @ 0x293e736 -> 0xa75470 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::collision::visualize_colliders::visualize_shape; module=1681, inline_record=40112; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugSphere::emit<country_core::utils::debug_viz::impl$1::show::closure_env$0>
// 信息 ID: c6819cac2358e86883168882
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::debug_viz::DebugSphere::emit -> ["country_core::utils::debug_viz::impl$1::show::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 10 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::walls::decorator_outline::DecoratorCollisionOutline::display @ 0x90b706 -> 0x904120 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::debug_viz::DebugSphere::show @ 0xa75460 -> 0x904120 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] country_core::utils::debug_viz::drain_global_debug_spheres @ 0xa75d90 -> 0x904120 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::audio_resources::AudioCore::visualize_sounds @ 0xa78734 -> 0x904120 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::debug_viz_threshold_curve::debug_viz_threshold_curve @ 0xb9a605 -> 0x904120 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::debug_viz_threshold_curve::debug_viz_threshold_curve @ 0xb9a65e -> 0x904120 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::debug_viz_threshold_curve::debug_viz_threshold_curve @ 0xb9a6b3 -> 0x904120 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::debug_viz_threshold_curve::debug_viz_threshold_curve @ 0xb9a700 -> 0x904120 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::snapping::snapping_maintain::snapping_signifiers @ 0xc0c8aa -> 0x904120 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::visualize_colliders::visualize_shape @ 0x180c0c9 -> 0x904120 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugSphere::emit<country_core::utils::debug_viz::impl$1::show_for::closure_env$0>
// 信息 ID: 3eab62179df0596b779a8744
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::debug_viz::DebugSphere::emit -> ["country_core::utils::debug_viz::impl$1::show_for::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::debug_viz::DebugSphere::show_for @ 0xa754d0 -> 0x9043b0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_move_shapes::ui_move_shapes @ 0xb5a3c1 -> 0x9043b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugSphere::line_width
// 信息 ID: 050354b6bb86ebce740c0fef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::wall_state::debug_viz_threshold_curve::debug_viz_threshold_curve; module=577, inline_record=6520; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::debug_viz_threshold_curve::debug_viz_threshold_curve; module=577, inline_record=6692; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::debug_viz_threshold_curve::debug_viz_threshold_curve; module=577, inline_record=6864; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugSphere::new
// 信息 ID: 839e55e19825612bd5a01ce7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x854481 -> 0xa75430 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x85488c -> 0xa75430 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x859363 -> 0xa75430 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_rotate_lantern::ui_rotate_decorator @ 0x864912 -> 0xa75430 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren::update_wrens @ 0x293e704 -> 0xa75430 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::wall_state::debug_viz_threshold_curve::debug_viz_threshold_curve; module=577, inline_record=6460; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugSphere::radius
// 信息 ID: 4cb179eafa2d95f17f852c92
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x8544f8 -> 0xa754b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x854904 -> 0xa754b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x859386 -> 0xa754b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren::update_wrens @ 0x293e712 -> 0xa754b0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::resources::walls::decorator_outline::DecoratorCollisionOutline::display; module=503, inline_record=106964; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::audio_resources::AudioCore::visualize_sounds; module=541, inline_record=36628; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_move_shapes::ui_move_shapes; module=570, inline_record=23640; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::shape_placement_params; module=570, inline_record=83244; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::shape_placement_params; module=570, inline_record=83856; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::ui_bulldozer::wall_eraser_bulldoze; module=577, inline_record=35984; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::ui_bulldozer::wall_eraser_bulldoze; module=577, inline_record=41284; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::snapping::snapping_maintain::snapping_signifiers; module=590, inline_record=26616; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugSphere::show
// 信息 ID: 86bf45a7a4c4f799dc430015
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 13 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x854532 -> 0xa75460 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x85493e -> 0xa75460 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_rotate_lantern::ui_rotate_decorator @ 0x864939 -> 0xa75460 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren::update_wrens @ 0x293e746 -> 0xa75460 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::resources::walls::decorator_outline::DecoratorCollisionOutline::display; module=503, inline_record=106992; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::debug_viz::drain_global_debug_spheres; module=541, inline_record=11564; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::audio_resources::AudioCore::visualize_sounds; module=541, inline_record=36656; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::shape_placement_params; module=570, inline_record=83272; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::shape_placement_params; module=570, inline_record=83884; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::debug_viz_threshold_curve::debug_viz_threshold_curve; module=577, inline_record=6492; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::debug_viz_threshold_curve::debug_viz_threshold_curve; module=577, inline_record=6664; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::debug_viz_threshold_curve::debug_viz_threshold_curve; module=577, inline_record=6836; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugSphere::show_for
// 信息 ID: 91414c5513800d4f713fa7c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x8593b2 -> 0xa754d0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::ui_move_shapes::ui_move_shapes; module=570, inline_record=23696; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::DebugSphere::solid
// 信息 ID: 28d5bc6a113a0287005a4255
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x8544bb -> 0xa75490 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_stairs_viz::visualize_ui_stairs @ 0x8548c6 -> 0xa75490 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x85936e -> 0xa75490 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_rotate_lantern::ui_rotate_decorator @ 0x864927 -> 0xa75490 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::drain_global_debug_spheres
// 信息 ID: 49cfe5fe7e7da5a3086b6702
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 646d10ef38ea5178692bce9c: void (*)(bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0x283d01 -> 0xa75c60 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0x30c2b9 -> 0xa75c60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::impl$1::emit::closure$0<country_core::utils::debug_viz::impl$1::show::closure_env$0>
// 信息 ID: 729e187bdcff9e2b0a4b6c8a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::debug_viz::impl$1::emit::closure$0 -> ["country_core::utils::debug_viz::impl$1::show::closure_env$0"]; 字段布局 = unknown。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::debug_viz::impl$1::emit", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::debug_viz::impl$1::emit::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::utils::debug_viz::impl$1::emit::closure_env$0<country_core::utils::debug_viz::impl$1::show::closure_env$0>,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::utils::debug_viz::impl$1::emit::closure_env$0<country_core::utils::debug_viz::impl$1::show::closure_env$0>,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x90ba20 -> 0x904640 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::impl$1::show::closure$0
// 信息 ID: 1dda0d5d23dab85ec6e6635a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::debug_viz::impl$1::show", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::debug_viz::impl$1::show::closure_env$0 <- country_core::utils::debug_viz::DebugSphere::emit<country_core::utils::debug_viz::impl$1::show::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::debug_viz::impl$1::show::closure_env$0 <- country_core::utils::debug_viz::impl$1::emit::closure$0<country_core::utils::debug_viz::impl$1::show::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::debug_viz::impl$1::show::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::utils::debug_viz::impl$1::emit::closure_env$0<country_core::utils::debug_viz::impl$1::show::closure_env$0>,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] country_core::utils::debug_viz::DebugSphere::emit<country_core::utils::debug_viz::impl$1::show::closure_env$0>; module=503, inline_record=28892; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::impl$1::show_for::closure$0
// 信息 ID: fcb02c9b53496c470e962759
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::debug_viz::impl$1::show_for", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::debug_viz::impl$1::show_for::closure_env$0 <- country_core::utils::debug_viz::DebugSphere::emit<country_core::utils::debug_viz::impl$1::show_for::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::utils::debug_viz::DebugSphere::emit<country_core::utils::debug_viz::impl$1::show_for::closure_env$0>; module=503, inline_record=30840; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::impl$3::emit::closure$0<country_core::utils::debug_viz::impl$3::show::closure_env$0>
// 信息 ID: 97f00d059e082cc2133d3280
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::debug_viz::impl$3::emit::closure$0 -> ["country_core::utils::debug_viz::impl$3::show::closure_env$0"]; 字段布局 = unknown。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::debug_viz::impl$3::emit", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::debug_viz::impl$3::emit::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::utils::debug_viz::impl$3::emit::closure_env$0<country_core::utils::debug_viz::impl$3::show::closure_env$0>,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::debug_viz::impl$3::emit::closure_env$0 <- core::ptr::drop_in_place<country_core::utils::debug_viz::impl$3::emit::closure_env$0<country_core::utils::debug_viz::impl$3::show::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::utils::debug_viz::impl$3::emit::closure_env$0<country_core::utils::debug_viz::impl$3::show::closure_env$0>,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x90bb03 -> 0x9061d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::impl$3::emit::closure$1<country_core::utils::debug_viz::impl$3::show::closure_env$0>
// 信息 ID: d3ba6c3e2ff2bedbcd973625
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::debug_viz::impl$3::emit::closure$1 -> ["country_core::utils::debug_viz::impl$3::show::closure_env$0"]; 字段布局 = unknown。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::utils::debug_viz::impl$3::emit", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::debug_viz::impl$3::emit::closure_env$1 <- core::ops::function::FnOnce::call_once<country_core::utils::debug_viz::impl$3::emit::closure_env$1<country_core::utils::debug_viz::impl$3::show_for::closure_env$0>,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::utils::debug_viz::impl$3::emit::closure_env$1<country_core::utils::debug_viz::impl$3::show_for::closure_env$0>,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x90bad0 -> 0x9064a0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::impl$3::show::closure$0
// 信息 ID: f65e035291a57c4c4eeedadd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::debug_viz::impl$3::show", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::debug_viz::impl$3::show::closure_env$0 <- country_core::utils::debug_viz::DebugRectangle3d::emit<country_core::utils::debug_viz::impl$3::show::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::debug_viz::impl$3::show::closure_env$0 <- country_core::utils::debug_viz::impl$3::emit::closure$0<country_core::utils::debug_viz::impl$3::show::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::debug_viz::impl$3::show::closure_env$0 <- country_core::utils::debug_viz::impl$3::emit::closure$1<country_core::utils::debug_viz::impl$3::show::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::debug_viz::impl$3::show::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::utils::debug_viz::impl$3::emit::closure_env$0<country_core::utils::debug_viz::impl$3::show::closure_env$0>,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::debug_viz::impl$3::show::closure_env$0 <- core::ptr::drop_in_place<country_core::utils::debug_viz::impl$3::emit::closure_env$0<country_core::utils::debug_viz::impl$3::show::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 5 条。
// [PDB 内联宿主] country_core::utils::debug_viz::DebugRectangle3d::emit<country_core::utils::debug_viz::impl$3::show::closure_env$0>; module=503, inline_record=42812; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::debug_viz::DebugRectangle3d::emit<country_core::utils::debug_viz::impl$3::show::closure_env$0>; module=503, inline_record=44672; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::impl$3::show_for::closure$0
// 信息 ID: 9535d75ff931e3e6dc28713a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::debug_viz::impl$3::show_for", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::debug_viz::impl$3::show_for::closure_env$0 <- country_core::utils::debug_viz::DebugRectangle3d::emit<country_core::utils::debug_viz::impl$3::show_for::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::debug_viz::impl$3::show_for::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::utils::debug_viz::impl$3::emit::closure_env$1<country_core::utils::debug_viz::impl$3::show_for::closure_env$0>,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] country_core::utils::debug_viz::DebugRectangle3d::emit<country_core::utils::debug_viz::impl$3::show_for::closure_env$0>; module=503, inline_record=37528; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::debug_viz::DebugRectangle3d::emit<country_core::utils::debug_viz::impl$3::show_for::closure_env$0>; module=503, inline_record=39376; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::impl$4::show::closure$0
// 信息 ID: 6f63af13cfbca0efa0d01f68
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::debug_viz::impl$4::show", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::debug_viz::impl$4::show::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::utils::debug_viz::impl$4::show::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::debug_viz::impl$4::show::closure_env$0 <- core::ptr::drop_in_place<country_core::utils::debug_viz::impl$4::show::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::utils::debug_viz::impl$4::show::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0xa7c563 -> 0xa75860 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::impl$5::emit::closure$0
// 信息 ID: de65e7a607c171640f6d8aff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::debug_viz::impl$5::emit", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::debug_viz::DebugLine::emit<country_core::utils::debug_viz::impl$5::show::closure_env$0>; module=503, inline_record=48960; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::debug_viz::DebugLine::emit<country_core::utils::debug_viz::impl$5::show_for::closure_env$0>; module=503, inline_record=52704; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::impl$5::emit::closure$1<country_core::utils::debug_viz::impl$5::show_for::closure_env$0>
// 信息 ID: 8bed91d082e88af3a7051056
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::utils::debug_viz::impl$5::emit::closure$1 -> ["country_core::utils::debug_viz::impl$5::show_for::closure_env$0"]; 字段布局 = unknown。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::utils::debug_viz::impl$5::emit", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::debug_viz::impl$5::emit::closure_env$1 <- core::ops::function::FnOnce::call_once<country_core::utils::debug_viz::impl$5::emit::closure_env$1<country_core::utils::debug_viz::impl$5::show::closure_env$0>,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::debug_viz::impl$5::emit::closure_env$1 <- core::ptr::drop_in_place<country_core::utils::debug_viz::impl$5::emit::closure_env$1<country_core::utils::debug_viz::impl$5::show::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::debug_viz::impl$5::emit::closure_env$1 <- core::ptr::drop_in_place<alloc::sync::ArcInner<country_core::utils::debug_viz::impl$5::emit::closure_env$1<country_core::utils::debug_viz::impl$5::show::closure_env$0> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 3 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::utils::debug_viz::impl$5::emit::closure_env$1<country_core::utils::debug_viz::impl$5::show::closure_env$0>,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x90bbb4 -> 0x907180 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::impl$5::show::closure$0
// 信息 ID: 771dbcd6292d854cb38311c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::debug_viz::impl$5::show", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::debug_viz::impl$5::show::closure_env$0 <- country_core::utils::debug_viz::DebugLine::emit<country_core::utils::debug_viz::impl$5::show::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::debug_viz::impl$5::show::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::utils::debug_viz::impl$5::emit::closure_env$1<country_core::utils::debug_viz::impl$5::show::closure_env$0>,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::debug_viz::impl$5::show::closure_env$0 <- core::ptr::drop_in_place<country_core::utils::debug_viz::impl$5::emit::closure_env$1<country_core::utils::debug_viz::impl$5::show::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::debug_viz::impl$5::show::closure_env$0 <- core::ptr::drop_in_place<alloc::sync::ArcInner<country_core::utils::debug_viz::impl$5::emit::closure_env$1<country_core::utils::debug_viz::impl$5::show::closure_env$0> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] country_core::utils::debug_viz::DebugLine::emit<country_core::utils::debug_viz::impl$5::show::closure_env$0>; module=503, inline_record=50680; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::impl$5::show_for::closure$0
// 信息 ID: 9a679568ffc33ed505dfedb9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::debug_viz::impl$5::show_for", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::debug_viz::impl$5::show_for::closure_env$0 <- country_core::utils::debug_viz::DebugLine::emit<country_core::utils::debug_viz::impl$5::show_for::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::debug_viz::impl$5::show_for::closure_env$0 <- country_core::utils::debug_viz::impl$5::emit::closure$1<country_core::utils::debug_viz::impl$5::show_for::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] country_core::utils::debug_viz::DebugLine::emit<country_core::utils::debug_viz::impl$5::show_for::closure_env$0>; module=503, inline_record=54424; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::impl$6::fmt
// 信息 ID: 18827b2efd364dd5e939f316
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::debug_viz::impl$7::fmt
// 信息 ID: 616af753befff43f6708f7fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
