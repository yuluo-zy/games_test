// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tiny-glade/src/panic_reporter/reverse_backtrace.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: de671c37930b420f5963b766db75daa462a71337f7da4124c0e56296f8ff1be5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: reverse_backtrace.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 15, 110
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use backtrace; // namespace candidate
//   package identities: [{"name":"backtrace","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.3.71"}]
// use core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":110,"record_offset":62956,"rva":2948400,"va":5371657520}
// 编译实例 RVAs: 0x2cfd30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$backtrace..symbolize..SymbolName$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10182548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371657520,"raw_hex":"e8945f9b00","size":5,"soft_fail":false,"target_va":5381840704,"va":5371658151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402e11a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reverse_backtrace.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t70664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371657520,"raw_hex":"e808140100","size":5,"soft_fail":false,"target_va":5371728288,"va":5371657619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ebcf0
//     owner=unresolved_namespace; count=3
//     target_aliases=4 candidates; complete list: reverse_backtrace.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3259395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371657520,"raw_hex":"e803bc3100","size":5,"soft_fail":false,"target_va":5374917872,"va":5371658472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=11
//     target_aliases=1 candidates; complete list: reverse_backtrace.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3423207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371657520,"raw_hex":"e8e73b3400","size":5,"soft_fail":false,"target_va":5375081504,"va":5371658292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] backtrace::capture::BacktraceSymbol::name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"backtrace","version":"0.3.71","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10176624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371657520,"raw_hex":"e870489b00","size":5,"soft_fail":false,"target_va":5381834624,"va":5371657995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] backtrace::print::BacktraceFrameFmt::backtrace_frame
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"backtrace","version":"0.3.71","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10179865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371657520,"raw_hex":"e819559b00","size":5,"soft_fail":false,"target_va":5381838304,"va":5371658434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt::closure_env$0>
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-1624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371657520,"raw_hex":"e8a8f9ffff","size":5,"soft_fail":false,"target_va":5371657408,"va":5371659027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39883913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371657520,"raw_hex":"e889946002","size":5,"soft_fail":false,"target_va":5411542880,"va":5371658962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39884182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371657520,"raw_hex":"e896956002","size":5,"soft_fail":false,"target_va":5411543104,"va":5371658917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39884215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371657520,"raw_hex":"e8b7956002","size":5,"soft_fail":false,"target_va":5411543120,"va":5371658900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::current_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11040975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371657520,"raw_hex":"e8cf78a800","size":5,"soft_fail":false,"target_va":5382698576,"va":5371657596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5371657520,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5371658554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":110,"record_offset":65248,"parent_record_offset":62956,"ancestor_record_offset":62956,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_direct] backtrace::capture::BacktraceFrame::symbols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"backtrace","version":"0.3.71","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.71/src/capture.rs
//     evidence={"module":110,"record_offset":63832,"parent_record_offset":62956,"ancestor_record_offset":62956,"inlinee":12041,"source_line":309,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_direct] backtrace::print::BacktraceFmt::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"backtrace","version":"0.3.71","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.71/src/print.rs
//     evidence={"module":110,"record_offset":63536,"parent_record_offset":62956,"ancestor_record_offset":62956,"inlinee":12039,"source_line":44,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":110,"record_offset":63648,"parent_record_offset":62956,"ancestor_record_offset":62956,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":110,"record_offset":63972,"parent_record_offset":62956,"ancestor_record_offset":62956,"inlinee":9843,"source_line":63,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":110,"record_offset":64996,"parent_record_offset":62956,"ancestor_record_offset":62956,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":110,"record_offset":63564,"parent_record_offset":62956,"ancestor_record_offset":62956,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":110,"record_offset":63136,"parent_record_offset":62956,"ancestor_record_offset":62956,"inlinee":7307,"source_line":990,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":110,"record_offset":65460,"parent_record_offset":62956,"ancestor_record_offset":62956,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::starts_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":110,"record_offset":64636,"parent_record_offset":62956,"ancestor_record_offset":62956,"inlinee":8789,"source_line":1366,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":110,"record_offset":64560,"parent_record_offset":64536,"ancestor_record_offset":62956,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":110,"record_offset":64536,"parent_record_offset":64480,"ancestor_record_offset":62956,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":110,"record_offset":65968,"parent_record_offset":65940,"ancestor_record_offset":62956,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":110,"record_offset":64508,"parent_record_offset":64480,"ancestor_record_offset":62956,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":110,"record_offset":64480,"parent_record_offset":64456,"ancestor_record_offset":62956,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":110,"record_offset":64456,"parent_record_offset":64432,"ancestor_record_offset":62956,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":110,"record_offset":64292,"parent_record_offset":64240,"ancestor_record_offset":62956,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":110,"record_offset":64204,"parent_record_offset":64140,"ancestor_record_offset":62956,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":110,"record_offset":64240,"parent_record_offset":64204,"ancestor_record_offset":62956,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] backtrace::print::impl$2::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"backtrace","version":"0.3.71","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.71/src/print.rs
//     evidence={"module":110,"record_offset":65564,"parent_record_offset":65540,"ancestor_record_offset":62956,"inlinee":12043,"source_line":308,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":110,"record_offset":66000,"parent_record_offset":65968,"ancestor_record_offset":62956,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":110,"record_offset":64720,"parent_record_offset":64692,"ancestor_record_offset":62956,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":110,"record_offset":64320,"parent_record_offset":64240,"ancestor_record_offset":62956,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::typed_swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":110,"record_offset":63212,"parent_record_offset":63188,"ancestor_record_offset":62956,"inlinee":6104,"source_line":4004,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":110,"record_offset":66024,"parent_record_offset":66000,"ancestor_record_offset":62956,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":110,"record_offset":63188,"parent_record_offset":63160,"ancestor_record_offset":62956,"inlinee":6271,"source_line":730,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":110,"record_offset":65364,"parent_record_offset":65320,"ancestor_record_offset":62956,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":110,"record_offset":64104,"parent_record_offset":64008,"ancestor_record_offset":62956,"inlinee":4194,"source_line":293,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":110,"record_offset":63856,"parent_record_offset":63832,"ancestor_record_offset":62956,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":110,"record_offset":64172,"parent_record_offset":64140,"ancestor_record_offset":62956,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=33
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":110,"record_offset":64384,"parent_record_offset":64240,"ancestor_record_offset":62956,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":110,"record_offset":63612,"parent_record_offset":63588,"ancestor_record_offset":62956,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":110,"record_offset":65768,"parent_record_offset":65740,"ancestor_record_offset":62956,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":110,"record_offset":65816,"parent_record_offset":65792,"ancestor_record_offset":62956,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":110,"record_offset":65792,"parent_record_offset":65768,"ancestor_record_offset":62956,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":110,"record_offset":63788,"parent_record_offset":63764,"ancestor_record_offset":62956,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":110,"record_offset":63712,"parent_record_offset":63676,"ancestor_record_offset":62956,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":110,"record_offset":63348,"parent_record_offset":63292,"ancestor_record_offset":62956,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":110,"record_offset":63236,"parent_record_offset":63212,"ancestor_record_offset":62956,"inlinee":6105,"source_line":1071,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":110,"record_offset":63264,"parent_record_offset":63236,"ancestor_record_offset":62956,"inlinee":6107,"source_line":3874,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_simple_untyped
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":110,"record_offset":63292,"parent_record_offset":63264,"ancestor_record_offset":62956,"inlinee":6108,"source_line":1139,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":110,"record_offset":63428,"parent_record_offset":63292,"ancestor_record_offset":62956,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":110,"record_offset":64348,"parent_record_offset":64240,"ancestor_record_offset":62956,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":110,"record_offset":64744,"parent_record_offset":64720,"ancestor_record_offset":62956,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":110,"record_offset":64768,"parent_record_offset":64744,"ancestor_record_offset":62956,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":110,"record_offset":63160,"parent_record_offset":63136,"ancestor_record_offset":62956,"inlinee":7309,"source_line":1013,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":110,"record_offset":64692,"parent_record_offset":64664,"ancestor_record_offset":62956,"inlinee":5361,"source_line":2581,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":110,"record_offset":65416,"parent_record_offset":65392,"ancestor_record_offset":62956,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":110,"record_offset":65392,"parent_record_offset":65320,"ancestor_record_offset":62956,"inlinee":4612,"source_line":101,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":110,"record_offset":65284,"parent_record_offset":65248,"ancestor_record_offset":62956,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":110,"record_offset":65320,"parent_record_offset":65284,"ancestor_record_offset":62956,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":110,"record_offset":63588,"parent_record_offset":63564,"ancestor_record_offset":62956,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":110,"record_offset":63764,"parent_record_offset":63740,"ancestor_record_offset":62956,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":110,"record_offset":64008,"parent_record_offset":63972,"ancestor_record_offset":62956,"inlinee":11874,"source_line":313,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":110,"record_offset":63676,"parent_record_offset":63648,"ancestor_record_offset":62956,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":110,"record_offset":63740,"parent_record_offset":63676,"ancestor_record_offset":62956,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":110,"record_offset":64664,"parent_record_offset":64636,"ancestor_record_offset":62956,"inlinee":8748,"source_line":981,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":110,"record_offset":65740,"parent_record_offset":65708,"ancestor_record_offset":62956,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":110,"record_offset":65708,"parent_record_offset":65680,"ancestor_record_offset":62956,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/reverse_backtrace.rs
//     evidence={"module":110,"record_offset":64140,"parent_record_offset":64104,"ancestor_record_offset":62956,"inlinee":6300,"source_line":36,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":110,"record_offset":68248,"rva":2950544,"va":5371659664}
// 编译实例 RVAs: 0x2d0590
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$std..path..Display$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t11086143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371659664,"raw_hex":"e83f29a900","size":5,"soft_fail":false,"target_va":5382745920,"va":5371659772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: reverse_backtrace.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3421670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371659664,"raw_hex":"e8e6353400","size":5,"soft_fail":false,"target_va":5375081504,"va":5371659829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=3
//     target_aliases=5 candidates; complete list: reverse_backtrace.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11168007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371659664,"raw_hex":"e80769aa00","size":5,"soft_fail":false,"target_va":5382827728,"va":5371659716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] backtrace::types::BytesOrWideString::into_path_buf
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"backtrace","version":"0.3.71","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10176327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371659664,"raw_hex":"e847479b00","size":5,"soft_fail":false,"target_va":5381836032,"va":5371659700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_strip_prefix
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11082024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371659664,"raw_hex":"e82819a900","size":5,"soft_fail":false,"target_va":5382741776,"va":5371659747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":110,"record_offset":68948,"parent_record_offset":68248,"ancestor_record_offset":68248,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::strip_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":110,"record_offset":68552,"parent_record_offset":68248,"ancestor_record_offset":68248,"inlinee":5198,"source_line":2508,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":110,"record_offset":68412,"parent_record_offset":68248,"ancestor_record_offset":68248,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":110,"record_offset":69196,"parent_record_offset":69172,"ancestor_record_offset":68248,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":110,"record_offset":69172,"parent_record_offset":69116,"ancestor_record_offset":68248,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":110,"record_offset":69144,"parent_record_offset":69116,"ancestor_record_offset":68248,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":110,"record_offset":69116,"parent_record_offset":69092,"ancestor_record_offset":68248,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":110,"record_offset":69092,"parent_record_offset":69068,"ancestor_record_offset":68248,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":110,"record_offset":68580,"parent_record_offset":68552,"ancestor_record_offset":68248,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":110,"record_offset":68972,"parent_record_offset":68948,"ancestor_record_offset":68248,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":110,"record_offset":68460,"parent_record_offset":68436,"ancestor_record_offset":68248,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":110,"record_offset":68508,"parent_record_offset":68484,"ancestor_record_offset":68248,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":110,"record_offset":68484,"parent_record_offset":68460,"ancestor_record_offset":68248,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":110,"record_offset":68436,"parent_record_offset":68412,"ancestor_record_offset":68248,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":110,"record_offset":68636,"parent_record_offset":68608,"ancestor_record_offset":68248,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":110,"record_offset":68608,"parent_record_offset":68580,"ancestor_record_offset":68248,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x189C
// 首条位置: {"module":15,"inlinee":6300}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":110,"record_offset":64204,"parent_record_offset":64140,"ancestor_record_offset":64140,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":110,"record_offset":64172,"parent_record_offset":64140,"ancestor_record_offset":64140,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":110,"record_offset":64560,"parent_record_offset":64536,"ancestor_record_offset":64140,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":110,"record_offset":64536,"parent_record_offset":64480,"ancestor_record_offset":64140,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":110,"record_offset":64508,"parent_record_offset":64480,"ancestor_record_offset":64140,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":110,"record_offset":64480,"parent_record_offset":64456,"ancestor_record_offset":64140,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":110,"record_offset":64456,"parent_record_offset":64432,"ancestor_record_offset":64140,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":110,"record_offset":64292,"parent_record_offset":64240,"ancestor_record_offset":64140,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":110,"record_offset":64240,"parent_record_offset":64204,"ancestor_record_offset":64140,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":110,"record_offset":64320,"parent_record_offset":64240,"ancestor_record_offset":64140,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":110,"record_offset":64384,"parent_record_offset":64240,"ancestor_record_offset":64140,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00110.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":110,"record_offset":64348,"parent_record_offset":64240,"ancestor_record_offset":64140,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00110.jsonl.gz"}
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
// 完整结构及原始类型记录: reverse_backtrace.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: reverse_backtrace.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: reverse_backtrace.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>", "bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>", "bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::arches::LintelSegments>)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::arches::LintelSegments>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::arches::LintelSegments>

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt
// 信息 ID: 7e12a8c4eff766bba502b725
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt::closure$0
// 信息 ID: 097180e51cac0b943b2b2628
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt::closure_env$0 <- core::ops::function::FnOnce::call_once<tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt::closure_env$0,tuple$<ref_mut$<core::fmt::Formatter>,enum2$<backtrace::types::BytesOrWideString> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt::closure_env$0 <- core::ptr::drop_in_place<tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt::closure_env$0,tuple$<ref_mut$<core::fmt::Formatter>,enum2$<backtrace::types::BytesOrWideString> > > @ 0x2cfc62 -> 0x2d0590 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt::closure$1
// 信息 ID: feae6b0235fbff2677c32af8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt; module=110, inline_record=64140; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
