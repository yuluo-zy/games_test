// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/raw/allocator.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 4d429eb6e38569ea5780c4ca1e7473b94e7acc49cfc77038fbca617b7c4875ac
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: allocator.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 703, 709, 1935, 1936, 1937, 3300, 4223, 4240, 4241
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use gpu_allocator; // namespace candidate
//   package identities: [{"name":"gpu-allocator","source":"local_or_path","version":"0.26.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::allocator::GpuAllocator::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":709,"record_offset":3048,"rva":12638592,"va":5381347712}
// 编译实例 RVAs: 0xc0d980
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: allocator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30499345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381347712,"raw_hex":"e81162d101","size":5,"soft_fail":false,"target_va":5411847168,"va":5381347818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] gpu_allocator::vulkan::Allocator::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"gpu-allocator","version":"0.26.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t18228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381347712,"raw_hex":"e834470000","size":5,"soft_fail":false,"target_va":5381365968,"va":5381347735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":709,"record_offset":3176,"parent_record_offset":3048,"ancestor_record_offset":3048,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::allocator::GpuAllocator::allocate
// 记录形式: compiled_procedure; instances=1
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":709,"record_offset":3216,"rva":12638784,"va":5381347904}
// 编译实例 RVAs: 0xc0da40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1296074","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381347904,"raw_hex":"e8cac61300","size":5,"soft_fail":false,"target_va":5382644528,"va":5381348449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: allocator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6266659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381347904,"raw_hex":"e8dd60a0ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5381348142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: allocator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6267409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381347904,"raw_hex":"e8ef5da0ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381348908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=2
//     target_aliases=69 candidates; complete list: allocator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5832247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381347904,"raw_hex":"e8c901a7ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5381348274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdfe90
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: allocator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-187871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381347904,"raw_hex":"e82122fdff","size":5,"soft_fail":false,"target_va":5381160592,"va":5381348458,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418ba4e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: allocator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13288856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381347904,"raw_hex":"e898c5ca00","size":5,"soft_fail":false,"target_va":5394638048,"va":5381349187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30191739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381347904,"raw_hex":"e87bb0cc01","size":5,"soft_fail":false,"target_va":5411540963,"va":5381349219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<gpu_allocator::vulkan::Allocation,enum2$<gpu_allocator::result::AllocationError> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381347904,"raw_hex":"e8cbf4ffff","size":5,"soft_fail":false,"target_va":5381346432,"va":5381349296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13292579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381347904,"raw_hex":"e823d4ca00","size":5,"soft_fail":false,"target_va":5394641824,"va":5381349240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] gpu_allocator::vulkan::Allocator::allocate
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"gpu-allocator","version":"0.26.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t21320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381347904,"raw_hex":"e848530000","size":5,"soft_fail":false,"target_va":5381369280,"va":5381347955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] gpu_allocator::vulkan::Allocator::generate_report
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"gpu-allocator","version":"0.26.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t30399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381347904,"raw_hex":"e8bf760000","size":5,"soft_fail":false,"target_va":5381378384,"va":5381347980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":709,"record_offset":4828,"parent_record_offset":3216,"ancestor_record_offset":3216,"inlinee":7662,"source_line":298,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":709,"record_offset":3376,"parent_record_offset":3216,"ancestor_record_offset":3216,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":709,"record_offset":4620,"parent_record_offset":3216,"ancestor_record_offset":3216,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":709,"record_offset":3460,"parent_record_offset":3216,"ancestor_record_offset":3216,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":709,"record_offset":5304,"parent_record_offset":3216,"ancestor_record_offset":3216,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":709,"record_offset":3404,"parent_record_offset":3216,"ancestor_record_offset":3216,"inlinee":8084,"source_line":607,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":709,"record_offset":4652,"parent_record_offset":3216,"ancestor_record_offset":3216,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":709,"record_offset":4536,"parent_record_offset":3216,"ancestor_record_offset":3216,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":709,"record_offset":3960,"parent_record_offset":3936,"ancestor_record_offset":3216,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":709,"record_offset":3984,"parent_record_offset":3960,"ancestor_record_offset":3216,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":709,"record_offset":5456,"parent_record_offset":5432,"ancestor_record_offset":3216,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":709,"record_offset":3936,"parent_record_offset":3756,"ancestor_record_offset":3216,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":709,"record_offset":5432,"parent_record_offset":5376,"ancestor_record_offset":3216,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":709,"record_offset":3688,"parent_record_offset":3656,"ancestor_record_offset":3216,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":709,"record_offset":5404,"parent_record_offset":5376,"ancestor_record_offset":3216,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":709,"record_offset":5376,"parent_record_offset":5352,"ancestor_record_offset":3216,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":709,"record_offset":3756,"parent_record_offset":3720,"ancestor_record_offset":3216,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":709,"record_offset":3720,"parent_record_offset":3688,"ancestor_record_offset":3216,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":709,"record_offset":5352,"parent_record_offset":5328,"ancestor_record_offset":3216,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":709,"record_offset":3796,"parent_record_offset":3756,"ancestor_record_offset":3216,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":709,"record_offset":4864,"parent_record_offset":4828,"ancestor_record_offset":3216,"inlinee":6036,"source_line":907,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":709,"record_offset":4068,"parent_record_offset":4040,"ancestor_record_offset":3216,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":709,"record_offset":3624,"parent_record_offset":3580,"ancestor_record_offset":3216,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":709,"record_offset":3656,"parent_record_offset":3624,"ancestor_record_offset":3216,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":709,"record_offset":3500,"parent_record_offset":3460,"ancestor_record_offset":3216,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":709,"record_offset":4288,"parent_record_offset":4256,"ancestor_record_offset":3216,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":709,"record_offset":4376,"parent_record_offset":4288,"ancestor_record_offset":3216,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":709,"record_offset":4040,"parent_record_offset":3580,"ancestor_record_offset":3216,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":709,"record_offset":3540,"parent_record_offset":3500,"ancestor_record_offset":3216,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":709,"record_offset":3580,"parent_record_offset":3540,"ancestor_record_offset":3216,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":709,"record_offset":3820,"parent_record_offset":3796,"ancestor_record_offset":3216,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":709,"record_offset":3844,"parent_record_offset":3820,"ancestor_record_offset":3216,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":709,"record_offset":4944,"parent_record_offset":4900,"ancestor_record_offset":3216,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":709,"record_offset":4124,"parent_record_offset":4096,"ancestor_record_offset":3216,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":709,"record_offset":4192,"parent_record_offset":4152,"ancestor_record_offset":3216,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":709,"record_offset":4096,"parent_record_offset":4068,"ancestor_record_offset":3216,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":709,"record_offset":4256,"parent_record_offset":4192,"ancestor_record_offset":3216,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":709,"record_offset":3868,"parent_record_offset":3844,"ancestor_record_offset":3216,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":709,"record_offset":3892,"parent_record_offset":3868,"ancestor_record_offset":3216,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":709,"record_offset":4708,"parent_record_offset":4676,"ancestor_record_offset":3216,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":709,"record_offset":4736,"parent_record_offset":4676,"ancestor_record_offset":3216,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":709,"record_offset":5328,"parent_record_offset":5304,"ancestor_record_offset":3216,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":709,"record_offset":4416,"parent_record_offset":4152,"ancestor_record_offset":3216,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":709,"record_offset":4328,"parent_record_offset":4288,"ancestor_record_offset":3216,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":709,"record_offset":3428,"parent_record_offset":3404,"ancestor_record_offset":3216,"inlinee":6884,"source_line":564,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":709,"record_offset":4152,"parent_record_offset":4124,"ancestor_record_offset":3216,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":709,"record_offset":4900,"parent_record_offset":4864,"ancestor_record_offset":3216,"inlinee":6038,"source_line":29,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":709,"record_offset":4560,"parent_record_offset":4536,"ancestor_record_offset":3216,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":709,"record_offset":4584,"parent_record_offset":4560,"ancestor_record_offset":3216,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":709,"record_offset":4764,"parent_record_offset":4676,"ancestor_record_offset":3216,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":709,"record_offset":4676,"parent_record_offset":4652,"ancestor_record_offset":3216,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":709,"record_offset":4788,"parent_record_offset":4764,"ancestor_record_offset":3216,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::allocator::impl$0::allocate::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/allocator.rs
//     evidence={"module":709,"record_offset":4220,"parent_record_offset":4192,"ancestor_record_offset":3216,"inlinee":40038,"source_line":48,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::allocator::impl$0::allocate::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9C66
// 首条位置: {"module":709,"inlinee":40038}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::allocator::impl$0::allocate::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=35
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x152F9
// 首条位置: {"module":1937,"inlinee":86777}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$67::cmp
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1937,"record_offset":14328,"parent_record_offset":14308,"ancestor_record_offset":14308,"inlinee":21350,"source_line":1836,"archive":"_pdb/B/modules/01937.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::allocator::GpuAllocator::free
// 记录形式: compiled_procedure; instances=1
// 源码行: 82
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":709,"record_offset":5736,"rva":12640592,"va":5381349712}
// 编译实例 RVAs: 0xc0e150
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdf610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: allocator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-192492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381349712,"raw_hex":"e81410fdff","size":5,"soft_fail":false,"target_va":5381158416,"va":5381350903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30191020","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381349712,"raw_hex":"e8acadcc01","size":5,"soft_fail":false,"target_va":5411541584,"va":5381350559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-3152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381349712,"raw_hex":"e8b0f3ffff","size":5,"soft_fail":false,"target_va":5381346640,"va":5381349787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t30193487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381349712,"raw_hex":"e84fb7cc01","size":5,"soft_fail":false,"target_va":5411543360,"va":5381349868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] gpu_allocator::vulkan::Allocator::free
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"gpu-allocator","version":"0.26.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t25183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381349712,"raw_hex":"e85f620000","size":5,"soft_fail":false,"target_va":5381375616,"va":5381350428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t35184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381349712,"raw_hex":"e870890000","size":5,"soft_fail":false,"target_va":5381385536,"va":5381350347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t40399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381349712,"raw_hex":"e8cf9d0000","size":5,"soft_fail":false,"target_va":5381390640,"va":5381350236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t35962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381349712,"raw_hex":"e87a8c0000","size":5,"soft_fail":false,"target_va":5381386480,"va":5381350513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t34938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381349712,"raw_hex":"e87a880000","size":5,"soft_fail":false,"target_va":5381384704,"va":5381349761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30387530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381349712,"raw_hex":"e84aadcf01","size":5,"soft_fail":false,"target_va":5411737808,"va":5381350273,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13322398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381349712,"raw_hex":"e89e48cb00","size":5,"soft_fail":false,"target_va":5394673152,"va":5381350749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30186504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381349712,"raw_hex":"e8089ccc01","size":5,"soft_fail":false,"target_va":5411537104,"va":5381350595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":709,"record_offset":8292,"parent_record_offset":5736,"ancestor_record_offset":5736,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":709,"record_offset":7300,"parent_record_offset":5736,"ancestor_record_offset":5736,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":709,"record_offset":5876,"parent_record_offset":5736,"ancestor_record_offset":5736,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":709,"record_offset":6324,"parent_record_offset":5736,"ancestor_record_offset":5736,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":709,"record_offset":7672,"parent_record_offset":7644,"ancestor_record_offset":5736,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":709,"record_offset":7724,"parent_record_offset":7700,"ancestor_record_offset":5736,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":709,"record_offset":7700,"parent_record_offset":7672,"ancestor_record_offset":5736,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":709,"record_offset":7612,"parent_record_offset":7580,"ancestor_record_offset":5736,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":709,"record_offset":7644,"parent_record_offset":7612,"ancestor_record_offset":5736,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":709,"record_offset":7816,"parent_record_offset":7644,"ancestor_record_offset":5736,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":709,"record_offset":7928,"parent_record_offset":7904,"ancestor_record_offset":5736,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":709,"record_offset":7748,"parent_record_offset":7724,"ancestor_record_offset":5736,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":709,"record_offset":6024,"parent_record_offset":5992,"ancestor_record_offset":5736,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":709,"record_offset":7448,"parent_record_offset":7412,"ancestor_record_offset":5736,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":709,"record_offset":6272,"parent_record_offset":6244,"ancestor_record_offset":5736,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":709,"record_offset":7548,"parent_record_offset":7412,"ancestor_record_offset":5736,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":709,"record_offset":7880,"parent_record_offset":7580,"ancestor_record_offset":5736,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":709,"record_offset":6736,"parent_record_offset":6712,"ancestor_record_offset":5736,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":709,"record_offset":6712,"parent_record_offset":6688,"ancestor_record_offset":5736,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":709,"record_offset":6920,"parent_record_offset":6892,"ancestor_record_offset":5736,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":709,"record_offset":7772,"parent_record_offset":7748,"ancestor_record_offset":5736,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":709,"record_offset":6664,"parent_record_offset":6640,"ancestor_record_offset":5736,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":709,"record_offset":6688,"parent_record_offset":6664,"ancestor_record_offset":5736,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":709,"record_offset":6992,"parent_record_offset":6964,"ancestor_record_offset":5736,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":709,"record_offset":6640,"parent_record_offset":6524,"ancestor_record_offset":5736,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":709,"record_offset":5992,"parent_record_offset":5964,"ancestor_record_offset":5736,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":709,"record_offset":6352,"parent_record_offset":6324,"ancestor_record_offset":5736,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":709,"record_offset":6492,"parent_record_offset":6460,"ancestor_record_offset":5736,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":709,"record_offset":6460,"parent_record_offset":6424,"ancestor_record_offset":5736,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":709,"record_offset":6400,"parent_record_offset":6376,"ancestor_record_offset":5736,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":709,"record_offset":6424,"parent_record_offset":6400,"ancestor_record_offset":5736,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":709,"record_offset":6376,"parent_record_offset":6352,"ancestor_record_offset":5736,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":709,"record_offset":6964,"parent_record_offset":6424,"ancestor_record_offset":5736,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":709,"record_offset":5964,"parent_record_offset":5932,"ancestor_record_offset":5736,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":709,"record_offset":5932,"parent_record_offset":5908,"ancestor_record_offset":5736,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":709,"record_offset":6124,"parent_record_offset":6092,"ancestor_record_offset":5736,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":709,"record_offset":6220,"parent_record_offset":6124,"ancestor_record_offset":5736,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":709,"record_offset":6092,"parent_record_offset":6068,"ancestor_record_offset":5736,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":709,"record_offset":5908,"parent_record_offset":5876,"ancestor_record_offset":5736,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":709,"record_offset":6244,"parent_record_offset":6220,"ancestor_record_offset":5736,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":709,"record_offset":6524,"parent_record_offset":6492,"ancestor_record_offset":5736,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":709,"record_offset":6808,"parent_record_offset":6524,"ancestor_record_offset":5736,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":709,"record_offset":6780,"parent_record_offset":6524,"ancestor_record_offset":5736,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":709,"record_offset":6584,"parent_record_offset":6524,"ancestor_record_offset":5736,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":709,"record_offset":7336,"parent_record_offset":7300,"ancestor_record_offset":5736,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":709,"record_offset":8364,"parent_record_offset":8328,"ancestor_record_offset":5736,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":709,"record_offset":8916,"parent_record_offset":8644,"ancestor_record_offset":5736,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":709,"record_offset":7476,"parent_record_offset":7448,"ancestor_record_offset":5736,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":709,"record_offset":7580,"parent_record_offset":7412,"ancestor_record_offset":5736,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":709,"record_offset":7852,"parent_record_offset":7580,"ancestor_record_offset":5736,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":709,"record_offset":7504,"parent_record_offset":7476,"ancestor_record_offset":5736,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":709,"record_offset":7412,"parent_record_offset":7372,"ancestor_record_offset":5736,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":709,"record_offset":7372,"parent_record_offset":7336,"ancestor_record_offset":5736,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::allocator::GpuAllocator::rename_allocation
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9AB4
// 首条位置: {"module":703,"inlinee":39604}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::allocator::GpuAllocator::inner
// 记录形式: compiled_procedure; instances=1
// 源码行: 99
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":709,"record_offset":9480,"rva":6806912,"va":5375516032}
// 编译实例 RVAs: 0x67dd80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::allocator::GpuAllocator::peak_total_allocated_bytes
// 记录形式: compiled_procedure; instances=1
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":709,"record_offset":9604,"rva":12641792,"va":5381350912}
// 编译实例 RVAs: 0xc0e600
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::allocator::GpuAllocator::peak_allocator_report
// 记录形式: compiled_procedure; instances=1
// 源码行: 107
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":709,"record_offset":9748,"rva":12641808,"va":5381350928}
// 编译实例 RVAs: 0xc0e610
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":709,"record_offset":9896,"parent_record_offset":9748,"ancestor_record_offset":9748,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00709.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::allocator::GpuAllocator::peak_device_local_bytes
// 记录形式: compiled_procedure; instances=1
// 源码行: 112
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":709,"record_offset":9924,"rva":12641840,"va":5381350960}
// 编译实例 RVAs: 0xc0e630
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::allocator::GpuAllocator::peak_system_bytes
// 记录形式: compiled_procedure; instances=1
// 源码行: 116
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":709,"record_offset":10064,"rva":12641856,"va":5381350976}
// 编译实例 RVAs: 0xc0e640
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: allocator.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::allocator::GpuAllocator
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 11; 过程引用数: 16
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,rhapsody::raw::allocator::GpuAllocator>", "constructor": "lock_api::mutex::MutexGuard", "arguments": ["parking_lot::raw_mutex::RawMutex", "rhapsody::raw::allocator::GpuAllocator"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,rhapsody::raw::allocator::GpuAllocator> >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,rhapsody::raw::allocator::GpuAllocator>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::allocator::GpuAllocator>", "constructor": "lock_api::mutex::Mutex", "arguments": ["parking_lot::raw_mutex::RawMutex", "rhapsody::raw::allocator::GpuAllocator"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::allocator::GpuAllocator>,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::allocator::GpuAllocator>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::allocator::GpuAllocator> >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::allocator::GpuAllocator>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::new<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::allocator::GpuAllocator> >", "constructor": "alloc::sync::Arc::new", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::allocator::GpuAllocator>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: allocator.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: allocator.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>,ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>)
// 候选参数类型: ["ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>", "ref$<rhapsody::rg::resource_info::ResourceLifetimeEvent>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)
// 候选参数类型: ["country_core::systems::collision::world::SubworldColliderHandle", "ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)
// 候选参数类型: ["rhapsody::raw::physical_device::PhysicalDevice"]

// ------------------------------------------------------------------
// 函数: rhapsody::raw::allocator::GpuAllocator::allocate
// 信息 ID: a0491db38db10f20523de87b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 9 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<ref$<slice2$<u8> > > @ 0x5b9d2e -> 0xc0da40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x5ba820 -> 0xc0da40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer_impl @ 0xbee3fb -> 0xc0da40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::profiler::impl$1::create_query_result_buffer @ 0xbff69b -> 0xc0da40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0xc07b6b -> 0xc0da40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<ref$<slice2$<u8> > > @ 0xfaedce -> 0xc0da40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<ref$<slice2$<u8> > > @ 0x16a692e -> 0xc0da40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x16a7420 -> 0xc0da40 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x17facb0 -> 0xc0da40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::allocator::GpuAllocator::free
// 信息 ID: c7340695c81e2132205b5670
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::immediate_empty_bucket @ 0xbfb015 -> 0xc0e150 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::allocator::GpuAllocator::inner
// 信息 ID: d89420cc68f804d0b40b4626
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 137943db23b17086ce8b9d61: bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)
// [泛型宿主/闭包/直接引用方参数候选] 1e83d0fd90a253224831420b: enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)
// 引用方: 原程序直接转移 1177 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::setup::setup_window @ 0x43b79 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::setup_window @ 0x43cba -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::setup_window @ 0x44288 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::find_best_fullscreen_video_mode @ 0x45e63 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::find_best_fullscreen_video_mode @ 0x45c78 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::find_best_fullscreen_video_mode @ 0x457de -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] winit::platform_impl::platform::window::InitData::on_create<tuple$<> > @ 0x7d1dc -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] winit::platform_impl::platform::window::init<tuple$<> > @ 0x7e616 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game_thread::closure_env$8,tuple$<> > @ 0xd797c -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game_thread::closure_env$8,tuple$<> > @ 0xd7a85 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::screenshot::write_image::closure_env$0,tuple$<> > @ 0xd8010 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::setup_panic_hook::closure$0 @ 0xdbaba -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::setup_panic_hook::closure$0 @ 0xdba0a -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0xdea3b -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0xdea5c -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0xdeef4 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::allocator::GpuAllocator::new
// 信息 ID: ee5d06c7cce6011f0ae03df9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create @ 0xbf4d99 -> 0xc0d980 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::allocator::GpuAllocator::peak_allocator_report
// 信息 ID: 1ed078a36adc0176b88f992a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_global_allocator<country_core::meta_commander::commander_update::closure_env$3,tuple$<u64,u64,u64,enum2$<core::option::Option<gpu_allocator::allocator::AllocatorReport> > > > @ 0x16a3686 -> 0xc0e610 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::allocator::GpuAllocator::peak_device_local_bytes
// 信息 ID: 9491403c76b9dd1f2fc314c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_global_allocator<tiny_glade::game_loop::shut_down_everything::closure_env$0,tuple$<u64,u64> > @ 0x5b9a6e -> 0xc0e630 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_global_allocator<country_core::meta_commander::commander_update::closure_env$3,tuple$<u64,u64,u64,enum2$<core::option::Option<gpu_allocator::allocator::AllocatorReport> > > > @ 0x16a3665 -> 0xc0e630 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::allocator::GpuAllocator::peak_system_bytes
// 信息 ID: 06571216691b8de9e347da47
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_global_allocator<tiny_glade::game_loop::shut_down_everything::closure_env$0,tuple$<u64,u64> > @ 0x5b9a79 -> 0xc0e640 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_global_allocator<country_core::meta_commander::commander_update::closure_env$3,tuple$<u64,u64,u64,enum2$<core::option::Option<gpu_allocator::allocator::AllocatorReport> > > > @ 0x16a3670 -> 0xc0e640 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::allocator::GpuAllocator::peak_total_allocated_bytes
// 信息 ID: 8835935210a99387e7ee1ab1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_global_allocator<country_core::meta_commander::commander_update::closure_env$3,tuple$<u64,u64,u64,enum2$<core::option::Option<gpu_allocator::allocator::AllocatorReport> > > > @ 0x16a367b -> 0xc0e600 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::allocator::GpuAllocator::rename_allocation
// 信息 ID: cf925f786ea8b0c9ba32167e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::set_buffer_debug_name; module=703, inline_record=12584; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::set_texture_debug_name; module=703, inline_record=139192; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::allocator::impl$0::allocate::closure$0
// 信息 ID: 4f889567324949efe02d2f46
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::allocator::impl$0::allocate", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::allocator::GpuAllocator::allocate; module=709, inline_record=4220; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::allocator::impl$0::allocate::closure$1
// 信息 ID: a308132a5d71ffef70e3b26a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::raw::allocator::impl$0::allocate", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::allocator::impl$0::allocate::closure_env$1 <- core::slice::sort::shared::smallsort::insertion_sort_shift_left<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::allocator::impl$0::allocate::closure_env$1 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::allocator::impl$0::allocate::closure_env$1 <- core::slice::sort::stable::driftsort_main<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1>,alloc::vec::Vec<tuple$<ref$<str$>,u64>,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::allocator::impl$0::allocate::closure_env$1 <- core::slice::sort::stable::drift::sort<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::allocator::impl$0::allocate::closure_env$1 <- core::slice::sort::shared::pivot::median3_rec<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::allocator::impl$0::allocate::closure_env$1 <- core::slice::sort::stable::quicksort::quicksort<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 28 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insertion_sort_shift_left<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >; module=1937, inline_record=14308; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insertion_sort_shift_left<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >; module=1937, inline_record=14472; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >; module=1937, inline_record=17384; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >; module=1937, inline_record=17492; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >; module=1937, inline_record=17664; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >; module=1937, inline_record=17788; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >; module=1937, inline_record=17960; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >; module=1937, inline_record=18360; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
