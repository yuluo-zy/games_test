// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/crates/gpu-profiler/src/backend/ash.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 6bd789bc0171ab81607011f2e71038f351d1a4838f4c1927277b087f0feb543c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ash.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 72, 99, 128, 236, 248, 249, 703, 722, 1935, 1936, 1972
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use gpu_profiler; // namespace candidate
//   package identities: [{"name":"gpu-profiler","source":"local_or_path","version":"0.1.0"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: gpu_profiler::backend::ash::ProfilerOutputBuffer::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9B69
// 首条位置: {"module":703,"inlinee":39785}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::profiler::impl$1::timestamp_period
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/profiler.rs
//     evidence={"module":703,"record_offset":147100,"parent_record_offset":147068,"ancestor_record_offset":147068,"inlinee":39787,"source_line":69,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: gpu_profiler::backend::ash::ProfilerSubmittedFrame::retrieve_results<rhapsody::raw::profiler::ProfilerBuffer>
// 记录形式: compiled_procedure; instances=1
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":99,"record_offset":47496,"rva":2706672,"va":5371415792}
// 编译实例 RVAs: 0x294cf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405c3580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ash.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3335705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371415792,"raw_hex":"e819e63200","size":5,"soft_fail":false,"target_va":5374752128,"va":5371416418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405c75b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ash.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3352096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371415792,"raw_hex":"e820263300","size":5,"soft_fail":false,"target_va":5374768560,"va":5371416459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d5ef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ash.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3411755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371415792,"raw_hex":"e82b0f3400","size":5,"soft_fail":false,"target_va":5374828272,"va":5371416512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ash.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3665507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371415792,"raw_hex":"e863ee3700","size":5,"soft_fail":false,"target_va":5375081488,"va":5371415976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: ash.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3665219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371415792,"raw_hex":"e843ed3700","size":5,"soft_fail":false,"target_va":5375081504,"va":5371416280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd3c80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ash.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9695062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371415792,"raw_hex":"e856ef9300","size":5,"soft_fail":false,"target_va":5381110912,"va":5371415845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40124517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371415792,"raw_hex":"e865406402","size":5,"soft_fail":false,"target_va":5411540963,"va":5371416441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40126072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371415792,"raw_hex":"e878466402","size":5,"soft_fail":false,"target_va":5411542422,"va":5371416345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40125688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371415792,"raw_hex":"e8f8446402","size":5,"soft_fail":false,"target_va":5411542064,"va":5371416371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40126711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371415792,"raw_hex":"e8f7486402","size":5,"soft_fail":false,"target_va":5411543104,"va":5371416388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":99,"record_offset":47772,"parent_record_offset":47496,"ancestor_record_offset":47496,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":99,"record_offset":48952,"parent_record_offset":47496,"ancestor_record_offset":47496,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::sort_unstable_by_key
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":99,"record_offset":48816,"parent_record_offset":47496,"ancestor_record_offset":47496,"inlinee":11461,"source_line":3069,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":99,"record_offset":47704,"parent_record_offset":47496,"ancestor_record_offset":47496,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":99,"record_offset":48208,"parent_record_offset":48184,"ancestor_record_offset":47496,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":99,"record_offset":48232,"parent_record_offset":48208,"ancestor_record_offset":47496,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":99,"record_offset":49092,"parent_record_offset":49068,"ancestor_record_offset":47496,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":99,"record_offset":48184,"parent_record_offset":48008,"ancestor_record_offset":47496,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":99,"record_offset":49068,"parent_record_offset":48980,"ancestor_record_offset":47496,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":99,"record_offset":48980,"parent_record_offset":48952,"ancestor_record_offset":47496,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":99,"record_offset":47948,"parent_record_offset":47920,"ancestor_record_offset":47496,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":99,"record_offset":49232,"parent_record_offset":49204,"ancestor_record_offset":47496,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":99,"record_offset":49204,"parent_record_offset":49180,"ancestor_record_offset":47496,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":99,"record_offset":48008,"parent_record_offset":47976,"ancestor_record_offset":47496,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":99,"record_offset":47976,"parent_record_offset":47948,"ancestor_record_offset":47496,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":99,"record_offset":49180,"parent_record_offset":49156,"ancestor_record_offset":47496,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":99,"record_offset":48044,"parent_record_offset":48008,"ancestor_record_offset":47496,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":99,"record_offset":48312,"parent_record_offset":48288,"ancestor_record_offset":47496,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":99,"record_offset":47892,"parent_record_offset":47856,"ancestor_record_offset":47496,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":99,"record_offset":47920,"parent_record_offset":47892,"ancestor_record_offset":47496,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":99,"record_offset":47800,"parent_record_offset":47772,"ancestor_record_offset":47496,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":99,"record_offset":48656,"parent_record_offset":48624,"ancestor_record_offset":47496,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":99,"record_offset":48744,"parent_record_offset":48656,"ancestor_record_offset":47496,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":99,"record_offset":48288,"parent_record_offset":47856,"ancestor_record_offset":47496,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":99,"record_offset":47828,"parent_record_offset":47800,"ancestor_record_offset":47496,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":99,"record_offset":47856,"parent_record_offset":47828,"ancestor_record_offset":47496,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":99,"record_offset":49012,"parent_record_offset":48980,"ancestor_record_offset":47496,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":99,"record_offset":48068,"parent_record_offset":48044,"ancestor_record_offset":47496,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":99,"record_offset":48092,"parent_record_offset":48068,"ancestor_record_offset":47496,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":99,"record_offset":48476,"parent_record_offset":48448,"ancestor_record_offset":47496,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":99,"record_offset":48888,"parent_record_offset":48848,"ancestor_record_offset":47496,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::__iterator_get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":99,"record_offset":48548,"parent_record_offset":48516,"ancestor_record_offset":47496,"inlinee":11460,"source_line":95,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":99,"record_offset":48360,"parent_record_offset":48336,"ancestor_record_offset":47496,"inlinee":4682,"source_line":100,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":99,"record_offset":48384,"parent_record_offset":48360,"ancestor_record_offset":47496,"inlinee":4684,"source_line":277,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":99,"record_offset":48516,"parent_record_offset":48384,"ancestor_record_offset":47496,"inlinee":11459,"source_line":269,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":99,"record_offset":48448,"parent_record_offset":48420,"ancestor_record_offset":47496,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":99,"record_offset":48420,"parent_record_offset":48384,"ancestor_record_offset":47496,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":99,"record_offset":48336,"parent_record_offset":48312,"ancestor_record_offset":47496,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":99,"record_offset":48624,"parent_record_offset":48384,"ancestor_record_offset":47496,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":99,"record_offset":49036,"parent_record_offset":49012,"ancestor_record_offset":47496,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":99,"record_offset":48116,"parent_record_offset":48092,"ancestor_record_offset":47496,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":99,"record_offset":48140,"parent_record_offset":48116,"ancestor_record_offset":47496,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":99,"record_offset":49256,"parent_record_offset":49232,"ancestor_record_offset":47496,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":99,"record_offset":49156,"parent_record_offset":49132,"ancestor_record_offset":47496,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":99,"record_offset":48696,"parent_record_offset":48656,"ancestor_record_offset":47496,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":99,"record_offset":47732,"parent_record_offset":47704,"ancestor_record_offset":47496,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::unstable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\unstable\mod.rs
//     evidence={"module":99,"record_offset":48848,"parent_record_offset":48816,"ancestor_record_offset":47496,"inlinee":7632,"source_line":20,"archive":"_pdb/B/modules/00099.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: gpu_profiler::backend::ash::impl$1::retrieve_results::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=68
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2826
// 首条位置: {"module":72,"inlinee":10278}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: gpu_profiler::backend::ash::Profiler::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 92
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":722,"record_offset":260,"rva":12652320,"va":5381361440}
// 编译实例 RVAs: 0xc10f20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ash.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6280099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381361440,"raw_hex":"e85d2ca0ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5381361582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30179157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381361440,"raw_hex":"e8557fcc01","size":5,"soft_fail":false,"target_va":5411540963,"va":5381361801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30181096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381361440,"raw_hex":"e8e886cc01","size":5,"soft_fail":false,"target_va":5411542880,"va":5381361779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*240(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381361440,"raw_hex":"ff90f0000000","size":6,"soft_fail":false,"va":5381361546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::create_query_pool
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":722,"record_offset":456,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":39489,"source_line":2573,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::QueryPoolCreateInfo::query_count
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":722,"record_offset":428,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":39488,"source_line":7695,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$354::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":722,"record_offset":404,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":39486,"source_line":7667,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":722,"record_offset":572,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":722,"record_offset":1248,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":722,"record_offset":860,"parent_record_offset":836,"ancestor_record_offset":260,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":722,"record_offset":884,"parent_record_offset":860,"ancestor_record_offset":260,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":722,"record_offset":836,"parent_record_offset":808,"ancestor_record_offset":260,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":722,"record_offset":748,"parent_record_offset":720,"ancestor_record_offset":260,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":722,"record_offset":808,"parent_record_offset":776,"ancestor_record_offset":260,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":722,"record_offset":776,"parent_record_offset":748,"ancestor_record_offset":260,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":722,"record_offset":964,"parent_record_offset":940,"ancestor_record_offset":260,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":722,"record_offset":692,"parent_record_offset":656,"ancestor_record_offset":260,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":722,"record_offset":720,"parent_record_offset":692,"ancestor_record_offset":260,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":722,"record_offset":600,"parent_record_offset":572,"ancestor_record_offset":260,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":722,"record_offset":1108,"parent_record_offset":1084,"ancestor_record_offset":260,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":722,"record_offset":1172,"parent_record_offset":1108,"ancestor_record_offset":260,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":722,"record_offset":940,"parent_record_offset":656,"ancestor_record_offset":260,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":722,"record_offset":628,"parent_record_offset":600,"ancestor_record_offset":260,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":722,"record_offset":656,"parent_record_offset":628,"ancestor_record_offset":260,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":722,"record_offset":484,"parent_record_offset":456,"ancestor_record_offset":260,"inlinee":15819,"source_line":24,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":722,"record_offset":508,"parent_record_offset":484,"ancestor_record_offset":260,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":722,"record_offset":532,"parent_record_offset":508,"ancestor_record_offset":260,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":722,"record_offset":1012,"parent_record_offset":988,"ancestor_record_offset":260,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":722,"record_offset":1060,"parent_record_offset":1036,"ancestor_record_offset":260,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":722,"record_offset":1036,"parent_record_offset":1012,"ancestor_record_offset":260,"inlinee":5298,"source_line":2539,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":722,"record_offset":988,"parent_record_offset":964,"ancestor_record_offset":260,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":722,"record_offset":1084,"parent_record_offset":1060,"ancestor_record_offset":260,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":722,"record_offset":1140,"parent_record_offset":1108,"ancestor_record_offset":260,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: gpu_profiler::backend::ash::Profiler::write_marker
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9DBD
// 首条位置: {"module":722,"inlinee":40381}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::cmd_write_timestamp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":722,"record_offset":2188,"parent_record_offset":2008,"ancestor_record_offset":2008,"inlinee":39500,"source_line":2117,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":722,"record_offset":2048,"parent_record_offset":2008,"ancestor_record_offset":2008,"inlinee":4126,"source_line":2956,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicU64::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":722,"record_offset":2132,"parent_record_offset":2008,"ancestor_record_offset":2008,"inlinee":19451,"source_line":2725,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_direct] gpu_profiler::shared::MarkerId::as_u64
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"gpu-profiler","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/gpu-profiler/src/shared.rs
//     evidence={"module":722,"record_offset":2104,"parent_record_offset":2008,"ancestor_record_offset":2008,"inlinee":40365,"source_line":11,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":722,"record_offset":2072,"parent_record_offset":2048,"ancestor_record_offset":2008,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":722,"record_offset":2156,"parent_record_offset":2132,"ancestor_record_offset":2008,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: gpu_profiler::backend::ash::Profiler::begin_frame
// 记录形式: compiled_procedure; instances=1
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":722,"record_offset":1280,"rva":12652688,"va":5381361808}
// 编译实例 RVAs: 0xc11090
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*896(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381361808,"raw_hex":"ff9080030000","size":6,"soft_fail":false,"va":5381361839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::cmd_reset_query_pool
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":722,"record_offset":1420,"parent_record_offset":1280,"ancestor_record_offset":1280,"inlinee":39499,"source_line":2105,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":722,"record_offset":1448,"parent_record_offset":1280,"ancestor_record_offset":1280,"inlinee":20619,"source_line":2725,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":722,"record_offset":1472,"parent_record_offset":1448,"ancestor_record_offset":1280,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: gpu_profiler::backend::ash::ActiveProfiler::create_marker
// 记录形式: compiled_procedure; instances=1
// 源码行: 153
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":722,"record_offset":1508,"rva":12652784,"va":5381361904}
// 编译实例 RVAs: 0xc110f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13362006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381361904,"raw_hex":"e856e3cb00","size":5,"soft_fail":false,"target_va":5394723952,"va":5381361941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] gpu_profiler::shared::FrameMarkers::create_marker
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"gpu-profiler","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/gpu-profiler/src/shared.rs
//     evidence={"module":722,"record_offset":1652,"parent_record_offset":1508,"ancestor_record_offset":1508,"inlinee":40375,"source_line":82,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":722,"record_offset":1740,"parent_record_offset":1708,"ancestor_record_offset":1508,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":722,"record_offset":1788,"parent_record_offset":1764,"ancestor_record_offset":1508,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":722,"record_offset":1764,"parent_record_offset":1740,"ancestor_record_offset":1508,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":722,"record_offset":1708,"parent_record_offset":1676,"ancestor_record_offset":1508,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":722,"record_offset":1676,"parent_record_offset":1652,"ancestor_record_offset":1508,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":722,"record_offset":1828,"parent_record_offset":1676,"ancestor_record_offset":1508,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: gpu_profiler::backend::ash::ActiveProfiler::write_marker
// 记录形式: compiled_procedure; instances=1
// 源码行: 157
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":722,"record_offset":1868,"rva":12652880,"va":5381362000}
// 编译实例 RVAs: 0xc11150
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30180208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381362000,"raw_hex":"e87083cc01","size":5,"soft_fail":false,"target_va":5411542296,"va":5381362083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] gpu_profiler::backend::ash::Profiler::write_marker
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"gpu-profiler","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/gpu-profiler/src/backend/ash.rs
//     evidence={"module":722,"record_offset":2008,"parent_record_offset":1868,"ancestor_record_offset":1868,"inlinee":40381,"source_line":108,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::cmd_write_timestamp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":722,"record_offset":2188,"parent_record_offset":2008,"ancestor_record_offset":1868,"inlinee":39500,"source_line":2117,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":722,"record_offset":2048,"parent_record_offset":2008,"ancestor_record_offset":1868,"inlinee":4126,"source_line":2956,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU64::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":722,"record_offset":2132,"parent_record_offset":2008,"ancestor_record_offset":1868,"inlinee":19451,"source_line":2725,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":722,"record_offset":2072,"parent_record_offset":2048,"ancestor_record_offset":1868,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":722,"record_offset":2156,"parent_record_offset":2132,"ancestor_record_offset":1868,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   [pdb_inline_transitive] gpu_profiler::shared::MarkerId::as_u64
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"gpu-profiler","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/gpu-profiler/src/shared.rs
//     evidence={"module":722,"record_offset":2104,"parent_record_offset":2008,"ancestor_record_offset":1868,"inlinee":40365,"source_line":11,"archive":"_pdb/B/modules/00722.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: gpu_profiler::backend::ash::ActiveProfiler::end_frame<rhapsody::raw::profiler::ProfilerBuffer>
// 记录形式: compiled_procedure; instances=1
// 源码行: 167
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1972,"record_offset":256,"rva":25966864,"va":5394675984}
// 编译实例 RVAs: 0x18c3910
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: ash.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19594825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394675984,"raw_hex":"e8b701d5fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5394676324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ash.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19594744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394675984,"raw_hex":"e80802d5fe","size":5,"soft_fail":false,"target_va":5375081520,"va":5394676259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bddfa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ash.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13523827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394675984,"raw_hex":"e88da431ff","size":5,"soft_fail":false,"target_va":5381152672,"va":5394676494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b8a60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ash.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-44946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394675984,"raw_hex":"e86e50ffff","size":5,"soft_fail":false,"target_va":5394631264,"va":5394676205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16864663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394675984,"raw_hex":"e897550101","size":5,"soft_fail":false,"target_va":5411540963,"va":5394676295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*912(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5394675984,"raw_hex":"ff9090030000","size":6,"soft_fail":false,"va":5394676090,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::cmd_copy_query_pool_results
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1972,"record_offset":544,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":39511,"source_line":1784,"archive":"_pdb/B/modules/01972.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1972,"record_offset":740,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01972.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1972,"record_offset":656,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01972.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1972,"record_offset":1504,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01972.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1972,"record_offset":572,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01972.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1972,"record_offset":460,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/01972.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::profiler::impl$2::raw
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/profiler.rs
//     evidence={"module":1972,"record_offset":516,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":38940,"source_line":79,"archive":"_pdb/B/modules/01972.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1972,"record_offset":1272,"parent_record_offset":1248,"ancestor_record_offset":256,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01972.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1972,"record_offset":1248,"parent_record_offset":1036,"ancestor_record_offset":256,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01972.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1972,"record_offset":1164,"parent_record_offset":1036,"ancestor_record_offset":256,"inlinee":7420,"source_line":299,"archive":"_pdb/B/modules/01972.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::realloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1972,"record_offset":1188,"parent_record_offset":1164,"ancestor_record_offset":256,"inlinee":7421,"source_line":135,"archive":"_pdb/B/modules/01972.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1972,"record_offset":888,"parent_record_offset":864,"ancestor_record_offset":256,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/01972.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1972,"record_offset":948,"parent_record_offset":828,"ancestor_record_offset":256,"inlinee":7416,"source_line":393,"archive":"_pdb/B/modules/01972.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1972,"record_offset":912,"parent_record_offset":888,"ancestor_record_offset":256,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/01972.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1972,"record_offset":1080,"parent_record_offset":1036,"ancestor_record_offset":256,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01972.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1972,"record_offset":1396,"parent_record_offset":1372,"ancestor_record_offset":256,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01972.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1972,"record_offset":1008,"parent_record_offset":976,"ancestor_record_offset":256,"inlinee":7418,"source_line":692,"archive":"_pdb/B/modules/01972.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1972,"record_offset":976,"parent_record_offset":948,"ancestor_record_offset":256,"inlinee":7417,"source_line":615,"archive":"_pdb/B/modules/01972.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::shrink_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1972,"record_offset":1036,"parent_record_offset":1008,"ancestor_record_offset":256,"inlinee":7419,"source_line":709,"archive":"_pdb/B/modules/01972.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1972,"record_offset":1372,"parent_record_offset":1348,"ancestor_record_offset":256,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01972.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1972,"record_offset":864,"parent_record_offset":828,"ancestor_record_offset":256,"inlinee":7415,"source_line":1246,"archive":"_pdb/B/modules/01972.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::into_boxed_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1972,"record_offset":796,"parent_record_offset":768,"ancestor_record_offset":256,"inlinee":7413,"source_line":1471,"archive":"_pdb/B/modules/01972.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1972,"record_offset":828,"parent_record_offset":796,"ancestor_record_offset":256,"inlinee":7414,"source_line":1407,"archive":"_pdb/B/modules/01972.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1972,"record_offset":680,"parent_record_offset":656,"ancestor_record_offset":256,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01972.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$39::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1972,"record_offset":768,"parent_record_offset":740,"ancestor_record_offset":256,"inlinee":7412,"source_line":4002,"archive":"_pdb/B/modules/01972.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1972,"record_offset":704,"parent_record_offset":680,"ancestor_record_offset":256,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01972.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1972,"record_offset":1104,"parent_record_offset":1080,"ancestor_record_offset":256,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01972.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1972,"record_offset":1324,"parent_record_offset":796,"ancestor_record_offset":256,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01972.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1972,"record_offset":620,"parent_record_offset":596,"ancestor_record_offset":256,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01972.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1972,"record_offset":1220,"parent_record_offset":1036,"ancestor_record_offset":256,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/01972.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1972,"record_offset":596,"parent_record_offset":572,"ancestor_record_offset":256,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01972.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1972,"record_offset":484,"parent_record_offset":460,"ancestor_record_offset":256,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01972.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: gpu_profiler::backend::ash::impl$3::end_frame::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 202
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x152C6
// 首条位置: {"module":1935,"inlinee":86726}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU64::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1936,"record_offset":62000,"parent_record_offset":61976,"ancestor_record_offset":61976,"inlinee":20263,"source_line":2698,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1936,"record_offset":62024,"parent_record_offset":62000,"ancestor_record_offset":61976,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01936.jsonl.gz"}
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
// 完整结构及原始类型记录: ash.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ash.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ash.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 15; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::PrevWallHoles>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::PrevWallHoles>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::PrevWallHoles>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>", "bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>", "bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>", "bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2DFollowMouse> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2DFollowMouse> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2DFollowMouse> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >,ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >)
// 候选参数类型: ["ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >", "ref$<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem> >"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: gpu_profiler::backend::ash::ActiveProfiler::create_marker
// 信息 ID: c6301f37c2d7a73849a0f6c0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2c20da -> 0xc110f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin_frame @ 0xbeb170 -> 0xc110f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: gpu_profiler::backend::ash::ActiveProfiler::end_frame<rhapsody::raw::profiler::ProfilerBuffer>
// 信息 ID: 069e5fb7a9dabde8d63545d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] gpu_profiler::backend::ash::ActiveProfiler::end_frame -> ["rhapsody::raw::profiler::ProfilerBuffer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::end_frame @ 0xbec561 -> 0x18c3910 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: gpu_profiler::backend::ash::ActiveProfiler::write_marker
// 信息 ID: b62c705a982bdb85129a7daa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSliceMut::par_chunk_map_mut::closure$0::async_block_env$0<alloc::vec::Vec<ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,alloc::alloc::Global>,ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,rhapsody::rg::compiled::impl$1::record_batch::closure_env$2<country_core::render::pass_builder::pass_resources::PassResources>,tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> > > > > @ 0xf87f2 -> 0xc11150 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$1::record_batch::closure$1<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2c355e -> 0xc11150 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin_frame @ 0xbeb189 -> 0xc11150 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: gpu_profiler::backend::ash::Profiler::begin_frame
// 信息 ID: 5c50dba5bda0dfd70c4bca75
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin_frame @ 0xbeb157 -> 0xc11090 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: gpu_profiler::backend::ash::Profiler::new
// 信息 ID: e7f991fcb87ed54e32768294
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create @ 0xbf606f -> 0xc10f20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: gpu_profiler::backend::ash::Profiler::write_marker
// 信息 ID: 6b3188af3b2676f02d945d2c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] gpu_profiler::backend::ash::ActiveProfiler::write_marker; module=722, inline_record=2008; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: gpu_profiler::backend::ash::ProfilerOutputBuffer::new
// 信息 ID: 658949a6b0bf0deb609232f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_gpu_profiler_output_buffer; module=703, inline_record=147068; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: gpu_profiler::backend::ash::ProfilerSubmittedFrame::retrieve_results<rhapsody::raw::profiler::ProfilerBuffer>
// 信息 ID: 2e2af4724005c2db5f5676ec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] gpu_profiler::backend::ash::ProfilerSubmittedFrame::retrieve_results -> ["rhapsody::raw::profiler::ProfilerBuffer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render @ 0x4e986d -> 0x294cf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: gpu_profiler::backend::ash::impl$1::retrieve_results::closure$0
// 信息 ID: e246b1cfddf2a5c82fad8fff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "gpu_profiler::backend::ash::impl$1::retrieve_results", "captures": "unknown"}
// [闭包环境类型引用] gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0 <- core::slice::sort::unstable::quicksort::quicksort<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0 <- core::slice::sort::unstable::heapsort::heapsort<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0 <- core::slice::sort::shared::pivot::median3_rec<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0 <- core::slice::sort::shared::pivot::choose_pivot<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0 <- core::slice::sort::shared::smallsort::sort4_stable<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0 <- core::slice::sort::shared::smallsort::small_sort_general<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0 <- core::slice::sort::shared::smallsort::bidirectional_merge<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0 <- core::slice::sort::shared::smallsort::insertion_sort_shift_left<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 63 条；闭包环境类型引用 9 条。
// [PDB 内联宿主] core::slice::sort::unstable::quicksort::quicksort<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >; module=72, inline_record=65488; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::unstable::quicksort::quicksort<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >; module=72, inline_record=65516; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::unstable::quicksort::quicksort<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >; module=72, inline_record=66100; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::unstable::quicksort::quicksort<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >; module=72, inline_record=66128; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::unstable::quicksort::quicksort<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >; module=72, inline_record=66340; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::unstable::quicksort::quicksort<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >; module=72, inline_record=66368; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::unstable::quicksort::quicksort<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >; module=72, inline_record=66644; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::unstable::quicksort::quicksort<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >; module=72, inline_record=66672; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: gpu_profiler::backend::ash::impl$3::end_frame::closure$0
// 信息 ID: e6a323540fd494ac5b4a407d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "gpu_profiler::backend::ash::impl$3::end_frame", "captures": "unknown"}
// [闭包环境类型引用] gpu_profiler::backend::ash::impl$3::end_frame::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<gpu_profiler::shared::MarkerId,core::iter::adapters::map::Map<core::iter::adapters::take::Take<core::slice::iter::Iter<core::sync::atomic::AtomicU64> >,gpu_profiler::backend::ash::impl$3::end_frame::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<gpu_profiler::shared::MarkerId,core::iter::adapters::map::Map<core::iter::adapters::take::Take<core::slice::iter::Iter<core::sync::atomic::AtomicU64> >,gpu_profiler::backend::ash::impl$3::end_frame::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >; module=1936, inline_record=61976; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
