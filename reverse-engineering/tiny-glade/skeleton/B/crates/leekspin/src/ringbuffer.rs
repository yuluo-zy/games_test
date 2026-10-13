// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/leekspin/src/ringbuffer.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: cb9e4562c3ea75c020195a918c51576b688466086660f693ca90ef8760eb9c3d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ringbuffer.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1963
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: leekspin::ringbuffer::RingBuffer::append
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1963,"record_offset":672,"rva":25957248,"va":5394666368}
// 编译实例 RVAs: 0x18c1380
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ringbuffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17180712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394666368,"raw_hex":"e828280601","size":5,"soft_fail":false,"target_va":5411847168,"va":5394666451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16875534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394666368,"raw_hex":"e80e800101","size":5,"soft_fail":false,"target_va":5411542064,"va":5394666525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_by_zero
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16877042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394666368,"raw_hex":"e8f2850101","size":5,"soft_fail":false,"target_va":5411543584,"va":5394666537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ringbuffer.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t17180662","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5394666368,"raw_hex":"e9f6270601","size":5,"soft_fail":false,"target_va":5411847168,"va":5394666501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1963,"record_offset":888,"parent_record_offset":672,"ancestor_record_offset":672,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1963,"record_offset":916,"parent_record_offset":672,"ancestor_record_offset":672,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1963,"record_offset":860,"parent_record_offset":672,"ancestor_record_offset":672,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1963,"record_offset":808,"parent_record_offset":672,"ancestor_record_offset":672,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1963,"record_offset":828,"parent_record_offset":808,"ancestor_record_offset":672,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: leekspin::ringbuffer::RingBuffer::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1963,"record_offset":256,"rva":25957136,"va":5394666256}
// 编译实例 RVAs: 0x18c1310
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ringbuffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19584765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394666256,"raw_hex":"e80329d5fe","size":5,"soft_fail":false,"target_va":5375081536,"va":5394666296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16874604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394666256,"raw_hex":"e86c7c0101","size":5,"soft_fail":false,"target_va":5411540963,"va":5394666354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1963,"record_offset":380,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4759,"source_line":3186,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1963,"record_offset":588,"parent_record_offset":564,"ancestor_record_offset":256,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1963,"record_offset":612,"parent_record_offset":588,"ancestor_record_offset":256,"inlinee":4772,"source_line":173,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1963,"record_offset":564,"parent_record_offset":480,"ancestor_record_offset":256,"inlinee":4771,"source_line":259,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1963,"record_offset":428,"parent_record_offset":404,"ancestor_record_offset":256,"inlinee":4769,"source_line":206,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1963,"record_offset":480,"parent_record_offset":452,"ancestor_record_offset":256,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1963,"record_offset":452,"parent_record_offset":428,"ancestor_record_offset":256,"inlinee":4770,"source_line":442,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1963,"record_offset":516,"parent_record_offset":480,"ancestor_record_offset":256,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$3::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1963,"record_offset":404,"parent_record_offset":380,"ancestor_record_offset":256,"inlinee":5369,"source_line":53,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1963,"record_offset":536,"parent_record_offset":516,"ancestor_record_offset":256,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: leekspin::ringbuffer::RingBuffer::to_inner
// 记录形式: compiled_procedure; instances=1
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1963,"record_offset":1060,"rva":25957424,"va":5394666544}
// 编译实例 RVAs: 0x18c1430
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ringbuffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19585125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394666544,"raw_hex":"e89b27d5fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5394666608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: ringbuffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17180529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394666544,"raw_hex":"e871270601","size":5,"soft_fail":false,"target_va":5411847168,"va":5394666634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: ringbuffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17180385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394666544,"raw_hex":"e8e1260601","size":5,"soft_fail":false,"target_va":5411847184,"va":5394666794,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16873474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394666544,"raw_hex":"e802780101","size":5,"soft_fail":false,"target_va":5411540963,"va":5394667484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_by_zero
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16876083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394666544,"raw_hex":"e833820101","size":5,"soft_fail":false,"target_va":5411543584,"va":5394667496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1963,"record_offset":1196,"parent_record_offset":1060,"ancestor_record_offset":1060,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::rotate_left
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1963,"record_offset":1880,"parent_record_offset":1060,"ancestor_record_offset":1060,"inlinee":13206,"source_line":3499,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1963,"record_offset":1816,"parent_record_offset":1060,"ancestor_record_offset":1060,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1963,"record_offset":1664,"parent_record_offset":1640,"ancestor_record_offset":1060,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1963,"record_offset":1688,"parent_record_offset":1664,"ancestor_record_offset":1060,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1963,"record_offset":1640,"parent_record_offset":1464,"ancestor_record_offset":1060,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1963,"record_offset":1396,"parent_record_offset":1364,"ancestor_record_offset":1060,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1963,"record_offset":1464,"parent_record_offset":1428,"ancestor_record_offset":1060,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1963,"record_offset":1428,"parent_record_offset":1396,"ancestor_record_offset":1060,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1963,"record_offset":1500,"parent_record_offset":1464,"ancestor_record_offset":1060,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1963,"record_offset":1324,"parent_record_offset":1292,"ancestor_record_offset":1060,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1963,"record_offset":1292,"parent_record_offset":1260,"ancestor_record_offset":1060,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1963,"record_offset":1228,"parent_record_offset":1196,"ancestor_record_offset":1060,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1963,"record_offset":1260,"parent_record_offset":1228,"ancestor_record_offset":1060,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1963,"record_offset":1364,"parent_record_offset":1324,"ancestor_record_offset":1060,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1963,"record_offset":1524,"parent_record_offset":1500,"ancestor_record_offset":1060,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1963,"record_offset":1548,"parent_record_offset":1524,"ancestor_record_offset":1060,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1963,"record_offset":1972,"parent_record_offset":1948,"ancestor_record_offset":1060,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1963,"record_offset":1948,"parent_record_offset":1904,"ancestor_record_offset":1060,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1963,"record_offset":2120,"parent_record_offset":2004,"ancestor_record_offset":1060,"inlinee":5303,"source_line":4520,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1963,"record_offset":1764,"parent_record_offset":1740,"ancestor_record_offset":1060,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1963,"record_offset":1596,"parent_record_offset":1572,"ancestor_record_offset":1060,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1963,"record_offset":1572,"parent_record_offset":1548,"ancestor_record_offset":1060,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1963,"record_offset":1740,"parent_record_offset":1324,"ancestor_record_offset":1060,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1963,"record_offset":2060,"parent_record_offset":2004,"ancestor_record_offset":1060,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1963,"record_offset":2500,"parent_record_offset":2428,"ancestor_record_offset":1060,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1963,"record_offset":2372,"parent_record_offset":2292,"ancestor_record_offset":1060,"inlinee":6105,"source_line":1071,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping::runtime
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1963,"record_offset":2400,"parent_record_offset":2372,"ancestor_record_offset":1060,"inlinee":6107,"source_line":3874,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_simple_untyped
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1963,"record_offset":2428,"parent_record_offset":2400,"ancestor_record_offset":1060,"inlinee":6108,"source_line":1139,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1963,"record_offset":2572,"parent_record_offset":2428,"ancestor_record_offset":1060,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::rotate::ptr_rotate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\rotate.rs
//     evidence={"module":1963,"record_offset":1904,"parent_record_offset":1880,"ancestor_record_offset":1060,"inlinee":13204,"source_line":14,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::rotate::ptr_rotate_memmove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\rotate.rs
//     evidence={"module":1963,"record_offset":2004,"parent_record_offset":1904,"ancestor_record_offset":1060,"inlinee":86871,"source_line":48,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::rotate::ptr_rotate_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\rotate.rs
//     evidence={"module":1963,"record_offset":2292,"parent_record_offset":1904,"ancestor_record_offset":1060,"inlinee":86872,"source_line":225,"archive":"_pdb/B/modules/01963.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1963,"record_offset":1844,"parent_record_offset":1816,"ancestor_record_offset":1060,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01963.jsonl.gz"}
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
// 完整结构及原始类型记录: ringbuffer.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] leekspin::ringbuffer::RingBuffer
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 3
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ringbuffer.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ringbuffer.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: leekspin::ringbuffer::RingBuffer::append
// 信息 ID: 470ba63eca42ddb8469bd5b1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 63 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::leek::update_persistent_descriptor_set @ 0xbe2a5b -> 0x18c1380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::leek::create_compute_pipeline @ 0xbe2a8b -> 0x18c1380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::leek::create_raster_pipeline @ 0xbe2abb -> 0x18c1380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::leek::begin_rg_pass @ 0xbe2af7 -> 0x18c1380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::leek::end_rg_pass @ 0xbe2b2b -> 0x18c1380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::shader::DescriptorSetState::allocate_descriptor_set @ 0xbe5d82 -> 0x18c1380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::shader::DescriptorSetState::allocate_descriptor_set @ 0xbe5e0e -> 0x18c1380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::shader::DescriptorSetState::allocate_descriptor_set @ 0xbe5e85 -> 0x18c1380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::wait_gpu_queue_trim @ 0xbea90b -> 0x18c1380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin_frame @ 0xbeae9b -> 0x18c1380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::begin_frame @ 0xbeb1e1 -> 0x18c1380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::submit_pending_cbs @ 0xbebf45 -> 0x18c1380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::command_buffer::CommandBuffer::new @ 0xbed07c -> 0x18c1380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::command_buffer::CommandBuffer::begin @ 0xbed25f -> 0x18c1380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::command_buffer::CommandBuffer::end @ 0xbed3fc -> 0x18c1380 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::issue_full_pipeline_barrier @ 0xbee0b7 -> 0x18c1380 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: leekspin::ringbuffer::RingBuffer::new
// 信息 ID: 0e2b9a39bbd168e3c288e5ae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::init_logging @ 0xbe6aab -> 0x18c1310 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: leekspin::ringbuffer::RingBuffer::to_inner
// 信息 ID: 7e9985974966b656774781e6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::dump_log @ 0xbe6b69 -> 0x18c1430 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
