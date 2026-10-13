// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/panic_reporter/crash_service.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 6cbdda8bf644afd492aa9d052b7af3023a8e894f0eee8202534054960bef901f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: crash_service.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 6, 35, 177, 205, 230
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::crash_service::start
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":177,"record_offset":2104,"rva":5319984,"va":5374029104}
// 编译实例 RVAs: 0x512d30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::sync::Arc<std::thread::lifecycle::Packet<()>>>::drop_slow
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t10673248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374029104,"raw_hex":"e860dca200","size":5,"soft_fail":false,"target_va":5384703584,"va":5374030331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <alloc::sync::Arc<std::thread::thread::Inner, std::alloc::System>>::drop_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t10673588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374029104,"raw_hex":"e8b4dda200","size":5,"soft_fail":false,"target_va":5384703904,"va":5374030311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::lock_contended
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t42585651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374029104,"raw_hex":"e833ce8902","size":5,"soft_fail":false,"target_va":5416616112,"va":5374030456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::wake
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t42585799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374029104,"raw_hex":"e8c7ce8902","size":5,"soft_fail":false,"target_va":5416616304,"va":5374030500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::thread::builder::Builder>::name
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t10619860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374029104,"raw_hex":"e8d40ba200","size":5,"soft_fail":false,"target_va":5384649296,"va":5374029431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t10579682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374029104,"raw_hex":"e8e26ea100","size":5,"soft_fail":false,"target_va":5384609408,"va":5374029721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1649332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374029104,"raw_hex":"e8b42a1900","size":5,"soft_fail":false,"target_va":5375678688,"va":5374029351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1647821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374029104,"raw_hex":"e8cd241900","size":5,"soft_fail":false,"target_va":5375678704,"va":5374030878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: crash_service.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5307149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374029104,"raw_hex":"e8f304afff","size":5,"soft_fail":false,"target_va":5368722192,"va":5374029336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: crash_service.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4691500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374029104,"raw_hex":"e8d469b8ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5374030343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400aaf40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_service.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4621275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374029104,"raw_hex":"e8257cb9ff","size":5,"soft_fail":false,"target_va":5369409344,"va":5374030614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404379b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_service.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-898026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374029104,"raw_hex":"e8164cf2ff","size":5,"soft_fail":false,"target_va":5373131184,"va":5374029205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140437a40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: crash_service.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-897902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374029104,"raw_hex":"e8924cf2ff","size":5,"soft_fail":false,"target_va":5373131328,"va":5374029225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d66d9c
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_service.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42285873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374029104,"raw_hex":"e8313b8502","size":5,"soft_fail":false,"target_va":5416316316,"va":5374030438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42590244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374029104,"raw_hex":"e824e08902","size":5,"soft_fail":false,"target_va":5416620794,"va":5374030545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<std::thread::join_handle::JoinHandle<tuple$<> >,std::io::error::Error> > >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t17984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374029104,"raw_hex":"e840460000","size":5,"soft_fail":false,"target_va":5374048992,"va":5374031003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42589225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374029104,"raw_hex":"e829dc8902","size":5,"soft_fail":false,"target_va":5416619696,"va":5374030466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::functions::sleep
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10697929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374029104,"raw_hex":"e8c93ca300","size":5,"soft_fail":false,"target_va":5384728160,"va":5374030226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::lifecycle::spawn_unchecked
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-657286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374029104,"raw_hex":"e87af8f5ff","size":5,"soft_fail":false,"target_va":5373372208,"va":5374029489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*52130875(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5374029104,"raw_hex":"ff153b741b03","size":6,"soft_fail":false,"va":5374030295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":177,"record_offset":3840,"parent_record_offset":2104,"ancestor_record_offset":2104,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":177,"record_offset":3748,"parent_record_offset":2104,"ancestor_record_offset":2104,"inlinee":4688,"source_line":971,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":177,"record_offset":2660,"parent_record_offset":2104,"ancestor_record_offset":2104,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::inspect_err
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":177,"record_offset":4484,"parent_record_offset":2104,"ancestor_record_offset":2104,"inlinee":11771,"source_line":1016,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":177,"record_offset":5816,"parent_record_offset":2104,"ancestor_record_offset":2104,"inlinee":5676,"source_line":593,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":177,"record_offset":5844,"parent_record_offset":2104,"ancestor_record_offset":2104,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] std::sync::once_lock::OnceLock::set
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":177,"record_offset":2272,"parent_record_offset":2104,"ancestor_record_offset":2104,"inlinee":15282,"source_line":238,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":177,"record_offset":3288,"parent_record_offset":2104,"ancestor_record_offset":2104,"inlinee":4978,"source_line":490,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] std::thread::builder::Builder::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":177,"record_offset":3812,"parent_record_offset":2104,"ancestor_record_offset":2104,"inlinee":7549,"source_line":78,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] std::thread::builder::Builder::spawn
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":177,"record_offset":4384,"parent_record_offset":2104,"ancestor_record_offset":2104,"inlinee":7491,"source_line":185,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":4204,"parent_record_offset":4180,"ancestor_record_offset":2104,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":4228,"parent_record_offset":4204,"ancestor_record_offset":2104,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":8004,"parent_record_offset":7980,"ancestor_record_offset":2104,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":8028,"parent_record_offset":8004,"ancestor_record_offset":2104,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":4252,"parent_record_offset":4228,"ancestor_record_offset":2104,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":8052,"parent_record_offset":8028,"ancestor_record_offset":2104,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":4180,"parent_record_offset":4148,"ancestor_record_offset":2104,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":7980,"parent_record_offset":7952,"ancestor_record_offset":2104,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":177,"record_offset":4072,"parent_record_offset":4036,"ancestor_record_offset":2104,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":177,"record_offset":7952,"parent_record_offset":7928,"ancestor_record_offset":2104,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":177,"record_offset":4148,"parent_record_offset":4108,"ancestor_record_offset":2104,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":177,"record_offset":4108,"parent_record_offset":4072,"ancestor_record_offset":2104,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":177,"record_offset":7928,"parent_record_offset":7904,"ancestor_record_offset":2104,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":177,"record_offset":3924,"parent_record_offset":3876,"ancestor_record_offset":2104,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":177,"record_offset":3960,"parent_record_offset":3924,"ancestor_record_offset":2104,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":177,"record_offset":3996,"parent_record_offset":3960,"ancestor_record_offset":2104,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":177,"record_offset":3876,"parent_record_offset":3840,"ancestor_record_offset":2104,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":177,"record_offset":2960,"parent_record_offset":2928,"ancestor_record_offset":2104,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":177,"record_offset":2780,"parent_record_offset":2756,"ancestor_record_offset":2104,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":177,"record_offset":4036,"parent_record_offset":3996,"ancestor_record_offset":2104,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":177,"record_offset":4640,"parent_record_offset":4532,"ancestor_record_offset":2104,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":177,"record_offset":4664,"parent_record_offset":4640,"ancestor_record_offset":2104,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":177,"record_offset":4308,"parent_record_offset":3996,"ancestor_record_offset":2104,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":177,"record_offset":4332,"parent_record_offset":4308,"ancestor_record_offset":2104,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":177,"record_offset":2596,"parent_record_offset":2312,"ancestor_record_offset":2104,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":177,"record_offset":2992,"parent_record_offset":2960,"ancestor_record_offset":2104,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":177,"record_offset":4444,"parent_record_offset":4412,"ancestor_record_offset":2104,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":177,"record_offset":3668,"parent_record_offset":3640,"ancestor_record_offset":2104,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":177,"record_offset":2516,"parent_record_offset":2492,"ancestor_record_offset":2104,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":177,"record_offset":3360,"parent_record_offset":3328,"ancestor_record_offset":2104,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":177,"record_offset":2816,"parent_record_offset":2780,"ancestor_record_offset":2104,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":177,"record_offset":3572,"parent_record_offset":3540,"ancestor_record_offset":2104,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":177,"record_offset":3384,"parent_record_offset":3360,"ancestor_record_offset":2104,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":177,"record_offset":2540,"parent_record_offset":2516,"ancestor_record_offset":2104,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":177,"record_offset":2840,"parent_record_offset":2816,"ancestor_record_offset":2104,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":177,"record_offset":2872,"parent_record_offset":2780,"ancestor_record_offset":2104,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":177,"record_offset":4744,"parent_record_offset":4720,"ancestor_record_offset":2104,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":177,"record_offset":4696,"parent_record_offset":4532,"ancestor_record_offset":2104,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":177,"record_offset":4720,"parent_record_offset":4696,"ancestor_record_offset":2104,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":177,"record_offset":4768,"parent_record_offset":4744,"ancestor_record_offset":2104,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":177,"record_offset":4556,"parent_record_offset":4532,"ancestor_record_offset":2104,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":177,"record_offset":6140,"parent_record_offset":6116,"ancestor_record_offset":2104,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":177,"record_offset":3540,"parent_record_offset":3512,"ancestor_record_offset":2104,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":177,"record_offset":3512,"parent_record_offset":3484,"ancestor_record_offset":2104,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":177,"record_offset":2468,"parent_record_offset":2444,"ancestor_record_offset":2104,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":177,"record_offset":2420,"parent_record_offset":2388,"ancestor_record_offset":2104,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":177,"record_offset":2360,"parent_record_offset":2312,"ancestor_record_offset":2104,"inlinee":14249,"source_line":317,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::get_or_try_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":177,"record_offset":2388,"parent_record_offset":2360,"ancestor_record_offset":2104,"inlinee":14250,"source_line":398,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":177,"record_offset":2444,"parent_record_offset":2420,"ancestor_record_offset":2104,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::try_insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":177,"record_offset":2312,"parent_record_offset":2272,"ancestor_record_offset":2104,"inlinee":15283,"source_line":278,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":177,"record_offset":3640,"parent_record_offset":3448,"ancestor_record_offset":2104,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":177,"record_offset":3448,"parent_record_offset":3420,"ancestor_record_offset":2104,"inlinee":4982,"source_line":118,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":177,"record_offset":3420,"parent_record_offset":3288,"ancestor_record_offset":2104,"inlinee":4981,"source_line":719,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":177,"record_offset":3328,"parent_record_offset":3288,"ancestor_record_offset":2104,"inlinee":4979,"source_line":31,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":177,"record_offset":2492,"parent_record_offset":2468,"ancestor_record_offset":2104,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::builder::Builder::spawn_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":177,"record_offset":4412,"parent_record_offset":4384,"ancestor_record_offset":2104,"inlinee":7492,"source_line":254,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":177,"record_offset":3484,"parent_record_offset":3448,"ancestor_record_offset":2104,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::crash_service::start::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/crash_service.rs
//     evidence={"module":177,"record_offset":4532,"parent_record_offset":4484,"ancestor_record_offset":2104,"inlinee":15285,"source_line":99,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::crash_service::start::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/crash_service.rs
//     evidence={"module":177,"record_offset":5532,"parent_record_offset":5488,"ancestor_record_offset":2104,"inlinee":15286,"source_line":147,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::crash_service::start::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x42EA
// 首条位置: {"module":230,"inlinee":17130}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":20876,"parent_record_offset":20780,"ancestor_record_offset":20780,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":20820,"parent_record_offset":20780,"ancestor_record_offset":20780,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":20900,"parent_record_offset":20876,"ancestor_record_offset":20780,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":20844,"parent_record_offset":20820,"ancestor_record_offset":20780,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::crash_service::start::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 99
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3BB5
// 首条位置: {"module":177,"inlinee":15285}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":177,"record_offset":4640,"parent_record_offset":4532,"ancestor_record_offset":4532,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":177,"record_offset":4696,"parent_record_offset":4532,"ancestor_record_offset":4532,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":177,"record_offset":4556,"parent_record_offset":4532,"ancestor_record_offset":4532,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":177,"record_offset":4664,"parent_record_offset":4640,"ancestor_record_offset":4532,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":177,"record_offset":4580,"parent_record_offset":4556,"ancestor_record_offset":4532,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":177,"record_offset":4604,"parent_record_offset":4580,"ancestor_record_offset":4532,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":177,"record_offset":4744,"parent_record_offset":4720,"ancestor_record_offset":4532,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":177,"record_offset":4720,"parent_record_offset":4696,"ancestor_record_offset":4532,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":177,"record_offset":4768,"parent_record_offset":4744,"ancestor_record_offset":4532,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::crash_service::start::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 103
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x42EB
// 首条位置: {"module":230,"inlinee":17131}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":230,"record_offset":22780,"parent_record_offset":21184,"ancestor_record_offset":21184,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":230,"record_offset":22116,"parent_record_offset":21184,"ancestor_record_offset":21184,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":230,"record_offset":22192,"parent_record_offset":21184,"ancestor_record_offset":21184,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":21536,"parent_record_offset":21184,"ancestor_record_offset":21184,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicU8::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":21592,"parent_record_offset":21184,"ancestor_record_offset":21184,"inlinee":5580,"source_line":2842,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] std::sync::once_lock::OnceLock::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":230,"record_offset":22612,"parent_record_offset":21184,"ancestor_record_offset":21184,"inlinee":14251,"source_line":155,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":230,"record_offset":21656,"parent_record_offset":21184,"ancestor_record_offset":21184,"inlinee":4978,"source_line":490,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] std::thread::local::LocalKey::set
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":230,"record_offset":21312,"parent_record_offset":21184,"ancestor_record_offset":21184,"inlinee":6617,"source_line":573,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":230,"record_offset":21464,"parent_record_offset":21440,"ancestor_record_offset":21184,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":230,"record_offset":21440,"parent_record_offset":21416,"ancestor_record_offset":21184,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":230,"record_offset":21488,"parent_record_offset":21464,"ancestor_record_offset":21184,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":230,"record_offset":21364,"parent_record_offset":21336,"ancestor_record_offset":21184,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":230,"record_offset":22832,"parent_record_offset":22808,"ancestor_record_offset":21184,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":22036,"parent_record_offset":22008,"ancestor_record_offset":21184,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":22452,"parent_record_offset":22260,"ancestor_record_offset":21184,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":22708,"parent_record_offset":22684,"ancestor_record_offset":21184,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":21728,"parent_record_offset":21696,"ancestor_record_offset":21184,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::store
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":22912,"parent_record_offset":22888,"ancestor_record_offset":21184,"inlinee":4840,"source_line":2870,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":22544,"parent_record_offset":22512,"ancestor_record_offset":21184,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":21940,"parent_record_offset":21908,"ancestor_record_offset":21184,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":21752,"parent_record_offset":21728,"ancestor_record_offset":21184,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":21620,"parent_record_offset":21592,"ancestor_record_offset":21184,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":21560,"parent_record_offset":21536,"ancestor_record_offset":21184,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":22568,"parent_record_offset":22544,"ancestor_record_offset":21184,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":230,"record_offset":21908,"parent_record_offset":21880,"ancestor_record_offset":21184,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":230,"record_offset":21880,"parent_record_offset":21852,"ancestor_record_offset":21184,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::is_completed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":230,"record_offset":22660,"parent_record_offset":22636,"ancestor_record_offset":21184,"inlinee":14253,"source_line":265,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once_lock::OnceLock::initialized
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once_lock.rs
//     evidence={"module":230,"record_offset":22636,"parent_record_offset":22612,"ancestor_record_offset":21184,"inlinee":14252,"source_line":517,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":230,"record_offset":22260,"parent_record_offset":22220,"ancestor_record_offset":21184,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":230,"record_offset":22008,"parent_record_offset":21816,"ancestor_record_offset":21184,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":230,"record_offset":21816,"parent_record_offset":21788,"ancestor_record_offset":21184,"inlinee":4982,"source_line":118,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":230,"record_offset":21788,"parent_record_offset":21656,"ancestor_record_offset":21184,"inlinee":4981,"source_line":719,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":230,"record_offset":22220,"parent_record_offset":22192,"ancestor_record_offset":21184,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":230,"record_offset":21696,"parent_record_offset":21656,"ancestor_record_offset":21184,"inlinee":4979,"source_line":31,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":230,"record_offset":22512,"parent_record_offset":22220,"ancestor_record_offset":21184,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":230,"record_offset":22684,"parent_record_offset":22660,"ancestor_record_offset":21184,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":230,"record_offset":21852,"parent_record_offset":21816,"ancestor_record_offset":21184,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::initialize_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":230,"record_offset":21336,"parent_record_offset":21312,"ancestor_record_offset":21184,"inlinee":6613,"source_line":528,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::impl$5::set::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":230,"record_offset":21416,"parent_record_offset":21336,"ancestor_record_offset":21184,"inlinee":6615,"source_line":574,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::crash_service::IS_RECOVERY_THREAD::constant$0::closure$1
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":230,"record_offset":21384,"parent_record_offset":21364,"ancestor_record_offset":21184,"inlinee":15281,"source_line":71,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::crash_service::start::closure$2::impl$0::drop
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/crash_service.rs
//     evidence={"module":230,"record_offset":22888,"parent_record_offset":22864,"ancestor_record_offset":21184,"inlinee":4839,"source_line":120,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::crash_service::start::closure$2::impl$0::drop
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 120
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12E7
// 首条位置: {"module":6,"inlinee":4839}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU8::store
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":22912,"parent_record_offset":22888,"ancestor_record_offset":22888,"inlinee":4840,"source_line":2870,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":22936,"parent_record_offset":22912,"ancestor_record_offset":22888,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::crash_service::start::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3BB6
// 首条位置: {"module":177,"inlinee":15286}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":177,"record_offset":5640,"parent_record_offset":5532,"ancestor_record_offset":5532,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":177,"record_offset":5696,"parent_record_offset":5532,"ancestor_record_offset":5532,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":177,"record_offset":5556,"parent_record_offset":5532,"ancestor_record_offset":5532,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":177,"record_offset":5664,"parent_record_offset":5640,"ancestor_record_offset":5532,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":177,"record_offset":5580,"parent_record_offset":5556,"ancestor_record_offset":5532,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":177,"record_offset":5604,"parent_record_offset":5580,"ancestor_record_offset":5532,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":177,"record_offset":5744,"parent_record_offset":5720,"ancestor_record_offset":5532,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":177,"record_offset":5720,"parent_record_offset":5696,"ancestor_record_offset":5532,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":177,"record_offset":5768,"parent_record_offset":5744,"ancestor_record_offset":5532,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::crash_service::note_crash_started
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 169
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1F67
// 首条位置: {"module":35,"inlinee":8039}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":35,"record_offset":2000,"parent_record_offset":1976,"ancestor_record_offset":1976,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":35,"record_offset":2024,"parent_record_offset":2000,"ancestor_record_offset":1976,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::crash_service::crash_in_progress
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 182
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1F63
// 首条位置: {"module":35,"inlinee":8035}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU8::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":35,"record_offset":1748,"parent_record_offset":1724,"ancestor_record_offset":1724,"inlinee":5580,"source_line":2842,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":35,"record_offset":1772,"parent_record_offset":1748,"ancestor_record_offset":1724,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::crash_service::park_secondary_panic
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 190
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1F64
// 首条位置: {"module":35,"inlinee":8036}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::thread::local::LocalKey::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":35,"record_offset":1888,"parent_record_offset":1864,"ancestor_record_offset":1864,"inlinee":6616,"source_line":606,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":35,"record_offset":1936,"parent_record_offset":1912,"ancestor_record_offset":1864,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":35,"record_offset":1912,"parent_record_offset":1888,"ancestor_record_offset":1864,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::crash_service::mark_shutting_down
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 199
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3BDF
// 首条位置: {"module":205,"inlinee":15327}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":7772,"parent_record_offset":7748,"ancestor_record_offset":7748,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":7796,"parent_record_offset":7772,"ancestor_record_offset":7748,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::crash_service::is_shutting_down
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 203
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1F69
// 首条位置: {"module":35,"inlinee":8041}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":35,"record_offset":4796,"parent_record_offset":4768,"ancestor_record_offset":4768,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":35,"record_offset":4824,"parent_record_offset":4796,"ancestor_record_offset":4768,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00035.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::crash_service::request_and_wait
// 记录形式: compiled_procedure; instances=1
// 源码行: 214
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":177,"record_offset":536,"rva":5319360,"va":5374028480}
// 编译实例 RVAs: 0x512ac0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::lock_contended
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t42587249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374028480,"raw_hex":"e871d48902","size":5,"soft_fail":false,"target_va":5416616112,"va":5374028858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::wake
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t42587377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374028480,"raw_hex":"e8f1d48902","size":5,"soft_fail":false,"target_va":5416616304,"va":5374028922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant as core::ops::arith::Add<core::time::Duration>>::add
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10740115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374028480,"raw_hex":"e893e1a300","size":5,"soft_fail":false,"target_va":5384768864,"va":5374028744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::now
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t10619300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374028480,"raw_hex":"e8a409a200","size":5,"soft_fail":false,"target_va":5384648032,"va":5374028727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081b80
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: crash_service.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4788643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374028480,"raw_hex":"e85deeb6ff","size":5,"soft_fail":false,"target_va":5369240448,"va":5374029086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400bac40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: crash_service.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4554534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374028480,"raw_hex":"e8da80baff","size":5,"soft_fail":false,"target_va":5369474112,"va":5374028641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42590822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374028480,"raw_hex":"e866e28902","size":5,"soft_fail":false,"target_va":5416619696,"va":5374028869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::functions::sleep
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10699352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374028480,"raw_hex":"e85842a300","size":5,"soft_fail":false,"target_va":5384728160,"va":5374028803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x1400bac40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: crash_service.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-4554739","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5374028480,"raw_hex":"e90d80baff","size":5,"soft_fail":false,"target_va":5369474112,"va":5374028846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::gt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":177,"record_offset":1868,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":15279,"source_line":1446,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":177,"record_offset":1276,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":177,"record_offset":704,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":177,"record_offset":760,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicU8::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":177,"record_offset":1804,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":5580,"source_line":2842,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicU8::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":177,"record_offset":1748,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":4840,"source_line":2870,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":177,"record_offset":816,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":4978,"source_line":490,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":177,"record_offset":1892,"parent_record_offset":1868,"ancestor_record_offset":536,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":177,"record_offset":1304,"parent_record_offset":1276,"ancestor_record_offset":536,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":177,"record_offset":1196,"parent_record_offset":1168,"ancestor_record_offset":536,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":177,"record_offset":1588,"parent_record_offset":1400,"ancestor_record_offset":536,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":177,"record_offset":888,"parent_record_offset":856,"ancestor_record_offset":536,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":177,"record_offset":1680,"parent_record_offset":1648,"ancestor_record_offset":536,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":177,"record_offset":1100,"parent_record_offset":1068,"ancestor_record_offset":536,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":177,"record_offset":784,"parent_record_offset":760,"ancestor_record_offset":536,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":177,"record_offset":728,"parent_record_offset":704,"ancestor_record_offset":536,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":177,"record_offset":1612,"parent_record_offset":1588,"ancestor_record_offset":536,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":177,"record_offset":1704,"parent_record_offset":1680,"ancestor_record_offset":536,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$21::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":177,"record_offset":1972,"parent_record_offset":1948,"ancestor_record_offset":536,"inlinee":6433,"source_line":79,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":177,"record_offset":1068,"parent_record_offset":1040,"ancestor_record_offset":536,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":177,"record_offset":1040,"parent_record_offset":1012,"ancestor_record_offset":536,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":177,"record_offset":1400,"parent_record_offset":1364,"ancestor_record_offset":536,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":177,"record_offset":1168,"parent_record_offset":976,"ancestor_record_offset":536,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":177,"record_offset":976,"parent_record_offset":948,"ancestor_record_offset":536,"inlinee":4982,"source_line":118,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":177,"record_offset":948,"parent_record_offset":816,"ancestor_record_offset":536,"inlinee":4981,"source_line":719,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":177,"record_offset":1364,"parent_record_offset":1336,"ancestor_record_offset":536,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":177,"record_offset":856,"parent_record_offset":816,"ancestor_record_offset":536,"inlinee":4979,"source_line":31,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":177,"record_offset":1648,"parent_record_offset":1364,"ancestor_record_offset":536,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::time::windows::impl$20::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\time\windows.rs
//     evidence={"module":177,"record_offset":1948,"parent_record_offset":1924,"ancestor_record_offset":536,"inlinee":6431,"source_line":13,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":177,"record_offset":1012,"parent_record_offset":976,"ancestor_record_offset":536,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::time::impl$24::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\time.rs
//     evidence={"module":177,"record_offset":1924,"parent_record_offset":1868,"ancestor_record_offset":536,"inlinee":6429,"source_line":154,"archive":"_pdb/C/modules/00177.jsonl.gz"}
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
// 完整结构及原始类型记录: crash_service.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::panic_reporter::crash_service::CrashKind
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tiny_glade::panic_reporter::crash_service::CrashKind>", "constructor": "enum2$", "arguments": ["tiny_glade::panic_reporter::crash_service::CrashKind"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<enum2$<tiny_glade::panic_reporter::crash_service::CrashKind> >", "constructor": "core::option::Option", "arguments": ["enum2$<tiny_glade::panic_reporter::crash_service::CrashKind>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<enum2$<tiny_glade::panic_reporter::crash_service::CrashKind> > >", "constructor": "enum2$", "arguments": ["core::option::Option<enum2$<tiny_glade::panic_reporter::crash_service::CrashKind> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::poison::mutex::MutexGuard<enum2$<core::option::Option<enum2$<tiny_glade::panic_reporter::crash_service::CrashKind> > > >", "constructor": "std::sync::poison::mutex::MutexGuard", "arguments": ["enum2$<core::option::Option<enum2$<tiny_glade::panic_reporter::crash_service::CrashKind> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::poison::mutex::MutexGuard<enum2$<core::option::Option<enum2$<tiny_glade::panic_reporter::crash_service::CrashKind> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::poison::mutex::MutexGuard<enum2$<core::option::Option<enum2$<tiny_glade::panic_reporter::crash_service::CrashKind> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::panic_reporter::crash_service::IS_RECOVERY_THREAD
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<tiny_glade::panic_reporter::crash_service::IS_RECOVERY_THREAD::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<bool> > > > > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["tiny_glade::panic_reporter::crash_service::IS_RECOVERY_THREAD::constant$0::closure_env$1", "tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<bool> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: crash_service.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: crash_service.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 69; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenPlatformStairDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenPlatformStairDrawables>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenPlatformStairDrawables>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjWallHeightModeCache>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::FinishAdjWallHeightMode>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjWallHeightModeCache>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::FinishAdjWallHeightMode>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjWallHeightModeCache>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::FinishAdjWallHeightMode>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::TestCube>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::RecalculateStoneFoundationSurfaceCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::DeleteStoneFoundationSurfaceCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>", "bevy_ecs::system::commands::Commands", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::RecalculateStoneFoundationSurfaceCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::DeleteStoneFoundationSurfaceCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::RecalculateStoneFoundationSurfaceCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::DeleteStoneFoundationSurfaceCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::FollowMouse>,enum2$<core::option::Option<ref$<country_core::components::HoverAboveTerrain> > >,ref_mut$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::FollowMouse>,enum2$<core::option::Option<ref$<country_core::components::HoverAboveTerrain> > >,ref_mut$<utils::components::transform::Transform> >,tuple$<> >", "bevy_ecs::change_detection::ResMut<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::FollowMouse>,enum2$<core::option::Option<ref$<country_core::components::HoverAboveTerrain> > >,ref_mut$<utils::components::transform::Transform> >,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::wooden_platform_stairs::RecalculateWoodenGateStepsCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::gates::Gates>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::wooden_platform_stairs::RecalculateWoodenGateStepsCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::gates::Gates>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::wooden_platform_stairs::RecalculateWoodenGateStepsCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::gates::Gates>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::brush_preview::BrushPreview>,enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > > >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::brush_preview::OverwriteBrushPreview>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser> >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::brush_preview::BrushPreview>,enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > > >,tuple$<> >", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::brush_preview::OverwriteBrushPreview>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>", "bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>", "bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>", "tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser> >"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::brush_preview::BrushPreview>,enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > > >,tuple$<> >
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::brush_preview::OverwriteBrushPreview>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::crash_service::crash_in_progress
// 信息 ID: 079d8925baf97168557fb197
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::setup_panic_hook::closure$0; module=35, inline_record=1724; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::crash_service::is_shutting_down
// 信息 ID: 0e61b376385d3e23ae4735e7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=35, inline_record=4768; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::crash_service::mark_shutting_down
// 信息 ID: da6223333ee4d64cbf04b0a1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::run; module=205, inline_record=7748; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::crash_service::note_crash_started
// 信息 ID: a395ae8d43d777180abf053f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::setup_panic_hook::closure$0; module=35, inline_record=1976; 不是独立调用边。
// [PDB 内联宿主] crash_handler::make_crash_event::impl$0::on_crash<tiny_glade::crash_pickup::client::start::closure_env$1>; module=230, inline_record=34864; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::crash_service::park_secondary_panic
// 信息 ID: c5673cfd13facb569aa064de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::setup_panic_hook::closure$0; module=35, inline_record=1864; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::crash_service::request_and_wait
// 信息 ID: 359ea85b4566e67d830ecf17
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::setup_panic_hook::closure$0 @ 0x1851f7 -> 0x512ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] crash_handler::make_crash_event::impl$0::on_crash<tiny_glade::crash_pickup::client::start::closure_env$1> @ 0x5e9e2e -> 0x512ac0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::crash_service::start
// 信息 ID: a7a9a1c5a4df16977e30aa21
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::main @ 0x643641 -> 0x512d30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::crash_service::start::closure$0
// 信息 ID: d96ac5f3427f204d841fe84d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::crash_service::start", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::crash_service::start::closure_env$0 <- std::thread::lifecycle::spawn_unchecked<tiny_glade::panic_reporter::crash_service::start::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::crash_service::start::closure_env$0 <- core::ops::function::FnOnce::call_once<std::thread::lifecycle::spawn_unchecked::closure_env$1<tiny_glade::panic_reporter::crash_service::start::closure_env$0,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::crash_service::start::closure_env$0 <- std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::panic_reporter::crash_service::start::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::panic_reporter::crash_service::start::closure_env$0,tuple$<> >; module=230, inline_record=20780; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::crash_service::start::closure$1
// 信息 ID: f3782f4af6dd107ec1ded817
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::panic_reporter::crash_service::start", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::crash_service::start; module=177, inline_record=4532; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::crash_service::start::closure$2
// 信息 ID: 2ef1c5214540c7894b1b55ae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::panic_reporter::crash_service::start", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::crash_service::start::closure_env$2 <- std::thread::lifecycle::spawn_unchecked<tiny_glade::panic_reporter::crash_service::start::closure_env$2,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::crash_service::start::closure_env$2 <- core::ops::function::FnOnce::call_once<std::thread::lifecycle::spawn_unchecked::closure_env$1<tiny_glade::panic_reporter::crash_service::start::closure_env$2,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::crash_service::start::closure_env$2 <- std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::panic_reporter::crash_service::start::closure_env$2,tuple$<> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::panic_reporter::crash_service::start::closure_env$2,tuple$<> >; module=230, inline_record=21184; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::crash_service::start::closure$2::impl$0::drop
// 信息 ID: f6281b411531b8b28be1082c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::panic_reporter::crash_service::start", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::panic_reporter::crash_service::start::closure_env$2,tuple$<> >; module=230, inline_record=22888; 不是独立调用边。
// [PDB 内联宿主] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::panic_reporter::crash_service::start::closure_env$2,tuple$<> >; module=230, inline_record=23056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::crash_service::start::closure$3
// 信息 ID: ad38cc467bbae344ac374432
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tiny_glade::panic_reporter::crash_service::start", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::crash_service::start; module=177, inline_record=5532; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
