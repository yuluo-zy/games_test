// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/panic_reporter/reverse_backtrace.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 188707e5c4f5690cd191533b23e39110835fdc63aac27cdefa5b6735dc1b6a96
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: reverse_backtrace.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 11, 188
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use backtrace; // namespace candidate
//   package identities: [{"name":"backtrace","source":"PDB_registry_path_only","version":"0.3.71"}]
// use core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":188,"record_offset":4904,"rva":3089232,"va":5371798352}
// 编译实例 RVAs: 0x2f2350
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$backtrace..symbolize..SymbolName$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4565241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371798352,"raw_hex":"e8f9a84500","size":5,"soft_fail":false,"target_va":5376364528,"va":5371799282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e2780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reverse_backtrace.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2162006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371798352,"raw_hex":"e8aa02dfff","size":5,"soft_fail":false,"target_va":5369636736,"va":5371798737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402f2020
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: reverse_backtrace.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371798352,"raw_hex":"e8a5f4ffff","size":5,"soft_fail":false,"target_va":5371797536,"va":5371800438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=2
//     target_aliases=89 candidates; complete list: reverse_backtrace.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t514562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371798352,"raw_hex":"e802da0700","size":5,"soft_fail":false,"target_va":5372314208,"va":5371799641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038b7b0
//     owner=unresolved_namespace; count=4
//     target_aliases=4 candidates; complete list: reverse_backtrace.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t626548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371798352,"raw_hex":"e8748f0900","size":5,"soft_fail":false,"target_va":5372426160,"va":5371799607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: reverse_backtrace.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t951193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371798352,"raw_hex":"e899830e00","size":5,"soft_fail":false,"target_va":5372749696,"va":5371798498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: reverse_backtrace.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t950277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371798352,"raw_hex":"e805800e00","size":5,"soft_fail":false,"target_va":5372749712,"va":5371799430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=1
//     target_aliases=28 candidates; complete list: reverse_backtrace.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3372615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371798352,"raw_hex":"e847763300","size":5,"soft_fail":false,"target_va":5375171056,"va":5371798436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31178187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371798352,"raw_hex":"e8cbbddb01","size":5,"soft_fail":false,"target_va":5402978144,"va":5371799952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5727178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371798352,"raw_hex":"e8ca635700","size":5,"soft_fail":false,"target_va":5377527120,"va":5371799937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] backtrace::capture::BacktraceFrame::symbols
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4569929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371798352,"raw_hex":"e849bb4500","size":5,"soft_fail":false,"target_va":5376369008,"va":5371799074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] backtrace::capture::BacktraceSymbol::name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4569920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371798352,"raw_hex":"e840bb4500","size":5,"soft_fail":false,"target_va":5376369056,"va":5371799131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] backtrace::print::BacktraceFrameFmt::backtrace_frame
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4565769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371798352,"raw_hex":"e809ab4500","size":5,"soft_fail":false,"target_va":5376365344,"va":5371799570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31179495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371798352,"raw_hex":"e8e7c2db01","size":5,"soft_fail":false,"target_va":5402979344,"va":5371799844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31180234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371798352,"raw_hex":"e8cac5db01","size":5,"soft_fail":false,"target_va":5402980128,"va":5371799889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31180369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371798352,"raw_hex":"e851c6db01","size":5,"soft_fail":false,"target_va":5402980304,"va":5371799930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31180402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371798352,"raw_hex":"e872c6db01","size":5,"soft_fail":false,"target_va":5402980320,"va":5371799913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::current_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5600703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371798352,"raw_hex":"e8bf755500","size":5,"soft_fail":false,"target_va":5377399136,"va":5371798428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":188,"record_offset":5084,"parent_record_offset":4904,"ancestor_record_offset":4904,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":188,"record_offset":7800,"parent_record_offset":4904,"ancestor_record_offset":4904,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_direct] backtrace::print::BacktraceFmt::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"backtrace","version":"0.3.71","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/backtrace-0.3.71/src/print.rs
//     evidence={"module":188,"record_offset":6412,"parent_record_offset":4904,"ancestor_record_offset":4904,"inlinee":12463,"source_line":44,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":188,"record_offset":6440,"parent_record_offset":4904,"ancestor_record_offset":4904,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":188,"record_offset":6616,"parent_record_offset":4904,"ancestor_record_offset":4904,"inlinee":8593,"source_line":63,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":188,"record_offset":7580,"parent_record_offset":4904,"ancestor_record_offset":4904,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":188,"record_offset":6532,"parent_record_offset":4904,"ancestor_record_offset":4904,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":188,"record_offset":6044,"parent_record_offset":4904,"ancestor_record_offset":4904,"inlinee":9336,"source_line":976,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":188,"record_offset":7976,"parent_record_offset":4904,"ancestor_record_offset":4904,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::starts_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":188,"record_offset":7220,"parent_record_offset":4904,"ancestor_record_offset":4904,"inlinee":10281,"source_line":1189,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":188,"record_offset":5476,"parent_record_offset":5452,"ancestor_record_offset":4904,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":188,"record_offset":5500,"parent_record_offset":5476,"ancestor_record_offset":4904,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":188,"record_offset":7152,"parent_record_offset":7128,"ancestor_record_offset":4904,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":188,"record_offset":5452,"parent_record_offset":5360,"ancestor_record_offset":4904,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":188,"record_offset":7128,"parent_record_offset":7072,"ancestor_record_offset":4904,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":188,"record_offset":7100,"parent_record_offset":7072,"ancestor_record_offset":4904,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":188,"record_offset":5360,"parent_record_offset":5332,"ancestor_record_offset":4904,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":188,"record_offset":5332,"parent_record_offset":5304,"ancestor_record_offset":4904,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":188,"record_offset":5568,"parent_record_offset":5332,"ancestor_record_offset":4904,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":188,"record_offset":7072,"parent_record_offset":7044,"ancestor_record_offset":4904,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":188,"record_offset":5244,"parent_record_offset":5204,"ancestor_record_offset":4904,"inlinee":7706,"source_line":124,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$0::to_vec::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":188,"record_offset":5964,"parent_record_offset":5940,"ancestor_record_offset":4904,"inlinee":7731,"source_line":131,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":188,"record_offset":5204,"parent_record_offset":5164,"ancestor_record_offset":4904,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":188,"record_offset":5124,"parent_record_offset":5084,"ancestor_record_offset":4904,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":188,"record_offset":5164,"parent_record_offset":5124,"ancestor_record_offset":4904,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":188,"record_offset":6904,"parent_record_offset":6852,"ancestor_record_offset":4904,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":188,"record_offset":6852,"parent_record_offset":6788,"ancestor_record_offset":4904,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":188,"record_offset":5988,"parent_record_offset":5964,"ancestor_record_offset":4904,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":188,"record_offset":5304,"parent_record_offset":5244,"ancestor_record_offset":4904,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] backtrace::capture::impl$12::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"backtrace","version":"0.3.71","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/backtrace-0.3.71/src/capture.rs
//     evidence={"module":188,"record_offset":5692,"parent_record_offset":5244,"ancestor_record_offset":4904,"inlinee":12459,"source_line":45,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] backtrace::capture::impl$13::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"backtrace","version":"0.3.71","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/backtrace-0.3.71/src/capture.rs
//     evidence={"module":188,"record_offset":5724,"parent_record_offset":5692,"ancestor_record_offset":4904,"inlinee":12461,"source_line":51,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] backtrace::print::impl$2::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"backtrace","version":"0.3.71","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/backtrace-0.3.71/src/print.rs
//     evidence={"module":188,"record_offset":8084,"parent_record_offset":8060,"ancestor_record_offset":4904,"inlinee":12465,"source_line":308,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":188,"record_offset":5396,"parent_record_offset":5360,"ancestor_record_offset":4904,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":188,"record_offset":5420,"parent_record_offset":5396,"ancestor_record_offset":4904,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":188,"record_offset":7304,"parent_record_offset":7276,"ancestor_record_offset":4904,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":188,"record_offset":6932,"parent_record_offset":6852,"ancestor_record_offset":4904,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":188,"record_offset":5844,"parent_record_offset":5816,"ancestor_record_offset":4904,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\take.rs
//     evidence={"module":188,"record_offset":5816,"parent_record_offset":5244,"ancestor_record_offset":4904,"inlinee":7708,"source_line":38,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":188,"record_offset":5788,"parent_record_offset":5244,"ancestor_record_offset":4904,"inlinee":6259,"source_line":493,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":188,"record_offset":6100,"parent_record_offset":6068,"ancestor_record_offset":4904,"inlinee":8892,"source_line":728,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap_simple
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":188,"record_offset":6172,"parent_record_offset":6148,"ancestor_record_offset":4904,"inlinee":8891,"source_line":761,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":188,"record_offset":6752,"parent_record_offset":6652,"ancestor_record_offset":4904,"inlinee":4399,"source_line":293,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":188,"record_offset":6820,"parent_record_offset":6788,"ancestor_record_offset":4904,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":188,"record_offset":5752,"parent_record_offset":5692,"ancestor_record_offset":4904,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":188,"record_offset":7932,"parent_record_offset":7908,"ancestor_record_offset":4904,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":188,"record_offset":5940,"parent_record_offset":5244,"ancestor_record_offset":4904,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":188,"record_offset":5656,"parent_record_offset":5632,"ancestor_record_offset":4904,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":188,"record_offset":5900,"parent_record_offset":5868,"ancestor_record_offset":4904,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":188,"record_offset":6228,"parent_record_offset":6172,"ancestor_record_offset":4904,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":188,"record_offset":5524,"parent_record_offset":5500,"ancestor_record_offset":4904,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":188,"record_offset":6124,"parent_record_offset":6100,"ancestor_record_offset":4904,"inlinee":8889,"source_line":1002,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_simple_untyped
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":188,"record_offset":6148,"parent_record_offset":6124,"ancestor_record_offset":4904,"inlinee":8890,"source_line":1059,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":188,"record_offset":6308,"parent_record_offset":6172,"ancestor_record_offset":4904,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":188,"record_offset":6960,"parent_record_offset":6852,"ancestor_record_offset":4904,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":188,"record_offset":7328,"parent_record_offset":7304,"ancestor_record_offset":4904,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":188,"record_offset":7352,"parent_record_offset":7328,"ancestor_record_offset":4904,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":188,"record_offset":5608,"parent_record_offset":5244,"ancestor_record_offset":4904,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":188,"record_offset":6068,"parent_record_offset":6044,"ancestor_record_offset":4904,"inlinee":9338,"source_line":999,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":188,"record_offset":7276,"parent_record_offset":7248,"ancestor_record_offset":4904,"inlinee":5503,"source_line":2554,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":188,"record_offset":7832,"parent_record_offset":7800,"ancestor_record_offset":4904,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":188,"record_offset":7908,"parent_record_offset":7864,"ancestor_record_offset":4904,"inlinee":5475,"source_line":373,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":188,"record_offset":7864,"parent_record_offset":7832,"ancestor_record_offset":4904,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":188,"record_offset":5632,"parent_record_offset":5608,"ancestor_record_offset":4904,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":188,"record_offset":6652,"parent_record_offset":6616,"ancestor_record_offset":4904,"inlinee":8600,"source_line":313,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":188,"record_offset":5868,"parent_record_offset":5844,"ancestor_record_offset":4904,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":188,"record_offset":7248,"parent_record_offset":7220,"ancestor_record_offset":4904,"inlinee":7373,"source_line":962,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/reverse_backtrace.rs
//     evidence={"module":188,"record_offset":6788,"parent_record_offset":6752,"ancestor_record_offset":4904,"inlinee":5364,"source_line":36,"archive":"_pdb/A/modules/00188.jsonl.gz"}
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
// 首条位置: {"module":188,"record_offset":9096,"rva":3091360,"va":5371800480}
// 编译实例 RVAs: 0x2f2ba0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$std..path..Display$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5640647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371800480,"raw_hex":"e8c7115600","size":5,"soft_fail":false,"target_va":5377441248,"va":5371800596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: reverse_backtrace.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t949054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371800480,"raw_hex":"e83e7b0e00","size":5,"soft_fail":false,"target_va":5372749712,"va":5371800653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=3
//     target_aliases=28 candidates; complete list: reverse_backtrace.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3370511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371800480,"raw_hex":"e80f6e3300","size":5,"soft_fail":false,"target_va":5375171056,"va":5371800540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] backtrace::types::BytesOrWideString::into_path_buf
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4569815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371800480,"raw_hex":"e8d7ba4500","size":5,"soft_fail":false,"target_va":5376370336,"va":5371800516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_strip_prefix
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5637008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371800480,"raw_hex":"e890035600","size":5,"soft_fail":false,"target_va":5377437584,"va":5371800571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":188,"record_offset":9796,"parent_record_offset":9096,"ancestor_record_offset":9096,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::strip_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":188,"record_offset":9400,"parent_record_offset":9096,"ancestor_record_offset":9096,"inlinee":11090,"source_line":2371,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":188,"record_offset":9260,"parent_record_offset":9096,"ancestor_record_offset":9096,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":188,"record_offset":10036,"parent_record_offset":10012,"ancestor_record_offset":9096,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":188,"record_offset":10012,"parent_record_offset":9956,"ancestor_record_offset":9096,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":188,"record_offset":9984,"parent_record_offset":9956,"ancestor_record_offset":9096,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":188,"record_offset":9956,"parent_record_offset":9928,"ancestor_record_offset":9096,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":188,"record_offset":9428,"parent_record_offset":9400,"ancestor_record_offset":9096,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":188,"record_offset":9820,"parent_record_offset":9796,"ancestor_record_offset":9096,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":188,"record_offset":9308,"parent_record_offset":9284,"ancestor_record_offset":9096,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":188,"record_offset":9356,"parent_record_offset":9332,"ancestor_record_offset":9096,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":188,"record_offset":9332,"parent_record_offset":9308,"ancestor_record_offset":9096,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":188,"record_offset":9284,"parent_record_offset":9260,"ancestor_record_offset":9096,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":188,"record_offset":9484,"parent_record_offset":9456,"ancestor_record_offset":9096,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":188,"record_offset":9456,"parent_record_offset":9428,"ancestor_record_offset":9096,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14F4
// 首条位置: {"module":11,"inlinee":5364}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":188,"record_offset":6852,"parent_record_offset":6788,"ancestor_record_offset":6788,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":188,"record_offset":6820,"parent_record_offset":6788,"ancestor_record_offset":6788,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":188,"record_offset":7152,"parent_record_offset":7128,"ancestor_record_offset":6788,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":188,"record_offset":7128,"parent_record_offset":7072,"ancestor_record_offset":6788,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":188,"record_offset":7100,"parent_record_offset":7072,"ancestor_record_offset":6788,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":188,"record_offset":7072,"parent_record_offset":7044,"ancestor_record_offset":6788,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":188,"record_offset":6904,"parent_record_offset":6852,"ancestor_record_offset":6788,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":188,"record_offset":6932,"parent_record_offset":6852,"ancestor_record_offset":6788,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":188,"record_offset":6996,"parent_record_offset":6852,"ancestor_record_offset":6788,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00188.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":188,"record_offset":6960,"parent_record_offset":6852,"ancestor_record_offset":6788,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00188.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt
// 信息 ID: f6659fc7fdf2ed1feb365a42
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt::closure$0
// 信息 ID: 2ea7cec6cb75c0534dd1c054
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt::closure_env$0 <- core::ops::function::FnOnce::call_once<tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt::closure_env$0,tuple$<ref_mut$<core::fmt::Formatter>,enum2$<backtrace::types::BytesOrWideString> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt::closure_env$0 <- core::ptr::drop_in_place<tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt::closure_env$0,tuple$<ref_mut$<core::fmt::Formatter>,enum2$<backtrace::types::BytesOrWideString> > > @ 0x2f1fb2 -> 0x2f2ba0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt::closure$1
// 信息 ID: 86a96ea2eba33c6f15b69873
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt; module=188, inline_record=6788; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
